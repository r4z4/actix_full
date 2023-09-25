//! src/configuration.rs

use std::collections::HashMap;
use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub db_username: String,
    pub db_password: String,
    pub db_port: u16,
    pub db_host: String,
    pub db_name: String,
}

pub fn get_config() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(
            "/home/ar3rz/rust/rust_full/backend/src/.config/Settings.toml",
        ))
        // .add_source(File::from_str("config/settings", config::FileFormat::Yaml))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_username, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}
