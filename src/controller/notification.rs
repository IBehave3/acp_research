/*use crate::infra::{collection, collection::BaseCollection, database};
use crate::model::notification::Notification;
use actix_web::Result;
use mongodb::{bson::doc, bson::oid::ObjectId, Collection};

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
}*/
