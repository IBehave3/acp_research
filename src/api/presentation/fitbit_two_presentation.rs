use actix_web::{get, Responder, Result, web::{Data, self}};

use crate::api::{controller::fitbit_two_controller, infra::database::DbPool, model::fitbit_two_model::FitbitTwoQueryParameters};

#[get("/validate")]
pub async fn get_fitbit_two_get_handler(
    pool: Data<DbPool>,
    fitbit_two_query_parameters: web::Query<FitbitTwoQueryParameters>,
) -> Result<impl Responder> {
    fitbit_two_controller::verify_fitbit_two(pool.into_inner(), fitbit_two_query_parameters.into_inner().verify).await
}
