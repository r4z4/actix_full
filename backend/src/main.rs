mod extractors;
mod handler;
mod model;
mod schema;
mod scopes;
mod tests;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use redis::{Client, Commands, ControlFlow, PubSubCommands};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};

pub struct AppState {
    db: Pool<Postgres>,
    secret: String,
    token: String,
}

use scopes::message::message_scope;
use scopes::user::user_scope;

use crate::scopes::admin::admin_scope;

trait RedisState {
    fn client(&self) -> &Arc<Client>;
}

struct Ctx {
    pub client: Arc<Client>,
}

impl Ctx {
    fn new() -> Ctx {
        let redis_host_name =
            env::var("REDIS_HOSTNAME").unwrap_or(env!("REDIS_HOSTNAME").to_owned());
        let redis_password =
            env::var("REDIS_PASSWORD").unwrap_or(env!("REDIS_PASSWORD").to_owned());
        let redis_conn_url = format!("redis://:{}@{}:6379", redis_password, redis_host_name);
        let client = Client::open(redis_conn_url).unwrap();
        Ctx {
            client: Arc::new(client),
        }
    }
}

impl RedisState for Ctx {
    fn client(&self) -> &Arc<Client> {
        &self.client
    }
}

fn subscribe(state: &impl RedisState) -> thread::JoinHandle<()> {
    let client = Arc::clone(state.client());
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();

        conn.subscribe(&["boo"], |msg| {
            let ch = msg.get_channel_name();
            let payload: String = msg.get_payload().unwrap();
            match payload.as_ref() {
                "10" => ControlFlow::Break(()),
                a => {
                    println!("Channel '{}' received '{}'.", ch, a);
                    ControlFlow::Continue
                }
            }
        })
        .unwrap();
    })
}

fn publish(state: &impl RedisState) {
    let client = Arc::clone(state.client());
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();

        for x in 0..11 {
            thread::sleep(Duration::from_millis(500));
            println!("Publish {} to boo.", x);
            let _: () = conn.publish("boo", x).unwrap();
        }
    });
}

fn redis_connect() -> redis::Connection {
    //format - host:port
    let redis_host_name = env::var("REDIS_HOSTNAME").unwrap_or(env!("REDIS_HOSTNAME").to_owned());

    let redis_password = env::var("REDIS_PASSWORD").unwrap_or(env!("REDIS_PASSWORD").to_owned());
    let redis_conn_url = format!("redis://:{}@{}:6379", redis_password, redis_host_name);
    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let database_url = env::var("DATABASE_URL").unwrap_or(env!("DATABASE_URL").to_owned());
    // let database_url = env!("DATABASE_URL");
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

    // let data = Arc::new(Mutex::new(web::Data::new(AppState {
    //     db: pool.clone(),
    //     secret: secret.clone(),
    //     token: "".to_string().clone(),
    // })));

    println!("🚀 Server started successfully");

    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let mut con = client.get_connection().unwrap();
    // let _: () = con.set("answer", 44).unwrap();
    // let answer: i32 = con.get("answer").unwrap();
    // println!("Answer: {}", answer);
    let mut con = redis_connect();
    let _: () = con.set("answer", 44).unwrap();
    let answer: i32 = con.get("answer").unwrap();
    println!("Answer: {}", answer);

    let ctx = Ctx::new();
    let handle = subscribe(&ctx);
    publish(&ctx);
    handle.join().unwrap();
    // let mut pubsub = con.as_pubsub();
    // pubsub.subscribe("channel_1")?;
    // pubsub.subscribe("channel_2")?;
    //
    // loop {
    //     let msg = pubsub.get_message()?;
    //     let payload : String = msg.get_payload()?;
    //     println!("channel '{}': {}", msg.get_channel_name(), payload);
    // }

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
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                secret: secret.clone(),
                token: "".to_string().clone(),
            }))
            .service(user_scope())
            .service(admin_scope())
            .service(message_scope())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
