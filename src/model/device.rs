use super::auth::UserIdQueryExtractor;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeviceIdQueryExtractor {
    #[serde(flatten)]
    pub user_id_query_extractor: UserIdQueryExtractor,
    #[serde(rename(serialize = "deviceId", deserialize = "deviceId"))]
    pub device_id: String,
}
