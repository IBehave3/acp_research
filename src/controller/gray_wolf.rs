use crate::infra::database;
use crate::model::gray_wolf::GrayWolf;
use crate::infra::collection::BaseCollection;
use crate::infra::collection::GRAY_WOLF_COLLECTION_NAME;

impl BaseCollection for GrayWolf {
    type DocumentType = Self;

    fn get_collection() -> mongodb::Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(GRAY_WOLF_COLLECTION_NAME)
    }
}