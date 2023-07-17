use crate::infra::{collection, collection::BaseCollection, database};
use actix_web::Result;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};

impl BaseCollection for IdMapping {
    type DocumentType = IdMapping;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::ID_MAPPING_COLLECTION_NAME)
    }
}

impl IdMapping {
    pub async fn get_id_mapping_by_user_id(
        user_id: &str,
    ) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let filter = doc! { "userId": user_id };
        return IdMapping::get_options(Some(filter), None).await;
    }

    pub async fn id_mapping_exists(user_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let res = Self::get_id_mapping_by_user_id(user_id).await?;
        Ok(res.is_some())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataStructureDeviceMapping {
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    pub data_structure_id: String,
    #[serde(rename(serialize = "deviceIds", deserialize = "deviceIds"))]
    pub device_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct IdMapping {
    pub _id: ObjectId,
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(
        serialize = "dataStructureDeviceIdMapping",
        deserialize = "dataStructureDeviceIdMapping"
    ))]
    pub data_structure_device_id_mapping: Vec<DataStructureDeviceMapping>,
}

#[derive(Deserialize, Serialize)]
pub struct UserIdQueryExtractor {
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    pub user_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserPostJsonExtractor {
    #[serde(rename(
        serialize = "dataStructureDeviceMapping",
        deserialize = "dataStructureDeviceMapping"
    ))]
    pub data_structure_device_mapping: Vec<DataStructureDeviceMapping>,
}
