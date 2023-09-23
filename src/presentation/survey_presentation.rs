use crate::{
    controller::{survey_controller},
    infra::{database::DbPool, jwt_middleware::AuthenticatedClaims},
    model::{survey_model::{ClientCreateDailySurvey, ClientCreateHourlySurvey}},
};
use actix_web::{
    post,
    web::{self, Data, Json}, Responder, Result,
};

#[post("/hourly")]
pub async fn create_hourly_survey_post_presentation(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    client_hourly_survey: Json<ClientCreateHourlySurvey>
) -> Result<impl Responder> {
    survey_controller::create_hourly_survey(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        client_hourly_survey.into_inner(),
    ).await
}

#[post("/daily")]
pub async fn create_daily_survey_post_presentation(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    client_daily_survey: Json<ClientCreateDailySurvey>
) -> Result<impl Responder> {
    survey_controller::create_daily_survey(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        client_daily_survey.into_inner(),
    ).await
}
