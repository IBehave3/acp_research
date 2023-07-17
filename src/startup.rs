use crate::infra::database;
use dotenv::dotenv;
use envy;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

pub static API_CONFIG: OnceLock<ServerConfig> = OnceLock::new();
pub static DB_CONFIG: OnceLock<DbConfig> = OnceLock::new();

pub async fn on_startup() {
    init_config();
    database::init_db().await;
}

fn init_config() {
    dotenv().ok();

    let api_config = match envy::prefixed("SERVER_").from_env::<ServerConfig>() {
        Ok(config) => config,
        Err(error) => panic!("Error: {:#?}", error),
    };

    API_CONFIG
        .set(api_config)
        .expect("Error API_CONFIG should only be set once");

    let db_config = match envy::prefixed("DB_").from_env::<DbConfig>() {
        Ok(config) => config,
        Err(error) => panic!("Error: {:#?}", error),
    };

    DB_CONFIG
        .set(db_config)
        .expect("Error DB_CONFIG should only be set once");
}
