use actix_web::{web, HttpResponse, Scope};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::extractors::auth_token::AuthToken;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("/decode-token", web::post().to(decode_token))
        .route("/protected", web::get().to(protected))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

async fn encode_token(path: web::Path<usize>, secret: String) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();
    HttpResponse::Ok().json(EncodeResponse {
        message: "success".to_owned(),
        token: token,
    })
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: usize,
}

async fn decode_token(body: web::Json<DecodeBody>, secret: String) -> HttpResponse {
    let decoded: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: "Authorized".to_string(),
            id: token.claims.id,
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}

async fn protected(auth_token: AuthToken) -> HttpResponse {
    println!("{}", auth_token.id);
    HttpResponse::Ok().json(Response {
        message: "protected".to_owned(),
    })
}
