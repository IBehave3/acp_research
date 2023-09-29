use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::user_locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUserLocation {
    pub userid: i32,

    pub timestamp: i32,
    pub location: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClientCreateUserLocation {
    pub timestamp: i32,
    pub location: String,
}