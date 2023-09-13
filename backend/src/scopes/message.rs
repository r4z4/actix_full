use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

pub fn message_scope() -> Scope {
    web::scope("/messages")
        .route("/consult/{id}", web::get().to(get_messages))
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

async fn get_messages(path: web::Path<usize>) -> HttpResponse {
    let id: usize = path.into_inner();
    let content = String::from("Message Content");
    let message: Message = Message { id, content };
    HttpResponse::Ok().json(EncodeResponse {
        content: message.content,
        id: message.id,
    })
}