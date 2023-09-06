use crate::{model::fitbit::Fitbit, infra::jwt_middleware::AuthenticatedClaims};
use actix_web::{post, web::{Json, self}, Responder, Result};
use bson::Document;

#[post("")]
pub async fn create_fitbit_post_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    data: Json<Document>
) -> Result<impl Responder> {
    Ok(Fitbit::create_fitbit_data(&authenticated_claims.username, data.into_inner()).await?)
}