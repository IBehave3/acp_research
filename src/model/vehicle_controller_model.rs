use actix_web::cookie::time::Date;
use chrono::{Utc, DateTime};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

// NOTE: insert models -------------------------------
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::vehicle_measurements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateVehicleMeasurement {
    pub userid: i32,
    pub date: Option<String>,
    pub time: Option<String>,
    pub timestamp_iso8601: Option<DateTime<Utc>>,
    pub speed: Option<f64>,
    pub steeringangle: Option<f64>,
    pub distance: Option<f64>,
    pub velocity: Option<f64>,
    pub accelerationpressure: Option<f64>,
    pub brakepressure: Option<f64>,
    pub lane: Option<f64>,
    pub scenarionumber: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateVehicleMeasurement {
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "Time")]
    pub time: Option<String>,
    #[serde(rename = "timestampIso8601")]
    pub timestamp_iso8601: Option<DateTime<Utc>>,
    #[serde(rename = "Speed")]
    pub speed: Option<f64>,
    #[serde(rename = "Steering Angle")]
    pub steering_angle: Option<f64>,
    #[serde(rename = "Distance")]
    pub distance: Option<f64>,
    #[serde(rename = "Velocity")]
    pub velocity: Option<f64>,
    #[serde(rename = "AccelPress")]
    pub acceleration_pressure: Option<f64>,
    #[serde(rename = "BrakePress")]
    pub brake_pressure: Option<f64>,
    #[serde(rename = "Lane")]
    pub lane: Option<f64>,
    #[serde(rename = "ScenarioNum")]
    pub scenario_number: Option<f64>,
}