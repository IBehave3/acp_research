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
    #[serde(rename(serialize = "username", deserialize = "username"))]
    pub username: String,
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
    #[serde(rename(serialize = "age", deserialize = "age"))]
    pub age: Option<u32>,
    #[serde(rename(serialize = "gender", deserialize = "gender"))]
    pub gender: Option<String>,
    #[serde(rename(serialize = "race", deserialize = "race"))]
    pub race: Option<String>,
    #[serde(rename(serialize = "birthLocation", deserialize = "birthLocation"))]
    pub birth_location: Option<String>,
    #[serde(rename(serialize = "homeOriginalLocation", deserialize = "homeOriginalLocation"))]
    pub home_original_location: Option<String>,
    #[serde(rename(serialize = "homeLastFiveYearsLocation", deserialize = "homeLastFiveYearsLocation"))]
    pub home_last_five_years_location: Option<String>,
    #[serde(rename(serialize = "employmentStatus", deserialize = "employmentStatus"))]
    pub employment_status: Option<String>,
    #[serde(rename(serialize = "levelOfEducation", deserialize = "levelOfEducation"))]
    pub level_of_education: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateIdMapping {
    #[serde(rename(serialize = "username", deserialize = "username"))]
    pub username: String,
    #[serde(rename(serialize = "password", deserialize = "password"))]
    pub password: String,
    #[serde(rename(serialize = "airthings", deserialize = "airthings"))]
    pub airthings: Option<AirthingsAuth>,
    #[serde(rename(serialize = "grayWolf", deserialize = "grayWolf"))]
    pub gray_wolf: Option<GrayWolfAuth>,
    #[serde(rename(serialize = "uhooAura", deserialize = "uhooAura"))]
    pub uhoo_aura: Option<UhooAuraAuth>,
    #[serde(rename(serialize = "age", deserialize = "age"))]
    pub age: Option<u32>,
    #[serde(rename(serialize = "gender", deserialize = "gender"))]
    pub gender: Option<String>,
    #[serde(rename(serialize = "race", deserialize = "race"))]
    pub race: Option<String>,
    #[serde(rename(serialize = "birthLocation", deserialize = "birthLocation"))]
    pub birth_location: Option<String>,
    #[serde(rename(serialize = "homeOriginalLocation", deserialize = "homeOriginalLocation"))]
    pub home_original_location: Option<String>,
    #[serde(rename(serialize = "homeLastFiveYearsLocation", deserialize = "homeLastFiveYearsLocation"))]
    pub home_last_five_years_location: Option<String>,
    #[serde(rename(serialize = "employmentStatus", deserialize = "employmentStatus"))]
    pub employment_status: Option<String>,
    #[serde(rename(serialize = "levelOfEducation", deserialize = "levelOfEducation"))]
    pub level_of_education: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginIdMapping {
    #[serde(rename(serialize = "username", deserialize = "username"))]
    pub username: String,
    #[serde(rename(serialize = "password", deserialize = "password"))]
    pub password: String,
}