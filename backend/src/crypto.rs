use std::{io::Error, sync::Arc, ops::Add};
use actix_web::{http::StatusCode, web::{Json, Data}, Responder, post, HttpResponse, get};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use chrono::{Utc, NaiveDateTime, Duration};
use common::ApiLoginResponse;
use hmac::{Hmac, digest::KeyInit};
use jsonwebtoken::{EncodingKey, Header, encode};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::FromRow;
use tracing::{instrument, Instrument};
use uuid::Uuid;
use validator::{Validate, ValidationError};
use lazy_static::lazy_static;

use crate::{AppState, Claims, extractors::jwt_auth::LoginUser, redis_connect, set_str};

pub struct CryptoService {
    pub key: Arc<String>
}
#[derive(FromRow, Serialize, Deserialize)]
pub struct UserNoPassword {
    user_id: i32,
    username: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct AuthUser {
    user_id: i32,
    username: String,
    password: String,
}

lazy_static! {
    static ref RE_USER_NAME: Regex = Regex::new(r"^[a-zA-Z0-9]{6,}$").unwrap();
    static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }
    if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
        Ok(())
    } else {
        return Err(ValidationError::new("Password Validation Failed"));
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserBody {
    #[validate(
        regex(
            path = "RE_USER_NAME",
            message = "Username must contain number & alphabets only & must be 6 characters long"
        )
    )]
    username: String,
    #[validate(length(min = 3, message = "Username must be greater than 3 chars"))]
    email: String,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. No spaces."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        )
    )]
    password: String,
}

#[instrument]
#[post("/register")]
async fn register_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let is_valid = body.validate();
    if is_valid.is_err() {
        return HttpResponse::InternalServerError().json(format!("{:?}", is_valid.err().unwrap()))
    }
    let _ = dbg!(is_valid);
    let user: CreateUserBody = body.into_inner();
    let hash_secret = std::env::var("HASH_SECRET").unwrap_or(env!("HASH_SECRET").to_owned());
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    let query_span = tracing::info_span!(
        "Saving user details in the database"
    );
    match sqlx::query_as::<_, UserNoPassword>(
        "INSERT INTO users (user_id, username, email, password)
        VALUES (DEFAULT, $1, $2, $3)
        RETURNING user_id, username"
    )
    .bind(user.username)
    .bind(user.email)
    .bind(hash)
    .fetch_one(&state.db)
    .instrument(query_span)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err))
    }
}

#[post("/auth")]
async fn basic_auth(state: Data<AppState>, credentials: Json<LoginUser>) -> impl Responder {
    // let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
    //     std::env::var("JWT_SECRET")
    //         .expect("JWT_SECRET must be set")
    //         .as_bytes()
    // ).unwrap();
    println!("Ugh");
    let secret = std::env::var("JWT_SECRET").unwrap_or(env!("JWT_SECRET").to_owned());
    let username = &credentials.username;
    let password = &credentials.password;

    match sqlx::query_as::<_, AuthUser>(
        "SELECT user_id, username, password FROM users WHERE username = $1",
    )
    .bind(username.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => {
            let hash_secret = std::env::var("HASH_SECRET").unwrap_or(env!("HASH_SECRET").to_owned());
            // Build the verifier
            let mut verifier = Verifier::default();
            let is_valid = verifier
                .with_hash(user.password)
                .with_password(password)
                .with_secret_key(hash_secret)
                .verify()
                .unwrap();

            if is_valid {
                let exp: usize = (Utc::now() + Duration::hours(2)).timestamp() as usize;
                let claims = Claims { 
                    user_id: user.user_id,
                    exp: exp,
                };
                let token: String = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(secret.as_str().as_ref()),
                )
                .unwrap();
                let mut con = redis_connect();
                let _ = set_str(&mut con, user.user_id.to_string().as_ref(), &token, 60);
                HttpResponse::Ok().json(ApiLoginResponse {
                    user_id: user.user_id,
                    username: user.username,
                    token: token,
                })
            } else {
                HttpResponse::Unauthorized().json("Incorrect username or password")
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err)),
    }
}