use crate::{
    extractors::jwt_auth,
    model::{EngagementModel, ConsultModel},
    schema::{CreateEngagementSchema, FilterOptions, UpdateEngagementSchema},
    AppState,
};
use validator::Validate;
use actix_multipart::Multipart;
use actix_web::{
    delete, get, http::header::CONTENT_LENGTH, patch, post, web, HttpMessage, HttpRequest,
    HttpResponse, Responder,
};
use chrono::{prelude::*, format::strftime};
use common::{SelectOption, ConsultPostRequest, ResponseConsult, ResponseConsultList};
use futures_util::TryStreamExt;
use image::{imageops::FilterType, DynamicImage};
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG, IMAGE_SVG};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::{instrument, Instrument};
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

#[get("/engagement/{id}")]
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

#[get("/consults/form/{id}")]
async fn consults_form_handler(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let consult_id = path.into_inner();
    let today = Utc::now();
    let formatted = today.format("%Y-%m-%d");
    let consult = sqlx::query_as!(
        ConsultModel,
        "SELECT * FROM consults WHERE consult_id = $1",
        consult_id
    )
    .fetch_one(&data.db)
    .await;

    match consult {
        Ok(consult) => {
            let options_response = serde_json::json!({"status": "success", "consult": consult
            });

            return HttpResponse::Ok().json(options_response);
        }
        Err(_) => {
            let message = format!("errpr fetching locations");
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ConsultPostResponse {
    pub consult_id: i32,
    pub consult_start: DateTime<FixedOffset>,
}

fn build_time(date: String, time: String) -> DateTime<FixedOffset> {
    // '2023-09-11 19:10:25-06'
    let time = date + " " + &time + ":00 -06:00";
    let datetime = DateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S %:z").unwrap();
    // let datetime_utc = datetime.with_timezone(&Utc);
    datetime
}

#[instrument]
#[post("/consults/form")]
async fn consults_form_submit_handler(body: web::Json<ConsultPostRequest>, data: web::Data<AppState>) -> impl Responder {
    let is_valid = body.validate();
    if is_valid.is_err() {
        return HttpResponse::InternalServerError().json(format!("{:?}", is_valid.err().unwrap()));
    }
    let consult: ConsultPostRequest = body.into_inner();
    let constructed_start_datetime = build_time(consult.start_date.unwrap(), consult.start_time.unwrap());
    let constructed_end_datetime = build_time(consult.end_date.unwrap(), consult.end_time.unwrap());
    let query_span = tracing::info_span!("Saving a new consult in the database");
    match sqlx::query_as::<_, ConsultPostResponse>(
        "INSERT INTO consults (client_id, consultant_id, consult_location, consult_start, consult_end)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING consult_id, consult_start",
    )
    .bind(consult.client_id)
    .bind(consult.consultant_id)
    .bind(consult.consult_location)
    .bind(constructed_start_datetime)
    .bind(constructed_end_datetime)
    .fetch_one(&data.db)
    .instrument(query_span)
    .await
    {
        Ok(consult) => HttpResponse::Ok().json(consult),
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err)),
    }
}



#[get("/location-options")]
async fn location_options_handler(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        SelectOption,
        "SELECT location_name AS key, location_id AS value FROM locations"
    )
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(options) => {
            let options_response = serde_json::json!({"status": "success", "options": options
            });

            return HttpResponse::Ok().json(options_response);
        }
        Err(_) => {
            let message = format!("errpr fetching locations");
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("/account-options")]
async fn account_options_handler(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        SelectOption,
        "SELECT account_name AS key, account_id AS value FROM accounts"
    )
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(options) => {
            let options_response = serde_json::json!({"status": "success", "options": options
            });

            return HttpResponse::Ok().json(options_response);
        }
        Err(_) => {
            let message = format!("errpr fetching locations");
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("/client-options")]
async fn client_options_handler(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        SelectOption,
        "SELECT client_slug AS key, client_id AS value FROM clients"
    )
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(options) => {
            let options_response = serde_json::json!({"status": "success", "options": options
            });

            return HttpResponse::Ok().json(options_response);
        }
        Err(_) => {
            let message = format!("errpr fetching locations");
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("/consultant-options")]
async fn consultant_options_handler(data: web::Data<AppState>) -> impl Responder {
    // Maintain a map of slugs to names then
    let query_result = sqlx::query_as!(
        SelectOption,
        "SELECT consultant_slug AS key, consultant_id AS value FROM consultants"
    )
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(options) => {
            let options_response = serde_json::json!({"status": "success", "options": options
            });

            return HttpResponse::Ok().json(options_response);
        }
        Err(_) => {
            let message = format!("errpr fetching consultants");
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[post("/upload/{id}")]
async fn upload(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
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

    if content_length == 0 || content_length > max_file_size {
        return HttpResponse::BadRequest().into();
    }

    let mut current_count: usize = 0;
    loop {
        if current_count >= max_file_count {
            break;
        }

        if let Ok(Some(mut field)) = payload.try_next().await {
            if field.name() != "upload" {
                continue;
            }
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() {
                continue;
            }
            if !legal_file_types.contains(&filetype.unwrap()) {
                continue;
            }
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
            })
            .await
            .unwrap()
            .await;
        } else {
            break;
        }
        current_count += 1;
    }

    HttpResponse::Ok().into()
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

#[get("/consults")]
pub async fn get_consults_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ResponseConsult,
        "SELECT consult_id, location_id, consult_start, notes FROM consults ORDER by consult_id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Error occurred while fetching all consult records";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let consults = query_result.unwrap();

    let json_response = ResponseConsultList {
        consults: consults,
    };

    // let json_response = serde_json::json!({
    //     "status": "success",
    //     "results": users.len(),
    //     "engagements": users
    // });
    HttpResponse::Ok().json(json_response)
    // HttpResponse::NoContent().finish()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultantList {
    pub consultants: Vec<ResponseConsultant>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultant {
    pub consultant_id: i32,
    pub specialty_id: i32,
    pub img_path: Option<String>,
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
        "SELECT consultant_id, specialty_id, img_path FROM consultants ORDER by consultant_id LIMIT $1 OFFSET $2",
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

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(engagement_list_handler)
        .service(create_engagement_handler)
        .service(get_engagement_handler)
        .service(edit_engagement_handler)
        .service(delete_engagement_handler)
        .service(get_users_handler)
        .service(get_consultants_handler)
        .service(location_options_handler)
        .service(consultant_options_handler)
        .service(client_options_handler)
        .service(consults_form_submit_handler)
        .service(get_consults_handler)
        .service(account_options_handler);

    conf.service(scope);
}
