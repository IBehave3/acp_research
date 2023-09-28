use crate::{
    api::controller::fitbit_controller,
    api::infra::jwt_middleware::AuthenticatedClaims,
    api::infra::database::DbPool,
    api::model::fitbit_model::ClientCreateFitbit,
};
use actix_web::{
    post,
    web::{self, Data, Json},
    Responder, Result,
};

#[post("")]
pub async fn create_fitbit_post_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    client_fitbit: Json<ClientCreateFitbit>,
) -> Result<impl Responder> {
    fitbit_controller::create_fitbit(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        client_fitbit.into_inner(),
    )
    .await
}
