use actix_web::{web, HttpResponse, Scope, Responder, get};
use chrono::{Duration, Utc, DateTime};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::{Claims, AppState};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("/decode-token", web::post().to(decode_token))
        .service(get_user_handler)
    //.route("/protected", web::get().to(protected))
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

async fn encode_token(path: web::Path<i32>, secret: String) -> HttpResponse {
    let user_id: i32 = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { user_id, exp };
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
    id: i32,
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
            id: token.claims.user_id,
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}


/******************
 **** Services ****
 *****************/

 #[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserProfile {
    user_id: i32,
    account_id: i32,
    email: String,
    username: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseUserProfile {
    user: UserProfile,
}

#[get("/{id}")]
async fn get_user_handler(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(
        UserProfile,
        "SELECT user_id, account_id, email, username, created_at FROM users WHERE user_id = $1",
        user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            // let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
            //     "user": user
            // })});
            // return HttpResponse::Ok().json(user_response);

            let json_response = ResponseUserProfile {
                user: user,
            };

            return HttpResponse::Ok().json(json_response);
        }
        Err(_) => {
            let message = format!("engagement with ID: {} not found", user_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

// async fn protected(auth_token: AuthToken) -> HttpResponse {
//     println!("{}", auth_token.id);
//     HttpResponse::Ok().json(Response {
//         message: "protected".to_owned(),
//     })
// }
