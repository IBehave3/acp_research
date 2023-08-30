use crate::{
    controller::auth,
    infra::jwt_middleware::AuthenticatedClaims,
    model::{
        airthings::AirthingsAuth,
        auth::{CreateIdMapping, IdMapping, IdMappingUserInformation, LoginIdMapping},
        gray_wolf::GrayWolfAuth,
        uhoo_aura::UhooAuraAuth,
    },
};
use actix_web::{
    get, patch, post,
    web::{self, Json},
    HttpResponse, Responder, Result,
};

#[post("/create-user")]
pub async fn create_user_post_handler(json: Json<CreateIdMapping>) -> Result<impl Responder> {
    Ok(IdMapping::create(json.into_inner()).await?)
}

#[post("/login-user")]
pub async fn login_user_get_handler(json: Json<LoginIdMapping>) -> Result<impl Responder> {
    Ok(IdMapping::login(json.into_inner()).await?)
}

#[get("/information-user")]
pub async fn information_user_get_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
) -> Result<impl Responder> {
    Ok(IdMapping::get_by_http_response_by_email(&authenticated_claims.username).await?)
}

#[patch("/information-user")]
pub async fn update_user_information(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    user_information: Json<IdMappingUserInformation>,
) -> Result<impl Responder> {
    Ok(IdMapping::update_user_information(
        &authenticated_claims.username,
        user_information.into_inner(),
    )
    .await?)
}

#[patch("/airthings-user")]
pub async fn airthings_user_patch_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    airthings_auth: Json<AirthingsAuth>,
) -> Result<impl Responder> {
    Ok(
        IdMapping::update_airthings(&authenticated_claims.username, airthings_auth.into_inner())
            .await?,
    )
}

#[patch("/gray-wolf-user")]
pub async fn gray_wolf_user_patch_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    gray_wolf_auth: Json<GrayWolfAuth>,
) -> Result<impl Responder> {
    Ok(
        IdMapping::update_gray_wolf(&authenticated_claims.username, gray_wolf_auth.into_inner())
            .await?,
    )
}

#[patch("/uhoo-aura-user")]
pub async fn uhoo_aura_user_patch_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    uhoo_aura_auth: Json<UhooAuraAuth>,
) -> Result<impl Responder> {
    Ok(
        IdMapping::update_uhoo_aura(&authenticated_claims.username, uhoo_aura_auth.into_inner())
            .await?,
    )
}
