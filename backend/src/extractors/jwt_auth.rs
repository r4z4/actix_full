use crate::{redis_connect, Claims};
use crate::AppState;
use actix_web::HttpMessage;
use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    http::{self, header::HeaderValue},
    web, Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct JwtAuth {
    pub user_id: i32,
}
#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

impl FromRequest for JwtAuth {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        dbg!(req);
        // Get auth token from auth header
        let auth_header: Option<HeaderValue> =
            req.headers().get(http::header::AUTHORIZATION).cloned();
        dbg!(&auth_header);
        let auth_token: String = auth_header.unwrap().to_str().unwrap_or("").to_string();
        // let mut con = redis_connect();
        // let answer: i32 = con.get("answer").unwrap();
        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid auth token")));
        }

        dbg!(&auth_token);

        let app_data: &AppState = req.app_data::<web::Data<AppState>>().unwrap();
        // Decode token w/ secret
        let decode: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(app_data.secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        dbg!(&decode);
        // Return self (auth token)
        match decode {
            Ok(token) => {
                req.extensions_mut()
                    .insert::<i32>(token.claims.user_id.to_owned());
        
                ready(Ok(JwtAuth { user_id: token.claims.user_id }))
            },
            Err(_e) => ready(Err(ErrorUnauthorized("Unauthorized :/"))),
        }
    }
}

// impl FromRequest for LoginUser {
//     type Error = ActixWebError;
//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         dbg!(req);
//         dbg!(username_input.clone());
//         // Get auth token from auth header

//             ready(Ok(LoginUser {
//                 username: username_input,
//                 password: password_input,
//             }))
//    }
// }
