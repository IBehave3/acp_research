use crate::infra::database;
use crate::model::airthings::Airthings;
use crate::infra::collection::BaseCollection;
use crate::infra::collection::AIRTHINGS_COLLECTION_NAME;

impl BaseCollection for Airthings {
    type DocumentType = Self;

    fn get_collection() -> mongodb::Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(AIRTHINGS_COLLECTION_NAME)
    }
}