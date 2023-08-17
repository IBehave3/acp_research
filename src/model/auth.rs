
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    #[serde(rename(serialize = "clientId", deserialize = "clientId"))]
    pub client_id: Option<String>,
    #[serde(rename(serialize = "clientSecret", deserialize = "clientSecret"))]
    pub client_secret: Option<String>,
    #[serde(rename(serialize = "groupId", deserialize = "groupId"))]
    pub group_id: Option<String>,
    #[serde(rename(serialize = "apiKey", deserialize = "apiKey"))]
    pub api_key: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataStructureDeviceMapping {
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    pub data_structure_id: String,
    #[serde(rename(serialize = "deviceIds", deserialize = "deviceIds"))]
    pub device_ids: Option<HashSet<String>>,
    pub auth: Option<Auth>,
}

#[derive(Serialize, Deserialize, Debug)]
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
