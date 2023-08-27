/*use crate::infra::{collection, collection::BaseCollection, database};
use crate::model::push_data::PushData;

impl BaseCollection for PushData {
    type DocumentType = PushData;

    fn get_collection() -> mongodb::Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::PUSH_DATA_COLLECTION_NAME)
    }
}*/
