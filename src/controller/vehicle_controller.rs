use std::sync::Arc;

use actix_web::{Responder, HttpResponse};
use diesel_async::RunQueryDsl;
use log::error;

use crate::infra::api_error::{ApiError, self};
use crate::infra::database::DbPool;
use crate::infra::jwt_middleware::AuthenticatedClaims;
use crate::model::vehicle_controller_model::{CreateVehicleMeasurement, ClientCreateVehicleMeasurement};
use crate::schema::vehicle_measurements::dsl::vehicle_measurements;
use diesel::result::{DatabaseErrorKind, Error as diesel_error};

pub async fn create_vehicle_measurement(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_create_vehicle_measurement: ClientCreateVehicleMeasurement,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    match diesel::insert_into(vehicle_measurements)
        .values(CreateVehicleMeasurement {
            userid: authenticated_claims.user_id,
            date: client_create_vehicle_measurement.date,
            time: client_create_vehicle_measurement.time,
            timestamp_iso8601: client_create_vehicle_measurement.timestamp_iso8601,
            speed: client_create_vehicle_measurement.speed,
            steeringangle: client_create_vehicle_measurement.steering_angle,
            distance: client_create_vehicle_measurement.distance,
            velocity: client_create_vehicle_measurement.velocity,
            accelerationpressure: client_create_vehicle_measurement.acceleration_pressure,
            brakepressure: client_create_vehicle_measurement.brake_pressure,
            lane: client_create_vehicle_measurement.lane,
            scenarionumber: client_create_vehicle_measurement.scenario_number,
        })
        .execute(database_connection)
        .await
    {
        Ok(_) => (),
        Err(err) => {
            error!("{err}");

            if let diesel_error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                return Ok(HttpResponse::Conflict().finish());
            }

            return Err(api_error::ApiError::DbError {
                message: "create_vehicle_measurement failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Created().finish())
}
