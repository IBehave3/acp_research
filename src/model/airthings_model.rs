use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

// NOTE: base types -------------------------------
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::airthings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Airthings {
    pub id: i32,
    pub userid: i32,
    pub battery: f64,
    pub co2: f64,
    pub humidity: f64,
    pub pm1: f64,
    pub pm25: f64,
    pub pressure: f64,
    pub radonshorttermavg: f64,
    pub temp: f64,
    pub time: i32,
    pub voc: f64,
    pub relaydevicetype: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::airthings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateAirthings {
    pub userid: i32,
    pub battery: f64,
    pub co2: f64,
    pub humidity: f64,
    pub pm1: f64,
    pub pm25: f64,
    pub pressure: f64,
    pub radonshorttermavg: f64,
    pub temp: f64,
    pub time: i32,
    pub voc: f64,
    pub relaydevicetype: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientAirthings {
    pub data: ClientCoreAirthings
}

#[derive(Serialize, Deserialize)]
pub struct ClientCoreAirthings {
    pub battery: f64,
    pub co2: f64,
    pub humidity: f64,
    pub pm1: f64,
    pub pm25: f64,
    pub pressure: f64,
    #[serde(rename = "radonShortTermAvg")]
    pub radonshorttermavg: f64,
    pub temp: f64,
    pub time: i32,
    pub voc: f64,
    #[serde(rename = "relayDeviceType")]
    pub relaydevicetype: String,
}
