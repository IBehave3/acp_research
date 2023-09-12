use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Responder;
use diesel_async::RunQueryDsl;
use log::error;

use crate::infra::api_error::ApiError;
use crate::infra::database::DbPool;
use crate::infra::jwt_middleware::AuthenticatedClaims;
use crate::model::fitbit_model::ClientCreateFitbit;

use crate::model::fitbit_model::CreateFitbitAccelerometer;
use crate::model::fitbit_model::CreateFitbitBarometer;
use crate::model::fitbit_model::CreateFitbitGryoscope;
use crate::model::fitbit_model::CreateFitbitHeartRate;
use crate::model::fitbit_model::CreateFitbitOrientation;

use crate::schema::fitbit_heartrates::dsl::fitbit_heartrates;
use crate::schema::fitbit_accelerometers::dsl::fitbit_accelerometers;
use crate::schema::fitbit_barometers::dsl::fitbit_barometers;
use crate::schema::fitbit_gyroscopes::dsl::fitbit_gyroscopes;
use crate::schema::fitbit_orientations::dsl::fitbit_orientations;

pub async fn create_fitbit(pool: Arc<DbPool>, authenticated_claims: AuthenticatedClaims, client_fitbit: ClientCreateFitbit) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let user_id = authenticated_claims.user_id;

    // NOTE: inserting heartrates
    for heart_rate in client_fitbit.heart_rate {
        if let Err(err) = diesel::insert_into(fitbit_heartrates)
        .values(CreateFitbitHeartRate {
            timestamp: heart_rate.timestamp_iso,
            userid: user_id,
            heartrate: heart_rate.heart_rate,
        })
        .execute(database_connection)
        .await {
            error!("{err}");
            return Err(ApiError::DbError {
                message: "create_fitbit failed".to_string(),
            }
            .into());
        };
    }

    // NOTE: inserting accelerometers
    for accelerometer in client_fitbit.accelerometer {
        if let Err(err) = diesel::insert_into(fitbit_accelerometers)
        .values(CreateFitbitAccelerometer {
            timestamp: accelerometer.timestamp_iso,
            userid: user_id,
            x: accelerometer.x,
            y: accelerometer.y,
            z: accelerometer.z,
        })
        .execute(database_connection)
        .await {
            error!("{err}");
            return Err(ApiError::DbError {
                message: "create_fitbit failed".to_string(),
            }
            .into());
        };
    }

    // NOTE: inserting barometers
    for barometer in client_fitbit.barometer {
        if let Err(err) = diesel::insert_into(fitbit_barometers)
        .values(CreateFitbitBarometer {
            timestamp: barometer.timestamp_iso,
            userid: user_id,
            pressure: barometer.pressure,
        })
        .execute(database_connection)
        .await {
            error!("{err}");
            return Err(ApiError::DbError {
                message: "create_fitbit failed".to_string(),
            }
            .into());
        };
    }

    // NOTE: inserting gyroscopes
    for gyroscope in client_fitbit.gyroscope {
        if let Err(err) = diesel::insert_into(fitbit_gyroscopes)
        .values(CreateFitbitGryoscope {
            timestamp: gyroscope.timestamp_iso,
            userid: user_id,
            x: gyroscope.x,
            y: gyroscope.y,
            z: gyroscope.z,
        })
        .execute(database_connection)
        .await {
            error!("{err}");
            return Err(ApiError::DbError {
                message: "create_fitbit failed".to_string(),
            }
            .into());
        };
    }

    // NOTE: inserting orientations
    for orientation in client_fitbit.orientation {
        if let Err(err) = diesel::insert_into(fitbit_orientations)
        .values(CreateFitbitOrientation {
            timestamp: orientation.timestamp_iso,
            userid: user_id,
            x: orientation.x,
            y: orientation.y,
            z: orientation.z,
            scalar: orientation.scalar,
        })
        .execute(database_connection)
        .await {
            error!("{err}");
            return Err(ApiError::DbError {
                message: "create_fitbit failed".to_string(),
            }
            .into());
        };
    }

    Ok(HttpResponse::Created().finish())
}