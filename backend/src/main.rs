mod extractors;
mod handler;
mod model;
mod schema;
mod scopes;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::{Arc, Mutex};

pub struct AppState {
    db: Pool<Postgres>,
    secret: String,
    token: String,
}

use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let database_url = env!("DATABASE_URL");
    let secret = "secret".to_string();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let data = Arc::new(Mutex::new(web::Data::new(AppState {
        db: pool.clone(),
        secret: secret.clone(),
        token: "".to_string().clone(),
    })));

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(data.clone())
            .service(user_scope())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
