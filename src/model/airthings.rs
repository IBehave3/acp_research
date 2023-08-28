use std::collections::HashSet;
use bson::oid::ObjectId;
use mongodb::{bson::doc, bson::Document};
use serde::{Deserialize, Serialize};
use serde;
use bson::DateTime;


#[derive(Serialize, Deserialize, Debug)]
pub struct Airthings {
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(serialize = "userRefId", deserialize = "userRefId"))]
    pub user_ref_id: ObjectId,
    #[serde(flatten)]
    pub data: Document,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirthingsAuth {
    #[serde(rename(serialize = "clientSecret", deserialize = "clientSecret"))]
    pub client_secret: String,
    #[serde(rename(serialize = "clientId", deserialize = "clientId"))]
    pub client_id: String,
    #[serde(rename(serialize = "deviceIds", deserialize = "deviceIds"))]
    pub device_ids: HashSet<String>,
    #[serde(rename(serialize = "groupId", deserialize = "groupId"))]
    pub group_id: String,
}