use async_trait::async_trait;

use bson::Bson;
use futures::stream::TryStreamExt;
use log::info;
use mongodb::options::{DeleteOptions, UpdateOptions};
use mongodb::IndexModel;
use mongodb::{bson::doc, options::FindOneOptions, options::FindOptions};
use mongodb::{
    Collection, Database,
    {bson::oid::ObjectId, bson::Document},
};
use serde::{de::DeserializeOwned, Serialize};

pub const ID_MAPPING_COLLECTION_NAME: &str = "id_mapping";
pub const GRAY_WOLF_COLLECTION_NAME: &str = "gray_wolf";
pub const UHOO_AURA_COLLECTION_NAME: &str = "uhoo_aura";
pub const AIRTHINGS_COLLECTION_NAME: &str = "airthings";
pub const FITBIT_COLLECTION_NAME: &str = "fitbit";

pub async fn create_collection(db: &Database, container_name: &str, model: Option<IndexModel>) {
    match db.create_collection(container_name, None).await {
        Err(_) => info!("container {container_name} already exists"),
        Ok(_) => {
            info!("contianer {container_name} created");
            info!("creating indexes for {container_name}");

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
        doc: &Self::DocumentType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "_id": oid };
        Self::replace_options(filter, doc).await?;
        Ok(())
    }

    async fn replace_options(
        filter: Document,
        doc: &Self::DocumentType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        collection.replace_one(filter, doc, None).await?;
        Ok(())
    }

    async fn update(filter: Document, update: Document) -> Result<(), Box<dyn std::error::Error>> {
        Self::update_options(filter, update, None).await?;
        Ok(())
    }

    async fn update_options(
        filter: Document,
        update: Document,
        options: Option<UpdateOptions>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        collection.update_one(filter, update, options).await?;
        Ok(())
    }

    async fn delete(oid: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "_id" : oid};
        Self::delete_options(filter, None).await?;
        Ok(())
    }

    async fn delete_options(
        filter: Document,
        options: Option<DeleteOptions>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        collection.delete_one(filter, options).await?;
        Ok(())
    }

    async fn delete_all() -> Result<(), Box<dyn std::error::Error>> {
        Self::delete_all_options(doc! {}, None).await?;
        Ok(())
    }

    async fn delete_all_options(
        filter: Document,
        options: Option<DeleteOptions>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        collection.delete_many(filter, options).await?;
        Ok(())
    }

    async fn add(doc: Self::DocumentType) -> Result<Bson, Box<dyn std::error::Error>> {
        let collection = Self::get_collection();
        let result = collection.insert_one(doc, None).await?;
        Ok(result.inserted_id)
    }
}
