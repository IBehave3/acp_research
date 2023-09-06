use crate::{
    controller::{auth, user_controller},
    infra::{database::DbPool, jwt_middleware::AuthenticatedClaims},
    model::{
        airthings::AirthingsAuth,
        auth::{CreateIdMapping, IdMapping, IdMappingUserInformation, LoginIdMapping},
        gray_wolf::GrayWolfAuth,
        uhoo_aura::UhooAuraAuth,
        user_model::{
            ClientCreateUser, ClientLoginUser, ClientUpdateUserAirthings, ClientUpdateUserGrayWolf,
            ClientUpdateUserUhooAura,
        },
    },
};
use actix_web::{
    get, patch, post,
    web::{self, Data, Json},
    Responder, Result,
};

#[post("/create-user")]
pub async fn create_user_post_handler(
    pool: Data<DbPool>,
    json: Json<ClientCreateUser>,
) -> Result<impl Responder> {
    Ok(user_controller::create_user(pool.into_inner(), json.into_inner()).await?)
}

#[post("/login-user")]
pub async fn login_user_get_handler(
    pool: Data<DbPool>,
    json: Json<ClientLoginUser>,
) -> Result<impl Responder> {
    Ok(user_controller::login_user(pool.into_inner(), json.into_inner()).await?)
}

#[get("/information-user")]
pub async fn information_user_get_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
) -> Result<impl Responder> {
    Ok(user_controller::get_user(pool.into_inner(), authenticated_claims.into_inner()).await?)
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
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    airthings_update: Json<ClientUpdateUserAirthings>,
) -> Result<impl Responder> {
    Ok(user_controller::update_user_airthings(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        airthings_update.into_inner(),
    )
    .await?)
}

#[patch("/gray-wolf-user")]
pub async fn gray_wolf_user_patch_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    gray_wolf_update: Json<ClientUpdateUserGrayWolf>,
) -> Result<impl Responder> {
    Ok(user_controller::update_user_gray_wolf(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        gray_wolf_update.into_inner(),
    )
    .await?)
}

#[patch("/uhoo-aura-user")]
pub async fn uhoo_aura_user_patch_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    uhoo_aura_update: Json<ClientUpdateUserUhooAura>,
) -> Result<impl Responder> {
    Ok(user_controller::update_user_uhoo_aura(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        uhoo_aura_update.into_inner(),
    )
    .await?)
}
