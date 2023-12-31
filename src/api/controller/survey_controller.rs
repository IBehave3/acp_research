use std::sync::Arc;

use actix_web::{HttpResponse, Responder};

use diesel_async::RunQueryDsl;
use log::error;

use crate::api::infra::api_error::ApiError;
use crate::api::infra::database::DbPool;
use crate::api::infra::jwt_middleware::AuthenticatedClaims;
use crate::api::model::survey_model::CreateHourlySurvey;
use crate::api::model::survey_model::{
    ClientCreateDailySurvey, ClientCreateHourlySurvey, CreateDailySurvey,
};
use crate::schema::daily_surveys::dsl::daily_surveys;
use crate::schema::hourly_surveys::dsl::hourly_surveys;


pub async fn create_hourly_survey(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_hourly_survey: ClientCreateHourlySurvey,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let user_id = authenticated_claims.user_id;

    diesel::insert_into(hourly_surveys)
        .values(CreateHourlySurvey {
            userid: user_id,
            currentstress: client_hourly_survey.current_stress,
            location: client_hourly_survey.location,
            timestamp: client_hourly_survey.timestamp,
        })
        .execute(database_connection)
        .await
        .map_err(|err| {
            error!("{}", err);
            ApiError::DbError {
                message: "create_hourly_survey failed".to_owned(),
            }
        })?;

    Ok(HttpResponse::Created().finish())
}

pub async fn create_daily_survey(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_daily_survey: ClientCreateDailySurvey,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let user_id = authenticated_claims.user_id;

    diesel::insert_into(daily_surveys)
        .values(CreateDailySurvey {
            userid: user_id,
            unabletocontrolimportantthings: client_daily_survey.unable_to_control_important_things,
            oftenfeltconfidenthandlepersonalproblems: client_daily_survey
                .often_felt_confident_handle_personal_problems,
            feelthingsaregoingmyway: client_daily_survey.feel_things_are_going_my_way,
            feeldifficultiespilingcannotovercome: client_daily_survey
                .feel_difficulties_piling_cannot_overcome,
            stressyourhealth: client_daily_survey.stress_your_health,
            stressyourfinances: client_daily_survey.stress_your_finances,
            stressfamilysocialrelationships: client_daily_survey.stress_family_social_relationships,
            stressyourword: client_daily_survey.stress_your_word,
        })
        .execute(database_connection)
        .await
        .map_err(|err| {
            error!("{err}");
            ApiError::DbError {
                message: "create_daily_survey failed".to_owned(),
            }
        })?;

    Ok(HttpResponse::Created().finish())
}
