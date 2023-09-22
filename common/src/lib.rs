use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Consult {
    pub consult_id: uuid::Uuid,
    pub client_id: i32,
    pub consultant_id: i32,
    pub user_id: i32,
    // String in common?
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Message {
    pub message_id: i32,
    pub content: String,
    pub subject: String,
    pub sent_to: i32,
    pub sent_from: i32,
    pub sent_at: Option<String>,
    pub read_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsultsFormResponse {
    pub today: String,
    pub location_options: Vec<SelectOption>,
    pub consultant_options: Vec<SelectOption>,
    pub client_options: Vec<SelectOption>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectOptionResponse {
    pub status: String,
    pub options: Option<Vec<SelectOption>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    pub key: String,
    pub value: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiLoginResponse {
    pub user_id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ConsultPostRequest {
    // If passing, it is an edit
    // pub consult_id: Option<i32>,
    pub client_id: i32,
    pub consultant_id: i32,
    pub consult_location: i32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    #[validate(length(min = 10, message = "Notes must be greater than 10 chars"))]
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiConsultResponse {
    pub consult_id: i32,
    pub consult_start: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Engagement {
    pub id: uuid::Uuid,
    pub text: String,
    pub rating: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngagementData {
    pub engagement: Engagement,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngagementResponse {
    pub status: String,
    pub data: EngagementData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngagementListResponse {
    pub status: String,
    pub results: i32,
    pub engagements: Vec<Engagement>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Consultant {
    pub consultant_id: uuid::Uuid,
    pub img_path: i32,
    pub specialty: String,
    pub territory: String,
    pub user_id: i32,
    // String in common?
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
