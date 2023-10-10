use crate::{
    extractors::jwt_auth,
    model::{ConsultModel, ResponseConsult, ResponseConsultList},
    schema::FilterOptions,
    AppState,
};
use serde_with::{StringWithSeparator, formats::CommaSeparator};
use validator::Validate;
use actix_multipart::{Multipart, form::{MultipartForm, tempfile::TempFile}};
use actix_web::{
    delete, get, http::{header::CONTENT_LENGTH, Error}, patch, post, web, HttpMessage, HttpRequest,
    HttpResponse, Responder, put,
};
use common::SelectOption;
use futures_util::TryStreamExt;
use image::{imageops::FilterType, DynamicImage};
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG, IMAGE_SVG};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Health Check Response";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
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
        "SELECT COALESCE(client_company_name, CONCAT(client_f_name, ' ', client_l_name)) AS key, client_id AS value FROM clients"
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
        "SELECT CONCAT(consultant_f_name, ' ',consultant_l_name) AS key, consultant_id AS value FROM consultants"
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

#[get("/specialty-options")]
async fn specialty_options_handler(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        SelectOption,
        "SELECT specialty_name AS key, specialty_id AS value FROM specialties"
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

#[get("/territory-options")]
async fn territory_options_handler(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        SelectOption,
        "SELECT territory_name AS key, territory_id AS value FROM territories"
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

// #[derive(Debug, MultipartForm)]
// struct UploadForm {
//     #[multipart(rename = "file")]
//     files: Vec<TempFile>,
// }

// async fn save_files(
//     MultipartForm(form): MultipartForm<UploadForm>,
// ) -> Result<impl Responder, Error> {
//     for f in form.files {
//         let path = format!("./tmp/{}", f.file_name.unwrap());
//         // log::info!("saving to {path}");
//         println!("saving to {path}");
//         f.file.persist(path).unwrap();
//     }

//     Ok(HttpResponse::Ok())
// }

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

#[get("/clients")]
pub async fn get_clients_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ResponseClient,
        "SELECT client_id, client_address_one, client_city, client_zip FROM clients ORDER by client_id LIMIT $1 OFFSET $2",
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

    let clients = query_result.unwrap();

    let json_response = ResponseClientList {
        clients: clients,
    };

    // let json_response = serde_json::json!({
    //     "status": "success",
    //     "results": users.len(),
    //     "engagements": users
    // });
    HttpResponse::Ok().json(json_response)
    // HttpResponse::NoContent().finish()
}

#[get("/clients/{id}")]
pub async fn get_client_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let client_id = path.into_inner();
    let query_result = sqlx::query_as!(
        ResponseClient,
        "SELECT client_id, client_address_one, client_city, client_zip FROM clients WHERE client_id = $1",
        client_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Error occurred while fetching all consult records";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let client = query_result.unwrap();

    let json_response = ResponseClient {
        client_id: client.client_id,
        client_address_one: client.client_address_one,
        client_city: client.client_city,
        client_zip: client.client_zip,
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
pub struct ResponseClientList {
    pub clients: Vec<ResponseClient>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseClient {
    pub client_id: i32,
    pub client_address_one: String,
    pub client_city: String,
    pub client_zip: String,
}

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug,Serialize,Deserialize)]
pub struct AttachmentIdParams {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub ids: Vec<String>
}

#[get("/attachments/{ids}")]
pub async fn get_attachments_handler(
    path: web::Path<AttachmentIdParams>,
    data: web::Data<AppState>,
) -> impl Responder {
    let attachment_id_strs = path.into_inner().ids;
    let attachment_id_ints: Vec<i32> = attachment_id_strs.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let ints = &vec![1,2];
    let query_result = sqlx::query_as!(
        ResponseAttachment,
        "SELECT attachment_id, path, mime_type_id FROM attachments WHERE attachment_id = ANY($1::integer[])",
        &attachment_id_ints
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Error occurred while fetching all consult records";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let attachments = query_result.unwrap();

    let json_response = ResponseAttachmentList {
        attachments: attachments,
    };

    HttpResponse::Ok().json(json_response)
    // HttpResponse::NoContent().finish()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAttachmentList {
    pub attachments: Vec<ResponseAttachment>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAttachment {
    pub attachment_id: i32,
    pub path: String,
    pub mime_type_id: i32,
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(location_options_handler)
        .service(consultant_options_handler)
        .service(client_options_handler)
        .service(territory_options_handler)
        .service(specialty_options_handler)
        .service(get_clients_handler)
        .service(get_client_handler)
        .service(get_attachments_handler)
        .service(account_options_handler);

    conf.service(scope);
}
