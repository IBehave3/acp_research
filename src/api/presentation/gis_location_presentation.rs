use crate::api::model::gis_location_model::ClientCreateGisLocation;
use crate::{api::controller::gis_location_controller, api::infra::jwt_middleware::AuthenticatedClaims};
use crate::api::infra::database::DbPool;
use actix_web::{
    post,
    web::{self, Data, Json},
    Responder, Result,
};

#[post("")]
pub async fn create_gis_location_post_presentation(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    client_create_gis_location: Json<ClientCreateGisLocation>,
) -> Result<impl Responder> {
    Ok(gis_location_controller::create_gis_location(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        client_create_gis_location.into_inner(),
    ).await)
}
