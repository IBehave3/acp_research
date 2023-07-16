use crate::infra::collection;
use crate::startup;
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::OnceLock;

static DB: OnceLock<Database> = OnceLock::new();

pub fn get_db_connection() -> Database {
    DB.get().unwrap().clone()
}

pub async fn init_db() {
    set_client_connection().await;

    let db = &get_db_connection();

    collection::create_collection(&db, collection::ID_MAPPING_COLLECTION_NAME).await;
    collection::create_collection(&db, collection::PUSH_DATA_COLLECTION_NAME).await;
    collection::create_collection(&db, collection::NOTIFICATION_COLLECTION_NAME).await;
    collection::create_collection(&db, collection::TEST_COLLECTION_NAME).await;
}

pub async fn set_client_connection() {
    let host = &startup::DB_CONFIG.get().unwrap().host;
    let port = &startup::DB_CONFIG.get().unwrap().port;
    let database = &startup::DB_CONFIG.get().unwrap().database;

    let db_conn_string = format!("mongodb://{host}:{port}/{database}");

    println!("Info conn string: {db_conn_string}");

    let client_options = match ClientOptions::parse(db_conn_string).await {
        Ok(client_options) => client_options,
        Err(error) => panic!("{:#?}", error),
    };

    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(error) => panic!("{:#?}", error),
    };

    DB.set(client.default_database().unwrap())
        .expect("Error DB should only be set once");
}
