use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtToken {
    #[serde(rename(serialize = "token", deserialize = "token"))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaims {
    #[serde(rename(serialize = "email", deserialize = "email"))]
    pub email: String,
}