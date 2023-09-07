use actix_web::HttpResponse;
use actix_web::Responder;
use bson::DateTime;
use bson::Document;
use chrono::Utc;

use crate::infra::database;
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
    pub async fn create_fitbit_data(username: &str, data: Document) -> Result<impl Responder, Box<dyn std::error::Error>> {
        Ok(HttpResponse::Created().finish())
    }
}