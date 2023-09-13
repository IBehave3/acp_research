use actix_web::{HttpResponse, web::{Data, self}, post, Responder};
use crate::infra::jwt_middleware::AuthenticatedClaims;
use actix_web::Result;

#[post("")]
pub async fn create_gis_location_post_presentation(
    pool: Data<crate::infra::database::DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}