use std::collections::HashSet;
use bson::oid::ObjectId;
use mongodb::{bson::doc, bson::Document};
use serde::{Deserialize, Serialize};
use serde;
use bson::DateTime;

#[derive(Serialize, Deserialize, Debug)]

pub struct UhooAura {
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(serialize = "userRefId", deserialize = "userRefId"))]
    pub user_ref_id: ObjectId,
    #[serde(flatten, rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub data: Document,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UhooAuraAuth {
    #[serde(rename(serialize = "clientSecret", deserialize = "clientSecret"))]
    pub client_secret: String,
    #[serde(rename(serialize = "deviceIds", deserialize = "deviceIds"))]
    pub device_ids: HashSet<String>,
}