use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum StateAbbr {
    AK, AL, NE, MN
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
    pub specialty: Option<String>,
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
pub struct ConsultModel {
    pub consult_id: i32,
    pub client_id: i32,
    pub consultant_id: i32,
    pub consult_location: String,
    pub consult_start: Option<chrono::DateTime<chrono::Utc>>,
    pub consult_end: Option<chrono::DateTime<chrono::Utc>>,
    pub notes: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
