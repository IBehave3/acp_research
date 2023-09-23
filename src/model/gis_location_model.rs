use diesel::prelude::{Insertable, QueryableByName};
use serde::{Deserialize, Serialize};
use diesel::sql_types::{Integer, Double};

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::gis_locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateGisLocation{
    pub timestamp: i32,
    pub userid: i32,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateGisLocation {
    pub timestamp: i32,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(QueryableByName, Debug)]
pub struct GisLocationRawQueryResult {
    #[sql_type = "Integer"]
    pub location_id: i32,
    #[sql_type = "Integer"]
    pub user_id: i32,
    #[sql_type = "Integer"]
    pub ts: i32,
    #[sql_type = "Double"]
    pub distance_in_feet: f64,
}
