use chrono::{Utc, DateTime};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

// NOTE: insert models -------------------------------
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::fitbit_heartrates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateFitbitHeartRate {
    pub timestamp: DateTime<Utc>,
    pub userid: i32,
    pub heartrate: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::fitbit_gyroscopes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateFitbitGryoscope{
    pub timestamp: DateTime<Utc>,
    pub userid: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::fitbit_orientations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateFitbitOrientation {
    pub timestamp: DateTime<Utc>,
    pub userid: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub scalar: f64,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::fitbit_barometers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateFitbitBarometer {
    pub timestamp: DateTime<Utc>,
    pub userid: i32,
    pub pressure: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::fitbit_accelerometers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateFitbitAccelerometer {
    pub timestamp: DateTime<Utc>,
    pub userid: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// NOTE: client models -------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCreateFitbit {
    #[serde(rename = "heartRate")]
    pub heart_rate: Vec<ClientFitbitHeartRate>,
    pub accelerometer: Vec<ClientFitbitAccelerometer>,
    pub barometer: Vec<ClientFitbitBarometer>,
    pub gyroscope: Vec<ClientFitbitGryoscope>,
    pub orientation: Vec<ClientFitbitOrientation>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitHeartRate {
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: DateTime<Utc>,
    #[serde(rename = "heartRate")]
    pub heart_rate: i32,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitGryoscope{
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitOrientation {
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub scalar: f64
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitBarometer {
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: DateTime<Utc>,
    pub pressure: i32
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitAccelerometer {
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}