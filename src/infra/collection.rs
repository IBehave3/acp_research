use crate::infra::database;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOneOptions, options::FindOptions};
use mongodb::{
    bson::{oid::ObjectId, Document},
    Collection, Database,
};
use serde::{de::DeserializeOwned, Serialize};

pub const ID_MAPPING_COLLECTION_NAME: &str = "id_mapping";
pub const PUSH_DATA_COLLECTION_NAME: &str = "push_data";
pub const NOTIFICATION_COLLECTION_NAME: &str = "notification";
pub const TEST_COLLECTION_NAME: &str = "test";

pub async fn create_collection(db: &Database, container_name: &str) {
    match db.create_collection(container_name, None).await {
        Err(_) => println!("Info container {container_name} already exists"),
        Ok(_) => println!("Infor contianer {container_name} created"),
    };
}

pub async fn get_all_collection(collection_name: &str) -> Vec<Document> {
    let db = database::get_db_connection();
    let collection: Collection<Document> = db.collection(collection_name);
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut results: Vec<Document> = Vec::new();

    while cursor.advance().await.unwrap() == true {
        results.push(cursor.deserialize_current().unwrap());
    }

    return results;
}

pub async fn get_all_collection_names() -> Vec<String> {
    let db = database::get_db_connection();
    let mut results: Vec<String> = Vec::new();

    for name in db.list_collection_names(None).await.unwrap() {
        results.push(name);
    }

    return results;
}

#[async_trait]
pub trait BaseCollection {
    type DocumentType: Send + Sync + DeserializeOwned + Serialize + Unpin;

    fn get_collection() -> Collection<Self::DocumentType>;

    async fn get(oid: ObjectId) -> Option<Self::DocumentType> {
        let filter = doc! { "_id": oid };
        return Self::get_options(Some(filter), None).await;
    }

    async fn get_options(
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> Option<Self::DocumentType> {
        let collection = Self::get_collection();
        return collection.find_one(filter, options).await.unwrap();
    }

    async fn get_all() -> Vec<Self::DocumentType> {
        let collection = Self::get_collection();
        let mut cursor = collection.find(None, None).await.unwrap();
        let mut results: Vec<Self::DocumentType> = Vec::new();

        while let Some(doc) = cursor.try_next().await.unwrap() {
            results.push(doc);
        }

        results
    }

    async fn get_all_options(
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Vec<Self::DocumentType> {
        let collection = Self::get_collection();
        let mut cursor = collection.find(filter, options).await.unwrap();
        let mut results: Vec<Self::DocumentType> = Vec::new();

        while let Some(doc) = cursor.try_next().await.unwrap() {
            results.push(doc);
        }

        results
    }

    async fn add(doc: Self::DocumentType) {
        let collection = Self::get_collection();
        collection.insert_one(doc, None).await.unwrap();
    }
}
