use crate::api::model::user_location_model::ClientCreateUserLocation;
use crate::{api::controller::user_location_controller, api::infra::jwt_middleware::AuthenticatedClaims};
use crate::api::infra::database::DbPool;
use actix_web::{
    post,
    web::{self, Data, Json},
    Responder, Result,
};

#[post("/custom")]
pub async fn create_user_location_post_presentation(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    client_create_user_location: Json<ClientCreateUserLocation>,
) -> Result<impl Responder> {
    Ok(user_location_controller::create_user_location(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        client_create_user_location.into_inner(),
    ).await)
}
