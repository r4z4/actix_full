use actix_web::{web, HttpResponse, Scope, Responder, get};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{model::UserModel, AppState, schema::FilterOptions};

pub fn admin_scope() -> Scope {
    web::scope("/admin")
        // .route("/users", web::get().to(get_users_handler))
        .route("/users/{id}", web::get().to(get_user_handler))
}

// #[derive(Serialize, Deserialize)]
// struct Response {
//     message: String,
// }

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

#[get("/users")]
pub async fn get_users_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
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

    // let json_response = ResponseUserList {
    //     users: users
    // };

    let json_response = serde_json::json!({
        "status": "success",
        "results": users.len(),
        "engagements": users
    });
    // HttpResponse::Ok().json(json_response)
    HttpResponse::NoContent().finish()
}

async fn get_user_handler(path: web::Path<usize>) -> HttpResponse {
    let id: usize = path.into_inner();
    let content = String::from("Message Content");
    let message: Message = Message { id, content };
    HttpResponse::Ok().json(EncodeResponse {
        content: message.content,
        id: message.id,
    })
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/admin")
        .service(get_users_handler);

    conf.service(scope);
}