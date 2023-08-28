use crate::{model::{auth::{CreateIdMapping, IdMapping, LoginIdMapping}, airthings::AirthingsAuth, gray_wolf::GrayWolfAuth, uhoo_aura::UhooAuraAuth}, infra::jwt_middleware::AuthenticatedClaims};
use actix_web::{get, post, web::{Json, self}, Responder, Result, patch};

#[post("/create-user")]
pub async fn create_user_post_handler(
    json: Json<CreateIdMapping>
) -> Result<impl Responder> {
    Ok(IdMapping::create(json.into_inner()).await?)
}

#[get("/login-user")]
pub async fn login_user_get_handler(
    json: Json<LoginIdMapping>
) -> Result<impl Responder> {
    Ok(IdMapping::login(json.into_inner()).await?)
}

#[get("/information-user")]
pub async fn information_user_get_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>
) -> Result<impl Responder> {
    Ok(IdMapping::get_by_http_response_by_email(&authenticated_claims.email).await?)
}

#[patch("airthings-user")]
pub async fn airthings_user_patch_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    airthings_auth: Json<AirthingsAuth>
) -> Result<impl Responder> {
    Ok(IdMapping::update_airthings(&authenticated_claims.email, airthings_auth.into_inner()).await?)
}

#[patch("gray-wolf-user")]
pub async fn gray_wolf_user_patch_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    gray_wolf_auth: Json<GrayWolfAuth>
) -> Result<impl Responder> {
    Ok(IdMapping::update_gray_wolf(&authenticated_claims.email, gray_wolf_auth.into_inner()).await?)
}

#[patch("uhoo-aura-user")]
pub async fn uhoo_aura_user_patch_handler(
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    uhoo_aura_auth: Json<UhooAuraAuth>
) -> Result<impl Responder> {
    Ok(IdMapping::update_uhoo_aura(&authenticated_claims.email, uhoo_aura_auth.into_inner()).await?)
}