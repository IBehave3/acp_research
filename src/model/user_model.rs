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

    pub age: i32,
    pub race: Option<Vec<Option<String>>>,
    pub otherrace: Option<String>,
    pub gender: Option<String>,
    pub othergender: Option<String>,
    pub employed: bool,
    pub levelofeducation: String,

    pub unabletocontrolimportantthings: i32,
    pub oftenfeltconfidenthandlepersonalproblems: i32,
    pub feltthingsgoyourway: i32,
    pub feltdifficultiespilingup: i32,

    pub bouncebackquickly: i32,
    pub hardtimestressfullevents: i32,
    pub longrecoverytime: i32,
    pub hardtosnapback: i32,
    pub comethroughdifficulttimes: i32,
    pub longtimegetoversetbacks: i32,
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
pub struct UserGrayWolf {
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
pub struct UserUhooAura {
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
    pub resilience: ClientCreateUserResilience,
    pub demographic: ClientCreateUserDemographic,
    pub stress: ClientCreateUserStress,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateUserResilience {
    #[serde(rename = "bounceBackQuickly")]
    pub bounce_back_quickly: i32,
    #[serde(rename = "hardTimeStressfullEvents")]
    pub hard_time_stressfull_events: i32,
    #[serde(rename = "longRecoveryTime")]
    pub long_recovery_time: i32,
    #[serde(rename = "hardToSnapBack")]
    pub hard_to_snap_back: i32,
    #[serde(rename = "comeThroughDifficulTimes")]
    pub come_through_difficult_times: i32,
    #[serde(rename = "longTimeGetOverSetBacks")]
    pub long_time_get_over_set_backs: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateUserDemographic {
    pub age: i32,
    pub race: Option<Vec<Option<String>>>,
    #[serde(rename = "otherRace")]
    pub other_race: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "otherGender")]
    pub other_gender: Option<String>,
    pub employed: bool,
    #[serde(rename = "levelOfEducation")]
    pub level_of_education: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateUserStress {
    #[serde(rename = "unableToControlImportanThings")]
    pub unable_to_control_important_things: i32,
    #[serde(rename = "oftenFeltConfidentHandlePersonalProblems")]
    pub often_felt_confident_handle_personal_problems: i32,
    #[serde(rename = "feltThingsGoYourWay")]
    pub felt_things_go_your_way: i32,
    #[serde(rename = "feltDifficultiesPilingUp")]
    pub felt_difficulties_piling_up: i32,
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
    pub gray_wolf: Option<UserGrayWolf>,
    pub uhoo_aura: Option<UserUhooAura>,
}

// NOTE: insert types -------------------------------
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUser {
    pub username: String,
    pub passwordhash: String,
    pub salt: String,

    pub age: i32,
    pub race: Option<Vec<Option<String>>>,
    pub otherrace: Option<String>,
    pub gender: Option<String>,
    pub othergender: Option<String>,
    pub employed: bool,
    pub levelofeducation: String,

    pub unabletocontrolimportantthings: i32,
    pub oftenfeltconfidenthandlepersonalproblems: i32,
    pub feltthingsgoyourway: i32,
    pub feltdifficultiespilingup: i32,

    pub bouncebackquickly: i32,
    pub hardtimestressfullevents: i32,
    pub longrecoverytime: i32,
    pub hardtosnapback: i32,
    pub comethroughdifficulttimes: i32,
    pub longtimegetoversetbacks: i32,
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
