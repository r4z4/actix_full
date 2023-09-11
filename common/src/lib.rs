use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Consult {
    pub consult_id: uuid::Uuid,
    pub client_id: i32,
    pub consultant_id: i32,
    pub specialty: String,
    pub territory: String,
    pub user_id: i32,
    // String in common?
    pub created_at: String,
    pub updated_at: Option<String>,
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
