use serde::{Deserialize, Serialize};
use mongodb::bson::Document;

#[derive(Deserialize)]
pub struct SocketQuery {
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct SocketRequest {
    pub socket_query: SocketQuery,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FitbitMessage {
    data: Document,
}