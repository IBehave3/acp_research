use crate::infra::collection;
use crate::startup;
use log::info;
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::OnceLock;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

static DB: OnceLock<Database> = OnceLock::new();
pub type DbPool = Pool<AsyncPgConnection>;

pub fn get_db_connection() -> Database {
    match DB.get() {
        Some(db) => db.clone(),
        None => panic!("Error DB not set"),
    }
}

pub async fn init_db() {
    set_client_connection().await;

    //let db = &get_db_connection();

    //collection::create_collection(db, collection::ID_MAPPING_COLLECTION_NAME, None).await;
    //collection::create_collection(db, collection::AIRTHINGS_COLLECTION_NAME, None).await;
    //collection::create_collection(db, collection::GRAY_WOLF_COLLECTION_NAME, None).await;
    //collection::create_collection(db, collection::UHOO_AURA_COLLECTION_NAME, None).await;
    //collection::create_collection(db, collection::FITBIT_COLLECTION_NAME, None).await;
}

pub async fn set_client_connection() {
    let db_config = match startup::DB_CONFIG.get() {
        Some(db_config) => db_config,
        None => panic!("Error DB_CONFIG not initialized"),
    };

    let host = &db_config.host;
    let port = &db_config.port;
    let database = &db_config.database;
    let username = &db_config.username;
    let password = &db_config.password;

    let db_conn_string = format!("mongodb://{username}:{password}@{host}:{port}/{database}");

    info!("conn string: {db_conn_string}");

    let client_options = match ClientOptions::parse(db_conn_string).await {
        Ok(client_options) => client_options,
        Err(error) => panic!("Error: {:#?}", error),
    };

    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(error) => panic!("Error: {:#?}", error),
    };

    let default_database = match client.default_database() {
        Some(default_database) => default_database,
        None => panic!("Error not default database found for client"),
    };

    DB.set(default_database)
        .expect("Error DB should only be set once");
}
