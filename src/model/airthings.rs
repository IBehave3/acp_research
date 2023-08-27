use std::collections::HashSet;
use bson::oid::ObjectId;
use mongodb::{bson::doc, bson::Document};
use serde::{Deserialize, Serialize};
use serde;
use bson::DateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct AirthingsCoreData {
    #[serde(rename(serialize = "battery", deserialize = "battery"))]
    pub battery: f32,
    #[serde(rename(serialize = "co2", deserialize = "co2"))]
    pub co2: f32,
    #[serde(rename(serialize = "humidity", deserialize = "humidity"))]
    pub humidity: f32,
    #[serde(rename(serialize = "pm1", deserialize = "pm1"))]
    pub pm1: f32,
    #[serde(rename(serialize = "pm25", deserialize = "pm25"))]
    pub pm25: f32,
    #[serde(rename(serialize = "pressure", deserialize = "pressure"))]
    pub pressure: f32,
    #[serde(rename(serialize = "radonShortTermAvg", deserialize = "radonShortTermAvg"))]
    pub radon_short_term_avg: f32,
    #[serde(rename(serialize = "temp", deserialize = "temp"))]
    pub temp: f32,
    #[serde(rename(serialize = "time", deserialize = "time"))]
    pub time: i32,
    #[serde(rename(serialize = "voc", deserialize = "voc"))]
    pub voc: f32,
    #[serde(rename(serialize = "relayDeviceType", deserialize = "relayDeviceType"))]
    pub relay_device_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirthingsData {
    #[serde(rename(serialize = "data", deserialize = "data"))]
    pub data: AirthingsCoreData
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Airthings {
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(serialize = "userRefId", deserialize = "userRefId"))]
    pub user_ref_id: ObjectId,
    #[serde(flatten)]
    pub airthings_data: AirthingsCoreData,
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