use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientFitbit {
    #[serde(rename = "heartRate")]
    pub heart_rate: Vec<ClientFitbitHeartRate>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitHeartRate {
    #[serde(rename = "timestampISO")]
    pub time_stamp_iso: String,
    #[serde(rename = "heartRate")]
    pub heart_rate: i32,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitGryoscope{
    #[serde(rename = "timestampISO")]
    pub time_stamp_iso: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitOrientation {
    #[serde(rename = "timestampISO")]
    pub time_stamp_iso: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitBarometer {
    #[serde(rename = "timestampISO")]
    pub time_stamp_iso: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ClientFitbitAccelerometer {
    #[serde(rename = "timestampISO")]
    pub time_stamp_iso: String,
}