use crate::infra::jwt_middleware::AuthenticatedClaims;
use crate::model::gis_location_model::ClientCreateGisLocation;
use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse, Responder, Result,
};

#[post("")]
pub async fn create_gis_location_post_presentation(
    _pool: Data<crate::infra::database::DbPool>,
    _authenticated_claims: web::ReqData<AuthenticatedClaims>,
    _client_create_gis_location: Json<ClientCreateGisLocation>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
