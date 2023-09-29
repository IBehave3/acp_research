use std::sync::Arc;

use actix_web::{Responder, HttpResponse};
use diesel_async::RunQueryDsl;
use log::error;

use crate::api::infra::api_error::ApiError;
use crate::api::infra::database::DbPool;
use crate::api::infra::jwt_middleware::AuthenticatedClaims;
use crate::api::model::user_location_model::{CreateUserLocation, ClientCreateUserLocation};
use crate::schema::user_locations::dsl::user_locations;


pub async fn create_user_location(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_create_user_location: ClientCreateUserLocation,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let user_id = authenticated_claims.user_id;

    diesel::insert_into(user_locations)
        .values(CreateUserLocation {
            userid: user_id,
            timestamp: client_create_user_location.timestamp,
            location: client_create_user_location.location.to_string(),
        })
        .execute(database_connection)
        .await
        .map_err(|err| {
            error!("{}", err);
            ApiError::DbError {
                message: "create_user_location failed".to_owned(),
            }
        })?;

    Ok(HttpResponse::Created().finish())
}
