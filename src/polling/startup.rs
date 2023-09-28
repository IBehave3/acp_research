use crate::share::model::{DatabaseConfig, DATABASE_CONFIG};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenv::dotenv;

use super::infra::database::CONNECTION_POOL;

fn init_polling_config() {
    dotenv().ok();

    let database_config = match envy::from_env::<DatabaseConfig>() {
        Ok(config) => config,
        Err(error) => panic!("Error: {:#?}", error),
    };

    let url = database_config.database_url.clone();

    DATABASE_CONFIG
        .set(database_config)
        .expect("Error DATABASE_CONFIG should only be set once");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    let pool = match Pool::builder(config).max_size(25).build() {
        Ok(pool) => pool,
        Err(err) => {
            panic!("{err}");
        }
    };

    match CONNECTION_POOL.set(pool) {
        Ok(_) => (),
        Err(_) => {
            panic!("Error CONNECTION_POOL should only be set once")
        }
    }
}

pub async fn on_polling_startup() {
    init_polling_config();
}
