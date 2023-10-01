use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize)]
pub enum StateAbbr {
    AK,
    AL,
    NE,
    MN,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MimeType {
    ApplicationPdf,
    ApplicationJson,
    ImageJpeg,
    ImagePng,
    ImageGif,
    ImageSvg,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AttachmentChannel {
    Email,
    Upload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsult {
    pub consult_id: i32,
    pub location_id: i32,
    // #[serde(serialize_with = "serialize_dt", skip_serializing_if  = "Option::is_none")]
    pub consult_start: DateTime<Utc>,
    // Using CONCAT does not return a Vec<>
    pub consult_attachments: Option<String>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsultPostResponse {
    pub consult_id: i32,
    pub consult_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultList {
    pub consults: Vec<ResponseConsult>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct EngagementModel {
    pub engagement_id: i32,
    pub text: String,
    pub rating: i32,
    pub user_id: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub secret: Option<String>,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ConsultantModel {
    pub consultant_id: i32,
    pub user_id: i32,
    pub img_path: String,
    pub specialty: String,
    pub territory: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ClientModel {
    pub client_id: i32,
    pub client_slug: uuid::Uuid,
    pub user_id: i32,
    pub client_company_name: String,
    pub client_f_name: String,
    pub client_l_name: String,
    pub client_dob: Option<chrono::NaiveDate>,
    pub client_address_one: String,
    pub client_address_two: Option<String>,
    pub client_city: String,
    pub client_state: StateAbbr,
    pub client_zip: u32,
    pub client_home_phone: Option<String>,
    pub client_mobile_phone: Option<String>,
    pub client_office_phone: Option<String>,
    pub client_email: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ContactModel {
    pub contact_id: i32,
    pub contact_title: String,
    pub contact_f_name: String,
    pub contact_l_name: String,
    pub contact_email: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ConsultModel {
    pub consult_id: i32,
    pub client_id: i32,
    pub consultant_id: i32,
    pub location_id: i32,
    pub consult_start: Option<chrono::DateTime<chrono::Utc>>,
    pub consult_end: Option<chrono::DateTime<chrono::Utc>>,
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MessageModel {
    pub message_id: i32,
    pub content: String,
    pub subject: String,
    pub sent_to: i32,
    pub sent_from: i32,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LocationModel {
    pub location_id: i32,
    pub location_slug: uuid::Uuid,
    pub location_address_one: String,
    pub location_address_two: Option<String>,
    pub location_city: String,
    pub location_state: String,
    pub location_zip: String,
    pub location_phone: String,
    pub location_contact_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AttachmentModel {
    pub attachment_id: i32,
    pub path: String,
    pub mime_type: MimeType,
    pub user_id: i32,
    pub channel: AttachmentChannel,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
