use crate::infra::{collection, collection::BaseCollection, database};
use actix_web::Result;
use mongodb::{bson, bson::doc, bson::oid::ObjectId, Collection};
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

impl BaseCollection for Notification {
    type DocumentType = Notification;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::NOTIFICATION_COLLECTION_NAME)
    }
}

impl Notification {
    pub async fn get_notification_by_id_mapping_id(
        id_mapping_id: ObjectId,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let filter = doc! { "idMappingRefId": id_mapping_id };
        let results = Notification::get_all_options(Some(filter), None).await?;

        Ok(results)
    }
}
