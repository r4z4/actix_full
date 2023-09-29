mod actor;
mod config;
pub mod crypto;
mod extractors;
mod handler;
mod model;
mod schema;
mod scopes;
mod tests;
use crate::actor::MyActorHandle;
use crate::config::get_config;
use crate::crypto::{basic_auth, register_user};
use crate::scopes::api_engagement::api_engagement_scope;
use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::http::header::HeaderValue;
use actix_web::middleware::Logger;
use actix_web::{http, Error, HttpMessage};
use actix_web::{http::header, web, App, HttpServer};
use actix_web_httpauth::extractors::basic::{self, BasicAuth};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::{bearer, AuthenticationError};
use actix_web_httpauth::middleware::HttpAuthentication;
use common::SelectOption;
use dotenv::dotenv;
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use redis::{Client, Commands, ControlFlow, PubSubCommands};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::collections::BTreeMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use std::{env, thread};

use scopes::admin::admin_scope;
use scopes::message::message_scope;
use scopes::user::user_scope;

#[derive(Debug)]
pub struct AppState {
    db: Pool<Postgres>,
    secret: String,
    token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // let jwt_secret: String = std::env::var("JWT_SCRET").expect("JWT SCRET must be set");
    // let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    println!("VALIDATOR!!");
    let auth_header: Option<HeaderValue> = req.headers().get(http::header::AUTHORIZATION).cloned();
    if auth_header.is_none() {
        let config = req.app_data::<basic::Config>().cloned().unwrap_or_default();
        return Err((AuthenticationError::from(config).into(), req));
    }
    dbg!(&auth_header);
    let auth_token: Option<String> = Some(auth_header.unwrap().to_str().unwrap().to_string());

    let app_data: &AppState = req.app_data::<web::Data<AppState>>().unwrap();
    let claims: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &auth_token.unwrap(),
        &DecodingKey::from_secret(app_data.secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<basic::Config>().cloned().unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
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

        conn.subscribe(&["updates"], |msg| {
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

pub fn set_str(
    con: &mut redis::Connection,
    key: &str,
    value: &str,
    ttl_seconds: i32,
) -> Result<(), String> {
    let _ = con
        .set::<&str, &str, String>(key, value)
        .map_err(|e| e.to_string());
    if ttl_seconds > 0 {
        let _ = con
            .expire::<&str, String>(key, ttl_seconds.try_into().unwrap())
            .map_err(|e| e.to_string());
    }
    Ok(())
}

pub fn set_int(
    con: &mut redis::Connection,
    key: &str,
    value: i32,
    ttl_seconds: i32,
) -> Result<(), String> {
    let _ = con
        .set::<&str, i32, String>(key, value)
        .map_err(|e| e.to_string());
    if ttl_seconds > 0 {
        let _ = con
            .expire::<&str, String>(key, ttl_seconds.try_into().unwrap())
            .map_err(|e| e.to_string());
    }
    Ok(())
}

fn publish(state: &impl RedisState) {
    let client = Arc::clone(state.client());
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();

        for x in 0..11 {
            thread::sleep(Duration::from_millis(500));
            println!("Publish {} to updates.", x);
            let _: () = conn.publish("updates", x).unwrap();
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
    dotenv().ok();
    get_config();
    let database_url = env::var("DATABASE_URL").unwrap_or(env!("DATABASE_URL").to_owned());
    // let database_url = env!("DATABASE_URL");
    let secret = std::env::var("JWT_SECRET").unwrap_or(env!("JWT_SECRET").to_owned());
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

    let addrs = [
        SocketAddr::from(([0, 0, 0, 0], 8000)),
        SocketAddr::from(([0, 0, 0, 0], 8080)),
    ];
    if let Ok(conn) = TcpStream::connect(&addrs[..]) {
        println!("Connected to the server!");
        let handler: MyActorHandle = MyActorHandle::new(conn);
        let res = handler.send_message("Yo".to_string()).await;
        dbg!(res);
    } else {
        println!("Couldn't connect to server...");
    }

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
    let location_options: Vec<SelectOption> = vec![
        SelectOption {
            key: "location_one".to_string(),
            value: 1,
        },
        SelectOption {
            key: "location_two".to_string(),
            value: 2,
        },
    ];
    let mut option: BTreeMap<String, i32> = BTreeMap::new();
    let prefix = "select-option";
    option.insert(String::from("location_one"), 1);
    option.insert(String::from("location_two"), 2);
    // Set it in Redis
    let _: () = redis::cmd("HSET")
        .arg(format!("{}:{}", prefix, "location"))
        .arg(option)
        .query(&mut con)
        .expect("failed to execute HSET");
    let _ = set_int(&mut con, "answer", 44, 60);
    // let _: () = con.set("answer", 44).unwrap();
    let answer: i32 = con.get("answer").unwrap();
    println!("Answer: {}", answer);

    let info: BTreeMap<String, String> = redis::cmd("HGETALL")
        .arg(format!("{}:{}", prefix, "location"))
        .query(&mut con)
        .expect("failed to execute HGETALL");
    println!("info for rust redis driver: {:?}", info);

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
        // let bearer_middleware = HttpAuthentication::bearer(bearer_validator);
        let auth = HttpAuthentication::basic(validator);
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                secret: secret.clone(),
                token: "".to_string().clone(),
            }))
            .service(basic_auth)
            .service(register_user)
            // .service(
            //     web::scope("")
            //     .wrap(bearer_middleware)
            //     // Add effected routed
            //     .service(message_scope())
            // )
            .service(user_scope())
            .service(admin_scope())
            .service(api_engagement_scope())
            // .service(message_scope())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
