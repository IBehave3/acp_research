use super::auth::UserIdQueryExtractor;
use bson::Document;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DataStructureQueryExtractor {
    #[serde(flatten)]
    pub user_id_query_extractor: UserIdQueryExtractor,
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    pub data_structure_id: String,
    pub metadata: Option<Document>,
}
