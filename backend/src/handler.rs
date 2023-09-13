use crate::{
    model::EngagementModel,
    schema::{CreateEngagementSchema, FilterOptions, UpdateEngagementSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, HttpRequest, http::header::CONTENT_LENGTH};
use chrono::prelude::*;
use futures_util::TryStreamExt;
use image::{DynamicImage, imageops::FilterType};
use serde_json::json;
use mime::{ Mime, IMAGE_JPEG, IMAGE_PNG, IMAGE_SVG, IMAGE_GIF};
use actix_multipart::Multipart;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Health Check Response";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/engagements")]
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

#[post("/engagement/")]
async fn create_engagement_handler(
    body: web::Json<CreateEngagementSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
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

#[get("/engagement/{id}")]
async fn get_engagement_handler(
    path: web::Path<i32>,
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

#[patch("/engagement/{id}")]
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

#[delete("/engagement/{id}")]
async fn delete_engagement_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM engagements WHERE engagement_id = $1", engagement_id)
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

#[post("/upload/{id}")]
async fn upload(mut payload: Multipart, req: HttpRequest) ->  HttpResponse {
    // 1. Limit file size
    // 2. Limit file count
    // 3. Limit file type
    // 4. Save file
    // 5. Covert into Gif

    let max_file_size: usize = 10_000;
    let max_file_count: usize = 3;
    let legal_file_types: [Mime; 4] = [IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG, IMAGE_SVG];

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => 0, 
    };

    if content_length == 0 || content_length > max_file_size { return HttpResponse::BadRequest().into(); } 

    let mut current_count: usize = 0;
    loop {
        if current_count >= max_file_count { break; }

        if let Ok(Some(mut field)) = payload.try_next().await {
            if field.name() != "upload" { continue; }
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() { continue; }
            if !legal_file_types.contains(&filetype.unwrap()) { continue; }
            let dir: &str = "./upload";

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap(),
            );
            let mut saved_file = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            web::block(move || async move {
                let updated_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).await.unwrap();
                updated_img
                    .resize_exact(200, 200, FilterType::Nearest)
                    .save(format!("{}{}.gif", dir, Uuid::new_v4()))
                    .unwrap();
            }).await.unwrap().await;

        } else {
            break;
        }
        current_count += 1;
    }

    HttpResponse::Ok().into()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(engagement_list_handler)
        .service(create_engagement_handler)
        .service(get_engagement_handler)
        .service(edit_engagement_handler)
        .service(delete_engagement_handler);

    conf.service(scope);
}