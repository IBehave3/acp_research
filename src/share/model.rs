use std::sync::OnceLock;

use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct  DatabaseConfig {
    pub database_url: String,
}

pub static DATABASE_CONFIG: OnceLock<DatabaseConfig> = OnceLock::new();
