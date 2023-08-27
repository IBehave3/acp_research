use bson::DateTime;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::model::airthings::AirthingsAuth;
use crate::model::gray_wolf::GrayWolfAuth;
use crate::model::uhoo_aura::UhooAuraAuth;

#[derive(Serialize, Deserialize, Debug)]
pub struct IdMapping {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: ObjectId,
    #[serde(rename(serialize = "email", deserialize = "email"))]
    pub email: String,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(serialize = "passwordHash", deserialize = "passwordHash"))]
    pub password_hash: String,
    #[serde(rename(serialize = "salt", deserialize = "salt"))]
    pub salt: String,
    #[serde(rename(serialize = "airthings", deserialize = "airthings"))]
    pub airthings: Option<AirthingsAuth>,
    #[serde(rename(serialize = "grayWolf", deserialize = "grayWolf"))]
    pub gray_wolf: Option<GrayWolfAuth>,
    #[serde(rename(serialize = "uhooAura", deserialize = "uhooAura"))]
    pub uhoo_aura: Option<UhooAuraAuth>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateIdMapping {
    #[serde(rename(serialize = "email", deserialize = "email"))]
    pub email: String,
    #[serde(rename(serialize = "password", deserialize = "password"))]
    pub password: String,
    #[serde(rename(serialize = "airthings", deserialize = "airthings"))]
    pub airthings: Option<AirthingsAuth>,
    #[serde(rename(serialize = "grayWolf", deserialize = "grayWolf"))]
    pub gray_wolf: Option<GrayWolfAuth>,
    #[serde(rename(serialize = "uhooAura", deserialize = "uhooAura"))]
    pub uhoo_aura: Option<UhooAuraAuth>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginIdMapping {
    #[serde(rename(serialize = "email", deserialize = "email"))]
    pub email: String,
    #[serde(rename(serialize = "password", deserialize = "password"))]
    pub password: String,
}