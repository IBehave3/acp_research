use crate::model::{auth::{CreateIdMapping, IdMapping, LoginIdMapping}, airthings::AirthingsAuth, gray_wolf::GrayWolfAuth, uhoo_aura::UhooAuraAuth};
use actix_web::{get, post, web::Json, Responder, Result, patch};
use actix_web_httpauth::extractors::bearer::BearerAuth;

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
    auth: BearerAuth
) -> Result<impl Responder> {
    Ok(IdMapping::get_by_token(auth.token()).await?)
}

#[patch("airthings-user")]
pub async fn airthings_user_patch_handler(
    auth: BearerAuth,
    airthings_auth: Json<AirthingsAuth>
) -> Result<impl Responder> {
    Ok(IdMapping::update_airthings(auth.token(), airthings_auth.into_inner()).await?)
}

#[patch("gray-wolf-user")]
pub async fn gray_wolf_user_patch_handler(
    auth: BearerAuth,
    gray_wolf_auth: Json<GrayWolfAuth>
) -> Result<impl Responder> {
    Ok(IdMapping::update_gray_wolf(auth.token(), gray_wolf_auth.into_inner()).await?)
}

#[patch("uhoo-aura-user")]
pub async fn uhoo_aura_user_patch_handler(
    auth: BearerAuth,
    uhoo_aura_auth: Json<UhooAuraAuth>
) -> Result<impl Responder> {
    Ok(IdMapping::update_uhoo_aura(auth.token(), uhoo_aura_auth.into_inner()).await?)
}