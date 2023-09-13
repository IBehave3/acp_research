use chrono::{Utc, DateTime, NaiveDateTime};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

// NOTE: insert models -------------------------------
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::hourly_surveys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateHourlySurvey {
    pub userid: i32,
    pub thermalsensation: i32,
    pub thermalacceptability: i32,
    pub thermalcomfort: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::daily_surveys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateDailySurvey {
    pub userid: i32,

    pub unabletocontrolimportantthings: i32,
    pub oftenfeltconfidenthandlepersonalproblems: i32,
    pub feelthingsaregoingmyway: i32,
    pub feeldifficultiespilingcannotovercome: i32,

    pub stressyourhealth: i32,
    pub stressyourfinances: i32,
    pub stressfamilysocialrelationships: i32,
    pub stressyourword: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateHourlySurvey {
    #[serde(rename = "thermalSensation")]
    pub thermal_sensation: i32,
    #[serde(rename = "thermalAcceptability")]
    pub thermal_acceptability: i32,
    #[serde(rename = "thermalComfort")]
    pub thermal_comfort: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateDailySurvey {
    #[serde(rename = "unableToControlImportantThings")]
    pub unable_to_control_important_things: i32,
    #[serde(rename = "oftenFeltConfidentHandlePersonalProblems")]
    pub often_felt_confident_handle_personal_problems: i32,
    #[serde(rename = "feelThingsAreGoingMyWay")]
    pub feel_things_are_going_my_way: i32,
    #[serde(rename = "feelDifficultiesPilingCannotOvercome")]
    pub feel_difficulties_piling_cannot_overcome: i32,

    #[serde(rename = "stressYourHealth")]
    pub stress_your_health: i32,
    #[serde(rename = "stressYourFinances")]
    pub stress_your_finances: i32,
    #[serde(rename = "stressFamilySocialRelationships")]
    pub stress_family_social_relationships: i32,
    #[serde(rename = "stressYourWord")]
    pub stress_your_word: i32,
}