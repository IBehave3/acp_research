use crate::infra::collection::BaseCollection;
use crate::infra::{collection, database};
use mongodb::bson::Document;
use mongodb::Collection;

struct Test {}

impl BaseCollection for Test {
    type DocumentType = Document;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::TEST_COLLECTION_NAME)
    }
}
