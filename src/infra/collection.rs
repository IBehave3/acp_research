use crate::infra::database;
use async_trait::async_trait;
use bson::Bson;
use futures::stream::TryStreamExt;
use mongodb::options::{CreateCollectionOptions, CreateIndexOptions, DeleteOptions, IndexOptions};
use mongodb::IndexModel;
use mongodb::{bson::doc, options::FindOneOptions, options::FindOptions};
use mongodb::{
    Collection, Database,
    {bson, bson::oid::ObjectId, bson::Document},
};
use serde::{de::DeserializeOwned, Serialize};

pub const ID_MAPPING_COLLECTION_NAME: &str = "id_mapping";
pub const PUSH_DATA_COLLECTION_NAME: &str = "push_data";
pub const NOTIFICATION_COLLECTION_NAME: &str = "notification";
pub const TEST_COLLECTION_NAME: &str = "test";

pub async fn create_collection(db: &Database, container_name: &str, model: Option<IndexModel>) {
    match db.create_collection(container_name, None).await {
        Err(_) => println!("Info container {container_name} already exists"),
        Ok(_) => {
            println!("Info contianer {container_name} created");
            println!("Info creating indexes for {container_name}");

            if model.is_some() {
                let model = model.unwrap();
                let collection: Collection<Document> = db.collection(container_name);
                collection
                    .create_index(model, None)
                    .await
                    .expect("Error failed to create index");
            }
        }
    };
}

pub async fn get_all_collection(
    collection_name: &str,
) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
    let db = database::get_db_connection();
    let collection: Collection<Document> = db.collection(collection_name);
    let mut cursor = collection.find(None, None).await?;
    let mut results: Vec<Document> = Vec::new();

    while cursor.advance().await? {
        results.push(cursor.deserialize_current()?);
    }

    Ok(results)
}

pub async fn get_all_collection_names() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let db = database::get_db_connection();
    let mut results: Vec<String> = Vec::new();

    for name in db.list_collection_names(None).await? {
        results.push(name);
    }

    Ok(results)
}

#[async_trait]
pub trait BaseCollection {
    type DocumentType: Send + Sync + DeserializeOwned + Serialize + Unpin;

    fn get_collection() -> Collection<Self::DocumentType>;

    async fn get(oid: ObjectId) -> Result<Option<Self::DocumentType>, Box<dyn std::error::Error>> {
        let filter = doc! { "_id": oid };
        return Self::get_options(Some(filter), None).await;
    }

    async fn get_options(
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Self::DocumentType>, Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        let result = collection.find_one(filter, options).await?;
        return Ok(result);
    }

    async fn get_all() -> Result<Vec<Self::DocumentType>, Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        let mut cursor = collection.find(None, None).await?;
        let mut results: Vec<Self::DocumentType> = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

    async fn get_all_options(
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Vec<Self::DocumentType>, Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        let mut cursor = collection.find(filter, options).await?;
        let mut results: Vec<Self::DocumentType> = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

    async fn replace(
        oid: ObjectId,
        doc: Self::DocumentType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "_id": oid };
        Self::replace_options(filter, doc).await?;
        Ok(())
    }

    async fn replace_options(
        filter: Document,
        doc: Self::DocumentType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        collection.replace_one(filter, doc, None).await?;
        Ok(())
    }

    async fn delete(oid: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "_id": oid };
        Self::delete_options(filter, None).await?;
        Ok(())
    }

    async fn delete_options(
        filter: Document,
        options: Option<DeleteOptions>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        collection.delete_many(filter, options).await?;
        Ok(())
    }

    async fn add(doc: Self::DocumentType) -> Result<(), Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        collection.insert_one(doc, None).await?;
        Ok(())
    }
}
