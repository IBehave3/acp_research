use std::sync::Arc;

use actix_web::{Responder, HttpResponse};
use diesel_async::RunQueryDsl;
use log::error;

use crate::infra::api_error::{self, ApiError};
use crate::infra::jwt_middleware::AuthenticatedClaims;
use crate::model::gis_location_model::CreateGisLocation;
use crate::{infra::database::DbPool, model::gis_location_model::ClientCreateGisLocation};
use crate::schema::gis_locations::dsl::gis_locations;
use diesel::result::{DatabaseErrorKind, Error as diesel_error};

pub async fn create_gis_location(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims, 
    client_create_gis_location: ClientCreateGisLocation,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    match diesel::insert_into(gis_locations)
        .values(CreateGisLocation {
            timestamp: client_create_gis_location.timestamp,
            userid: authenticated_claims.user_id,
            longitude: client_create_gis_location.longitude,
            latitude: client_create_gis_location.latitude,
        })
        .execute(database_connection)
        .await
    {
        Ok(blog_id) => blog_id,
        Err(err) => {
            error!("{err}");
            
            if let diesel_error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                return Ok(HttpResponse::Conflict().finish());
            }

            return Err(api_error::ApiError::DbError {
                message: "create_gis_location failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Created().finish())
}