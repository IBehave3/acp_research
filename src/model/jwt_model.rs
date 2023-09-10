use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtToken {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtCustomClaims {
    pub user_id: i32,
    pub username: String,
}