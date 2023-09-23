use chrono::{NaiveDateTime, DateTime, Utc};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

// NOTE: base types -------------------------------
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::keychains)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Keychain {
    pub id: i32,
    pub userid: i32,
    pub time: DateTime<Utc>,
    pub devmac: i32,
    pub voc: Option<f64>,
    pub pm1: Option<f64>,
    pub pm25: Option<f64>,
    pub pm10: Option<f64>,
    pub t: Option<f64>,
    pub h: Option<f64>,
    pub p: Option<f64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::keychains)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateKeychain {
    pub userid: i32,
    pub time: DateTime<Utc>,
    pub devmac: String,
    pub voc: Option<f64>,
    pub pm1: Option<f64>,
    pub pm25: Option<f64>,
    pub pm10: Option<f64>,
    pub t: Option<f64>,
    pub h: Option<f64>,
    pub p: Option<f64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientKeychain {
    pub status: i32,
    pub data: ClientCoreKeychain
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCoreKeychain {
    pub total: i32,
    pub items: Vec<ClientCoreItem>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCoreItem {
    pub time: String,
    pub voc: Option<f64>,
    pub pm1: Option<f64>,
    pub pm25: Option<f64>,
    pub pm10: Option<f64>,
    pub t: Option<f64>,
    pub h: Option<f64>,
    pub p: Option<f64>,
    pub coords: ClientCoreItemCoords,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCoreItemCoords {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}
