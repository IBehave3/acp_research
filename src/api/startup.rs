use dotenv::dotenv;

use serde::Deserialize;
use std::sync::OnceLock;

use crate::share::model::{DATABASE_CONFIG, DatabaseConfig};

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub static API_CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub async fn on_api_startup() {
    init_api_config();
}

fn init_api_config() {
    dotenv().ok();

    let api_config = match envy::prefixed("SERVER_").from_env::<ServerConfig>() {
        Ok(config) => config,
        Err(error) => panic!("Error: {:#?}", error),
    };

    API_CONFIG
        .set(api_config)
        .expect("Error API_CONFIG should only be set once");

    let database_config = match envy::from_env::<DatabaseConfig>() {
        Ok(config) => config,
        Err(error) => panic!("Error: {:#?}", error),
    };

    DATABASE_CONFIG
        .set(database_config)
        .expect("Error DATABASE_CONFIG should only be set once");
}
