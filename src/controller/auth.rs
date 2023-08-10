use crate::infra::{collection, collection::BaseCollection, database};
use crate::model::auth::IdMapping;
use actix_web::Result;
use mongodb::{bson::doc, Collection};

impl BaseCollection for IdMapping {
    type DocumentType = IdMapping;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::ID_MAPPING_COLLECTION_NAME)
    }
}

impl IdMapping {
    pub async fn get_airthings_users() -> Result<Vec<IdMapping>, Box<dyn std::error::Error>> {
        let filter = doc! { "dataStructureDeviceIdMapping.dataStructureId": "airthings".to_string() };
        let results = IdMapping::get_all_options(Some(filter), None).await?;

        Ok(results)
    }

    pub async fn get_id_mapping_by_user_id(
        user_id: &str,
    ) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let filter = doc! { "userId": user_id };
        IdMapping::get_options(Some(filter), None).await
    }

    pub async fn id_mapping_exists(user_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let res = Self::get_id_mapping_by_user_id(user_id).await?;
        Ok(res.is_some())
    }
}
