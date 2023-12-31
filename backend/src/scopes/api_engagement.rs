use actix_web::{get, web, HttpResponse, Responder, HttpRequest, HttpMessage, Scope, delete, patch, post};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{schema::{FilterOptions, UpdateEngagementSchema, CreateEngagementSchema}, AppState, extractors::jwt_auth, model::EngagementModel};

pub fn api_engagement_scope() -> Scope {
    web::scope("/api/engagements")
        // .route("/users", web::get().to(get_users_handler))
        .service(engagement_list_handler)
        .service(create_engagement_handler)
        .service(get_engagement_handler)
        .service(edit_engagement_handler)
        .service(delete_engagement_handler)
}

#[get("/")]
pub async fn engagement_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        EngagementModel,
        "SELECT * FROM engagements ORDER by engagement_id LIMIT $1 OFFSET $2",
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

    let engagements = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": engagements.len(),
        "engagements": engagements
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/create")]
async fn create_engagement_handler(
    body: web::Json<CreateEngagementSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding an engagement.",
        %request_id,
        // user_id = %form.email,
        text= body.text.to_string()
    );
    let _request_span_guard = request_span.enter();

    let query_result = sqlx::query_as!(
        EngagementModel,
        "INSERT INTO engagements (text,rating) VALUES ($1, $2) RETURNING *",
        body.text.to_string(),
        body.rating,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(engagement) => {
            let engagement_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "engagement": engagement
            })});

            return HttpResponse::Ok().json(engagement_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "engagement with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/{id}")]
async fn get_engagement_handler(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let engagement_id = path.into_inner();
    let query_result = sqlx::query_as!(
        EngagementModel,
        "SELECT * FROM engagements WHERE engagement_id = $1",
        engagement_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(engagement) => {
            let engagement_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "engagement": engagement
            })});

            return HttpResponse::Ok().json(engagement_response);
        }
        Err(_) => {
            let message = format!("engagement with ID: {} not found", engagement_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[patch("/{id}")]
async fn edit_engagement_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateEngagementSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let query_result = sqlx::query_as!(
        EngagementModel,
        "SELECT * FROM engagements WHERE engagement_id = $1",
        engagement_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("engagement with ID: {} not found", engagement_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let now = Utc::now();
    let engagement = query_result.unwrap();

    let query_result = sqlx::query_as!(
        EngagementModel,
        "UPDATE engagements SET text = $1, rating = $2, updated_at = $3 WHERE engagement_id = $4 RETURNING *",
        body.text.to_owned().unwrap_or(engagement.text),
        body.rating.to_owned().unwrap_or(engagement.rating),
        now,
        engagement_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(engagement) => {
            let engagement_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "engagement": engagement
            })});

            return HttpResponse::Ok().json(engagement_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/{id}")]
async fn delete_engagement_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let rows_affected = sqlx::query!(
        "DELETE FROM engagements WHERE engagement_id = $1",
        engagement_id
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let message = format!("engagement with ID: {} not found", engagement_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}