use actix_web::{get, web, HttpResponse, Responder, HttpRequest, HttpMessage, Scope, delete, patch, post, put};
use chrono::{Utc, DateTime, FixedOffset};
use common::{ConsultPutRequest, ConsultPostRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use tracing::{instrument, Instrument};
use uuid::Uuid;
use validator::Validate;
use crate::{
    model::{FormConsultEdit, ResponseConsult, ResponseConsultList}
};

use crate::{schema::{FilterOptions, UpdateEngagementSchema, CreateEngagementSchema}, AppState};

pub fn api_consult_scope() -> Scope {
    web::scope("/api/consults")
        // .route("/users", web::get().to(get_users_handler))
        .service(get_consults_handler)
        .service(consults_form_handler)
        .service(consults_form_submit_handler)
        .service(consults_form_put_handler)
}

#[get("/")]
pub async fn get_consults_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);

    let query_result = sqlx::query_as!(
        ResponseConsult,
        "SELECT consult_id, location_id, consult_start, consult_attachments, notes 
        FROM consults 
		LIMIT $1",
        limit as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let _ = dbg!(query_result);
        let message = "Error occurred while fetching all consult records";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let consults = query_result.unwrap();

    dbg!(consults.clone());

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

#[get("/form/{id}")]
async fn consults_form_handler(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let consult_id = path.into_inner();
    let today = Utc::now();
    let formatted = today.format("%Y-%m-%d");
    let consult = sqlx::query_as!(
        FormConsultEdit,
        "SELECT consultant_id, client_id, location_id, consult_attachments, consult_start, consult_end, notes 
        FROM consults WHERE consult_id = $1",
        consult_id
    )
    .fetch_one(&data.db)
    .await;

    dbg!(&consult);

    match consult {
        Ok(consult) => {
            let consult_resp = serde_json::json!({"status": "success", "consult": consult
            });

            return HttpResponse::Ok().json(consult_resp);
        }
        Err(_) => {
            let message = format!("errpr fetching consult for edit_form");
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[derive(Clone, Debug, Serialize, FromRow, Deserialize)]
pub struct ConsultPostResponse {
    pub consult_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, FromRow, Deserialize)]
pub struct ConsultPutResponse {
    pub consult_id: i32,
    pub updated_at: DateTime<Utc>,
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
    dbg!(&body);
    let is_valid = body.validate();
    if is_valid.is_err() {
        return HttpResponse::InternalServerError().json(format!("{:?}", is_valid.err().unwrap()));
    }
    let consult: ConsultPostRequest = body.into_inner();
    let constructed_start_datetime = build_time(consult.start_date.unwrap(), consult.start_time.unwrap());
    let constructed_end_datetime = build_time(consult.end_date.unwrap(), consult.end_time.unwrap());
    let query_span = tracing::info_span!("Saving a new consult in the database");
    match sqlx::query_as::<_, ConsultPostResponse>(
        "INSERT INTO consults (client_id, consultant_id, location_id, consult_start, consult_end)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING consult_id, created_at",
    )
    .bind(consult.client_id)
    .bind(consult.consultant_id)
    .bind(consult.location_id)
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

#[put("/consults/form")]
async fn consults_form_put_handler(body: web::Json<ConsultPutRequest>, data: web::Data<AppState>) -> impl Responder {
    dbg!(&body);
    let is_valid = body.validate();
    if is_valid.is_err() {
        return HttpResponse::InternalServerError().json(format!("{:?}", is_valid.err().unwrap()));
    }
    let consult: ConsultPutRequest = body.into_inner();
    let constructed_start_datetime = build_time(consult.start_date.unwrap(), consult.start_time.unwrap());
    let constructed_end_datetime = build_time(consult.end_date.unwrap(), consult.end_time.unwrap());
    let query_span = tracing::info_span!("Saving a new consult in the database");
    match sqlx::query_as::<_, ConsultPutResponse>(
        "UPDATE consults 
        SET client_id = $1, consultant_id = $2, location_id = $3, consult_start = $4, consult_end = $5, updated_at = NOW()
        WHERE consult_id = $6
        RETURNING consult_id, updated_at",
    )
    .bind(consult.client_id)
    .bind(consult.consultant_id)
    .bind(consult.location_id)
    .bind(constructed_start_datetime)
    .bind(constructed_end_datetime)
    .bind(consult.consult_id)
    .fetch_one(&data.db)
    .instrument(query_span)
    .await
    {
        Ok(consult) => HttpResponse::Ok().json(consult),
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err)),
    }
}