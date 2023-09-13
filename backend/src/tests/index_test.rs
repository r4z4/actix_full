use actix_web::HttpResponse;
use actix_web::dev::Service;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use actix_web::{http::{header::ContentType, StatusCode}, test, App, web};

    use super::*;

    #[actix_web::test]
    async fn test_init_service() {
        let app = test::init_service(
            App::new()
                .service(web::resource("/test").to(|| async { "OK" }))
        ).await;
    
        // Create request object
        let req = test::TestRequest::with_uri("/test").to_request();
    
        // Execute application
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}