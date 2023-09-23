use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

// NOTE: base types -------------------------------
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::uhoo_homes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UhooHome {
    pub id: i32,
    pub userid: i32,
    pub deviceid: String,
    pub virusindex: i32,
    pub temperature: f64,
    pub humidity: f64,
    pub pm25: i32,
    pub tvoc: i32,
    pub co2: i32,
    pub co: i32,
    pub airpressure: f64,
    pub ozone: i32,
    pub no2: i32,
    pub timestamp: i32,

    pub temperatureunit: String,
    pub tempunit: String,
    pub humidityunit: String,
    pub pm25unit: String,
    pub dustunit: String,
    pub tvocunit: String,
    pub vocunit: String,
    pub co2unit: String,
    pub counit: String,
    pub airpressureunit: String,
    pub pressureunit: String,
    pub ozoneunit: String,
    pub no2unit: String,
    pub pm1unit: String,
    pub pm4unit: String,
    pub pm10unit: String,
    pub ch2ounit: String,
    pub lightunit: String,
    pub h2sunit: String,
    pub nounit: String,
    pub so2unit: String,
    pub nh3unit: String,
    pub oxygenunit: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::uhoo_homes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUhooHome {
    pub userid: i32,
    pub deviceid: String,
    pub virusindex: i32,
    pub temperature: f64,
    pub humidity: f64,
    pub pm25: i32,
    pub tvoc: i32,
    pub co2: i32,
    pub co: i32,
    pub airpressure: f64,
    pub ozone: i32,
    pub no2: i32,
    pub timestamp: i32,

    pub temperatureunit: String,
    pub tempunit: String,
    pub humidityunit: String,
    pub pm25unit: String,
    pub dustunit: String,
    pub tvocunit: String,
    pub vocunit: String,
    pub co2unit: String,
    pub counit: String,
    pub airpressureunit: String,
    pub pressureunit: String,
    pub ozoneunit: String,
    pub no2unit: String,
    pub pm1unit: String,
    pub pm4unit: String,
    pub pm10unit: String,
    pub ch2ounit: String,
    pub lightunit: String,
    pub h2sunit: String,
    pub nounit: String,
    pub so2unit: String,
    pub nh3unit: String,
    pub oxygenunit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientUhooHome {
    pub data: Vec<ClientUhooHomeData>,
    pub usersettings: ClientUhooHomeUserSettings,
    pub sensorlist: Vec<String>,
    pub count: i32,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientUhooHomeData {
    #[serde(rename = "virusIndex")]
    pub virusindex: i32,
    pub temperature: f64,
    pub humidity: f64,
    pub pm25: i32,
    pub tvoc: i32,
    pub co2: i32,
    pub co: i32,
    #[serde(rename = "airPressure")]
    pub airpressure: f64,
    pub ozone: i32,
    pub no2: i32,
    pub timestamp: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientUhooHomeUserSettings {
    pub temperature: String,
    pub temp: String,
    pub humidity: String,
    pub pm25: String,
    pub dust: String,
    pub tvoc: String,
    pub voc: String,
    pub co2: String,
    pub co: String,
    #[serde(rename = "airPressure")]
    pub airpressure: String,
    pub pressure: String,
    pub ozone: String,
    pub no2: String,
    pub pm1: String,
    pub pm4: String,
    pub pm10: String,
    pub ch2o: String,
    pub light: String,
    pub h2s: String,
    pub no: String,
    pub so2: String,
    pub nh3: String,
    pub oxygen: String,
}
