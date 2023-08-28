use actix_web::HttpResponse;
use actix_web::Responder;
use bson::DateTime;
use bson::Document;
use chrono::Utc;

use crate::infra::database;
use crate::model::auth::IdMapping;
use crate::model::fitbit::Fitbit;
use crate::infra::collection::BaseCollection;
use crate::infra::collection::FITBIT_COLLECTION_NAME;

impl BaseCollection for Fitbit {
    type DocumentType = Self;

    fn get_collection() -> mongodb::Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(FITBIT_COLLECTION_NAME)
    }
}

impl Fitbit {
    pub async fn create_fitbit_data(email: &str, data: Document) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let id_mapping = match IdMapping::get_by_email(email).await? {
            Some(id_mapping) => id_mapping,
            None => {
                return Ok(HttpResponse::NotFound().finish());
            }
        };


        let result = Fitbit::add(Fitbit {
            user_ref_id: id_mapping.id,
            created_at: DateTime::from_chrono(Utc::now()),
            data,
        }).await?;

        Ok(HttpResponse::Created().json(result))
    }
}