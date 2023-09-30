use actix_web::{get, web, HttpResponse, Responder, HttpRequest, HttpMessage, Scope};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{schema::FilterOptions, AppState, extractors::jwt_auth};

pub fn admin_scope() -> Scope {
    web::scope("/admin")
        // .route("/users", web::get().to(get_users_handler))
        .service(get_users_handler)
        .service(get_consultants_handler)
        .service(get_consultant_handler)
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    id: usize,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: usize,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUserList {
    pub users: Vec<ResponseUser>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUser {
    pub user_id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultantList {
    pub consultants: Vec<ResponseConsultant>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultant {
    pub consultant_id: i32,
    pub specialty_id: i32,
    pub consultant_f_name: String,
    pub consultant_l_name: String,
    pub consultant_slug: String,
}

// #[get("/users")]
// pub async fn get_users_handler(
//     opts: web::Query<FilterOptions>,
//     data: web::Data<AppState>,
// ) -> impl Responder {
//     let limit = opts.limit.unwrap_or(10);
//     let offset = (opts.page.unwrap_or(1) - 1) * limit;

//     let query_result = sqlx::query_as!(
//         ResponseUser,
//         "SELECT user_id, username, email FROM users ORDER by user_id LIMIT $1 OFFSET $2",
//         limit as i32,
//         offset as i32
//     )
//     .fetch_all(&data.db)
//     .await;

//     if query_result.is_err() {
//         let message = "Error occurred while fetching all engagement items";
//         return HttpResponse::InternalServerError()
//             .json(json!({"status": "error","message": message}));
//     }

//     let users = query_result.unwrap();

//     // let json_response = ResponseUserList {
//     //     users: users
//     // };

//     let json_response = serde_json::json!({
//         "status": "success",
//         "results": users.len(),
//         "engagements": users
//     });
//     // HttpResponse::Ok().json(json_response)
//     HttpResponse::NoContent().finish()
// }

// async fn get_user_handler(path: web::Path<usize>) -> HttpResponse {
//     let id: usize = path.into_inner();
//     let content = String::from("Message Content");
//     let message: Message = Message { id, content };
//     HttpResponse::Ok().json(ResponseUser {
//         username: message.content,
//         user_id: 3,
//         email: "aa@aa.com".to_owned(),
//     })
// }

#[get("/users")]
pub async fn get_users_handler(
    req: HttpRequest,
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
    _: jwt_auth::JwtAuth,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<i32>().unwrap();
    dbg!(user_id);
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ResponseUser,
        "SELECT user_id, username, email FROM users ORDER by user_id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Error occurred while fetching all engagement items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let users = query_result.unwrap();

    let json_response = ResponseUserList { users: users };

    // let json_response = serde_json::json!({
    //     "status": "success",
    //     "results": users.len(),
    //     "engagements": users
    // });
    HttpResponse::Ok().json(json_response)
    // HttpResponse::NoContent().finish()
}

#[get("/consultants")]
pub async fn get_consultants_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ResponseConsultant,
        "SELECT consultant_id, consultant_slug, consultant_f_name, consultant_l_name, specialty_id FROM consultants ORDER by consultant_id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Error occurred while fetching all consultant records";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let consultants = query_result.unwrap();

    let json_response = ResponseConsultantList {
        consultants: consultants,
    };

    // let json_response = serde_json::json!({
    //     "status": "success",
    //     "results": users.len(),
    //     "engagements": users
    // });
    HttpResponse::Ok().json(json_response)
    // HttpResponse::NoContent().finish()
}

#[get("/consultants/{id}")]
pub async fn get_consultant_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let consultant_id = path.into_inner();
    let query_result = sqlx::query_as!(
        ResponseConsultant,
        "SELECT consultant_id, consultant_slug, consultant_f_name, consultant_l_name, specialty_id FROM consultants WHERE consultant_id = $1",
        consultant_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Error occurred while fetching all consult records";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let consultant = query_result.unwrap();

    let json_response = ResponseConsultant {
        consultant_id: consultant.consultant_id,
        specialty_id: consultant.specialty_id,
        consultant_f_name: consultant.consultant_f_name,
        consultant_l_name: consultant.consultant_l_name,
        consultant_slug: consultant.consultant_slug,
    };

    // let json_response = serde_json::json!({
    //     "status": "success",
    //     "results": users.len(),
    //     "engagements": users
    // });
    HttpResponse::Ok().json(json_response)
    // HttpResponse::NoContent().finish()
}

// pub fn config(conf: &mut web::ServiceConfig) {
//     let scope = web::scope("/admin")
//         .service(get_users_handler)
//         .service(get_consultants_handler);

//     conf.service(scope);
// }
