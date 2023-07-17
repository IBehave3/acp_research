use mongodb::{bson, bson::doc, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserNotification {
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub timestamp: bson::DateTime,
    pub message: String,
    #[serde(rename(serialize = "statusCode", deserialize = "statusCode"))]
    pub status_code: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Notification {
    pub _id: ObjectId,
    #[serde(rename(serialize = "idMappingRefId", deserialize = "idMappingRefId"))]
    pub id_mapping_ref_id: ObjectId,
    #[serde(flatten)]
    pub user_notification: UserNotification,
}
