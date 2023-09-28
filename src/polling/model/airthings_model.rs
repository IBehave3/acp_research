use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

// NOTE: base types -------------------------------
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::airthings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Airthings {
    pub id: i32,
    pub userid: i32,
    pub deviceid: String,
    pub battery: Option<f64>,
    pub co2: Option<f64>,
    pub humidity: Option<f64>,
    pub pm1: Option<f64>,
    pub pm25: Option<f64>,
    pub pressure: Option<f64>,
    pub radonshorttermavg: Option<f64>,
    pub temp: Option<f64>,
    pub time: Option<i32>,
    pub voc: Option<f64>,
    pub relaydevicetype: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::airthings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateAirthings {
    pub userid: i32,
    pub deviceid: String,
    pub battery: Option<f64>,
    pub co2: Option<f64>,
    pub humidity: Option<f64>,
    pub pm1: Option<f64>,
    pub pm25: Option<f64>,
    pub pressure: Option<f64>,
    pub radonshorttermavg: Option<f64>,
    pub temp: Option<f64>,
    pub time: Option<i32>,
    pub voc: Option<f64>,
    pub relaydevicetype: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientAirthings {
    pub data: ClientCoreAirthings
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCoreAirthings {
    pub battery: Option<f64>,
    pub co2: Option<f64>,
    pub humidity: Option<f64>,
    pub pm1: Option<f64>,
    pub pm25: Option<f64>,
    pub pressure: Option<f64>,
    #[serde(rename = "radonShortTermAvg")]
    pub radonshorttermavg: Option<f64>,
    pub temp: Option<f64>,
    pub time: Option<i32>,
    pub voc: Option<f64>,
    #[serde(rename = "relayDeviceType")]
    pub relaydevicetype: Option<String>,
}
