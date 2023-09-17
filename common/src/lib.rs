use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiLoginResponse {
    pub user_id: i32,
    pub username: String,
    pub token: String,
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
