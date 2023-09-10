use crate::{infra::{jwt_middleware::AuthenticatedClaims, database::DbPool}};
use actix_web::{post, web::{Json, self, Data}, Responder, Result, HttpResponse};
use bson::Document;

#[post("")]
pub async fn create_fitbit_post_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    data: Json<Document>
) -> Result<impl Responder> {
    //Ok(Fitbit::create_fitbit_data(&authenticated_claims.username, data.into_inner()).await?)
    Ok(HttpResponse::Ok().finish())
}