use actix_web::{ web, Scope, HttpResponse };
use chrono::{ Utc, Duration };
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
struct Claims {
    id: usize,
    exp: usize,
}

async fn encode_token(path: web::Path<usize>, secret: String) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    ).unwrap();
    HttpResponse::Ok().json(EncodeResponse { 
        message: "success".to_owned(),
        token: token
    })
}

async fn decode_token() -> HttpResponse {
    HttpResponse::Ok().json(Response { message: "decode_token".to_owned() })
}

async fn protected() -> HttpResponse {
    HttpResponse::Ok().json(Response { message: "protected".to_owned() })
}