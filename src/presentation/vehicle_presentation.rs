use crate::{
    controller::vehicle_controller,
    infra::{database::DbPool, jwt_middleware::AuthenticatedClaims},
    model::vehicle_controller_model::ClientCreateVehicleMeasurement,
};
use actix_web::{
    post,
    web::{self, Data, Json},
    Responder, Result,
};

#[post("")]
pub async fn create_vehicle_measurement_post_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    client_create_vehicle_measurement: Json<ClientCreateVehicleMeasurement>,
) -> Result<impl Responder> {
    Ok(vehicle_controller::create_vehicle_measurement(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        client_create_vehicle_measurement.into_inner(),
    )
    .await)
}
