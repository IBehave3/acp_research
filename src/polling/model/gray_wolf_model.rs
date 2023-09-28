
use chrono::{NaiveDateTime, DateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

// NOTE: base types -------------------------------
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::gray_wolfs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GrayWolf {
    pub id: i32,
    pub userid: i32,
    pub deviceid: String,
    pub version: f64,
    pub generator: String,
    pub api: String,
    pub error: String,
    pub battery: String,
    pub status: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: NaiveDateTime
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::gray_wolf_sensors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GrayWolfSensors {
    pub id: i32,
    #[serde(rename = "grayWolfsId")]
    pub graywolfsid: i32,
    pub sensor: String,
    pub unit: String,
    pub value: f64,
    #[serde(rename = "sensorId")]
    pub sensorid: i32,
    pub status: String
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::gray_wolfs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateGrayWolf {
    pub userid: i32,
    pub deviceid: String,
    pub version: f64,
    pub generator: String,
    pub api: String,
    pub error: String,
    pub battery: String,
    pub status: String,
    #[serde(rename = "serialNumber")]
    pub serialnumber: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: DateTime<Utc>
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::gray_wolf_sensors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateGrayWolfSensor {
    #[serde(rename = "grayWolfsId")]
    pub graywolfsid: i32,
    pub sensor: String,
    pub unit: String,
    pub value: f64,
    #[serde(rename = "sensorId")]
    pub sensorid: i32,
    pub status: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientGrayWolf {
    pub version: f64,
    pub generator: String,
    pub api: String,
    pub error: String,
    pub battery: String,
    pub status: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: DateTime<Utc>,
    pub data: Vec<ClientGrayWolfSensor>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientGrayWolfSensor {
    pub sensor: String,
    pub unit: String,
    pub value: f64,
    pub id: i32,
    pub status: String,
}
