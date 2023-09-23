use crate::{
    controller::user_controller,
    infra::{database::DbPool, jwt_middleware::AuthenticatedClaims},
    model::user_model::{
            ClientCreateUser, ClientLoginUser, ClientUpdateUserAirthings, ClientUpdateUserGrayWolf,
            ClientUpdateUserUhooBusiness, ClientUpdateUserUhooHome, ClientUpdateUserKeychain,
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
    user_controller::create_user(pool.into_inner(), json.into_inner()).await
}

#[post("/login-user")]
pub async fn login_user_get_handler(
    pool: Data<DbPool>,
    json: Json<ClientLoginUser>,
) -> Result<impl Responder> {
    user_controller::login_user(pool.into_inner(), json.into_inner()).await
}

#[get("/information-user")]
pub async fn information_user_get_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
) -> Result<impl Responder> {
    user_controller::get_user(pool.into_inner(), authenticated_claims.into_inner()).await
}

#[patch("/airthings-user")]
pub async fn airthings_user_patch_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    airthings_update: Json<ClientUpdateUserAirthings>,
) -> Result<impl Responder> {
    user_controller::update_user_airthings(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        airthings_update.into_inner(),
    )
    .await
}

#[patch("/gray-wolf-user")]
pub async fn gray_wolf_user_patch_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    gray_wolf_update: Json<ClientUpdateUserGrayWolf>,
) -> Result<impl Responder> {
    user_controller::update_user_gray_wolf(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        gray_wolf_update.into_inner(),
    )
    .await
}

#[patch("/uhoo-business-user")]
pub async fn uhoo_business_user_patch_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    uhoo_business_update: Json<ClientUpdateUserUhooBusiness>,
) -> Result<impl Responder> {
    user_controller::update_user_uhoo_business(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        uhoo_business_update.into_inner(),
    )
    .await
}

#[patch("/uhoo-home-user")]
pub async fn uhoo_home_user_patch_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    uhoo_home_update: Json<ClientUpdateUserUhooHome>,
) -> Result<impl Responder> {
    user_controller::update_user_uhoo_home(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        uhoo_home_update.into_inner(),
    )
    .await
}

#[patch("/keychain-user")]
pub async fn keychain_user_patch_handler(
    pool: Data<DbPool>,
    authenticated_claims: web::ReqData<AuthenticatedClaims>,
    keychain_update: Json<ClientUpdateUserKeychain>,
) -> Result<impl Responder> {
    user_controller::update_user_keychain(
        pool.into_inner(),
        authenticated_claims.into_inner(),
        keychain_update.into_inner(),
    )
    .await
}


