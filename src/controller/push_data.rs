use crate::controller::auth::UserIdQueryExtractor;
use crate::infra::{collection, collection::BaseCollection, database};
use mongodb::{bson, bson::doc, bson::oid::ObjectId, bson::Document};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserPushData {
    #[serde(flatten)]
    pub data: Document,
}

#[derive(Deserialize, Serialize)]
pub struct PushData {
    pub _id: ObjectId,
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: bson::DateTime,
    #[serde(rename(serialize = "idMappingRefId", deserialize = "idMappingRefId"))]
    pub id_mapping_ref_id: ObjectId,
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    pub data_structure_id: String,
    pub data: UserPushData,
}

impl BaseCollection for PushData {
    type DocumentType = PushData;

    fn get_collection() -> mongodb::Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::PUSH_DATA_COLLECTION_NAME)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PushDataQueryExtractor {
    #[serde(flatten)]
    pub user_id_query_extractor: UserIdQueryExtractor,
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    pub data_structure_id: String,
    #[serde(rename(serialize = "deviceId", deserialize = "deviceId"))]
    pub device_id: Option<String>,
}
