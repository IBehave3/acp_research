use crate::infra::database;
use crate::model::uhoo_aura::UhooAura;
use crate::infra::collection::BaseCollection;
use crate::infra::collection::UHOO_AURA_COLLECTION_NAME;

impl BaseCollection for UhooAura {
    type DocumentType = Self;

    fn get_collection() -> mongodb::Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(UHOO_AURA_COLLECTION_NAME)
    }
}