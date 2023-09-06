use std::collections::HashSet;

use chrono::NaiveDateTime;
use diesel::{
    prelude::{Associations, Identifiable, Insertable, Queryable},
    query_builder::AsChangeset,
    Selectable,
};
use serde::{Deserialize, Serialize};

// NOTE: base types -------------------------------
#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub createdat: NaiveDateTime,
    pub username: String,
    pub passwordhash: String,
    pub salt: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, PartialEq,
)]
#[diesel(belongs_to(User, foreign_key = userid))]
#[diesel(table_name = crate::schema::user_airthings)]
pub struct UserAirthings {
    pub id: i32,
    pub userid: i32,
    pub clientsecret: String,
    pub clientid: String,
    pub groupid: String,
    pub deviceids: Option<Vec<Option<String>>>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, PartialEq,
)]
#[diesel(belongs_to(User, foreign_key = userid))]
#[diesel(table_name = crate::schema::user_gray_wolfs)]
pub struct UserGrayWolfs {
    pub id: i32,
    pub userid: i32,
    pub apikey: String,
    pub deviceids: Option<Vec<Option<String>>>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, PartialEq,
)]
#[diesel(belongs_to(User, foreign_key = userid))]
#[diesel(table_name = crate::schema::user_uhoo_auras)]
pub struct UserUhooAuras {
    pub id: i32,
    pub userid: i32,
    pub clientsecret: String,
    pub deviceids: Option<Vec<Option<String>>>,
}

// NOTE: client types -------------------------------
#[derive(Serialize, Deserialize)]
pub struct ClientLoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientUpdateUserAirthings {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "deviceIds")]
    pub device_ids: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientUpdateUserGrayWolf {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "deviceIds")]
    pub device_ids: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientUpdateUserUhooAura {
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "deviceIds")]
    pub device_ids: HashSet<String>, 
}

#[derive(Serialize, Deserialize)]
pub struct ClientGetUserInformation {
    #[serde(flatten)]
    pub user: User,
    pub airthings: Option<UserAirthings>,
    pub gray_wolf: Option<UserGrayWolfs>,
    pub uhoo_aura: Option<UserUhooAuras>
}

// NOTE: insert types -------------------------------
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUser {
    pub username: String,
    pub passwordhash: String,
    pub salt: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_airthings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUserAirthings {
    pub userid: i32,
    pub clientid: String,
    pub clientsecret: String,
    pub groupid: String,
    pub deviceids: Vec<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_gray_wolfs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUserGrayWolf {
    pub userid: i32,
    pub apikey: String,
    pub deviceids: Vec<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_uhoo_auras)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUserUhooAura {
    pub userid: i32,
    pub clientsecret: String,
    pub deviceids: Vec<String>,
}

// NOTE: update types -------------------------------
#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_airthings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUserAirthings {
    pub clientid: String,
    pub clientsecret: String,
    pub groupid: String,
    pub deviceids: Vec<String>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_gray_wolfs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUserGrayWolf {
    pub apikey: String,
    pub deviceids: Vec<String>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_uhoo_auras)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUserUhooAura {
    pub clientsecret: String,
    pub deviceids: Vec<String>,
}