use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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
