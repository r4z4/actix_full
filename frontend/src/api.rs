use common::{Engagement, EngagementListResponse, EngagementResponse, ErrorResponse};
use reqwasm::http;

pub async fn api_create_engagement(engagement_data: &str) -> Result<Engagement, String> {
    let response = match http::Request::post("http://localhost:8000/api/engagements/create")
        .header("Content-Type", "application/json")
        .body(engagement_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<EngagementResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.engagement),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn _api_fetch_single_engagement(engagement_id: &str) -> Result<Engagement, String> {
    let response = match http::Request::get(
        format!("http://localhost:8000/api/engagements/{}", engagement_id).as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<EngagementResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.engagement),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_fetch_engagements((page, limit): (i32, i32)) -> Result<Vec<Engagement>, String> {
    let response = match http::Request::get(
        format!(
            "http://localhost:8000/api/engagements?page={}&limit={}",
            page, limit
        )
        .as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<EngagementListResponse>().await;
    match res_json {
        Ok(data) => Ok(data.engagements),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_delete_engagement(engagement_id: &str) -> Result<(), String> {
    let response = match http::Request::delete(
        format!("http://localhost:8000/api/engagements/{}", engagement_id).as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 204 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    Ok(())
}
