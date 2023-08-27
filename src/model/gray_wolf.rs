use std::collections::HashSet;
use bson::oid::ObjectId;
use mongodb::{bson::doc, bson::Document};
use serde::{Deserialize, Serialize};
use serde;
use bson::DateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct GrayWolf {
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(serialize = "userRefId", deserialize = "userRefId"))]
    pub user_ref_id: ObjectId,
    #[serde(flatten, rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub data: Document,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrayWolfAuth {
    #[serde(rename(serialize = "apiKey", deserialize = "apiKey"))]
    pub api_key: String,
    #[serde(rename(serialize = "deviceIds", deserialize = "deviceIds"))]
    pub device_ids: HashSet<String>,
}