use std::{io::Error, sync::Arc};
use actix_web::{http::StatusCode, web::{Json, Data}, Responder, post, HttpResponse, get};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, digest::KeyInit};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::FromRow;

use crate::{AppState, TokenClaims, extractors::auth_token::LoginUser};

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

#[derive(Serialize, Deserialize)]
pub struct CreateUserBody {
    username: String,
    email: String,
    password: String,
}

#[post("/register")]
async fn register_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();
    println!("Sanity");
    dbg!("check");
    let hash_secret = std::env::var("HASH_SECRET").unwrap_or(env!("HASH_SECRET").to_owned());
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    match sqlx::query_as::<_, UserNoPassword>(
        "INSERT INTO users (user_id, username, email, password)
        VALUES (DEFAULT, $1, $2, $3)
        RETURNING user_id, username"
    )
    .bind(user.username)
    .bind(user.email)
    .bind(hash)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err))
    }
}

#[post("/auth")]
async fn basic_auth(state: Data<AppState>, credentials: LoginUser) -> impl Responder {
    // let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
    //     std::env::var("JWT_SECRET")
    //         .expect("JWT_SECRET must be set")
    //         .as_bytes()
    // ).unwrap();
    println!("Ugh");
    let secret = std::env::var("JWT_SECRET").unwrap_or(env!("JWT_SECRET").to_owned());
    let username = credentials.username;
    let password = credentials.password;

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
                let claims = TokenClaims { id: user.user_id };
                let token: String = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(secret.as_str().as_ref()),
                )
                .unwrap();
                
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().json("Incorrect username or password")
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err)),
    }
}