use crate::model::gis_location_model::ClientCreateGisLocation;
use crate::{controller::gis_location_controller, infra::jwt_middleware::AuthenticatedClaims};
use actix_web::{
    post,
    web::{self, Data, Json},
    Responder, Result,
};

#[post("")]
pub async fn create_gis_location_post_presentation(
    pool: Data<crate::infra::database::DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    client_create_gis_location: Json<ClientCreateGisLocation>,
) -> Result<impl Responder> {
    Ok(gis_location_controller::create_gis_location(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        client_create_gis_location.into_inner(),
    ).await)
}
