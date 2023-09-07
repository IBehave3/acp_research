use std::sync::Arc;

use actix_web::{HttpResponse, Responder};
use bcrypt::hash_with_result;
use log::{error, info};

use diesel::result::{DatabaseErrorKind, Error as diesel_error};
use diesel::{self, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{RunQueryDsl, AsyncPgConnection};

use crate::infra::api_error;
use crate::infra::api_error::ApiError;
use crate::infra::database::DbPool;
use crate::infra::jwt_middleware::AuthenticatedClaims;
use crate::model::airthings_model::Airthings;
use crate::model::jwt::{JwtCustomClaims, JwtToken};
use crate::model::user_model::{
    ClientCreateUser, ClientGetUserInformation, ClientLoginUser, ClientUpdateUserAirthings,
    ClientUpdateUserGrayWolf, ClientUpdateUserUhooAura, CreateUser, CreateUserAirthings,
    CreateUserGrayWolf, CreateUserUhooAura, UpdateUserAirthings, UpdateUserGrayWolf,
    UpdateUserUhooAura, User, UserAirthings, UserGrayWolfs, UserUhooAuras,
};
use crate::schema::user_airthings::dsl::user_airthings;
use crate::schema::user_airthings::{self as user_airthings_fields};
use crate::schema::user_gray_wolfs::dsl::user_gray_wolfs;
use crate::schema::user_gray_wolfs::{self as user_gray_wolfs_fields};
use crate::schema::user_uhoo_auras::dsl::user_uhoo_auras;
use crate::schema::user_uhoo_auras::{self as user_uhoo_auras_fields};
use crate::schema::users::dsl::users;
use crate::schema::users::{self as users_fields};

// FIXME: put into env vars
const BCRYPT_ITERATIONS: u32 = 12;
pub const MIN_PASSWORD_LEN: usize = 8;
pub const MIN_USERNAME_LEN: usize = 8;

pub async fn get_user(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;

    let user_information: (
        User,
        Option<UserAirthings>,
        Option<UserGrayWolfs>,
        Option<UserUhooAuras>,
    ) = users_fields::table
        .find(authenticated_claims.user_id)
        .left_join(user_airthings_fields::table)
        .left_join(user_gray_wolfs_fields::table)
        .left_join(user_uhoo_auras_fields::table)
        .select((
            User::as_select(),
            Option::<UserAirthings>::as_select(),
            Option::<UserGrayWolfs>::as_select(),
            Option::<UserUhooAuras>::as_select(),
        ))
        .first::<(
            User,
            Option<UserAirthings>,
            Option<UserGrayWolfs>,
            Option<UserUhooAuras>,
        )>(database_connection)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(ClientGetUserInformation {
        user: user_information.0,
        airthings: user_information.1,
        gray_wolf: user_information.2,
        uhoo_aura: user_information.3,
    }))
}

pub async fn create_user(
    pool: Arc<DbPool>,
    client_create_user: ClientCreateUser,
) -> actix_web::Result<impl Responder> {
    if client_create_user.username.len() < MIN_USERNAME_LEN {
        return Err(ApiError::InvalidUsernameError.into());
    }
    if client_create_user.password.len() < MIN_PASSWORD_LEN {
        return Err(ApiError::InvalidPasswordError.into());
    }

    let hash = hash_with_result(client_create_user.password, BCRYPT_ITERATIONS)
        .map_err(|_| ApiError::HashCreationError)?;

    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let blog_id = match diesel::insert_into(users)
        .values(CreateUser {
            username: client_create_user.username,
            passwordhash: hash.to_string(),
            salt: hash.get_salt(),
        })
        .execute(database_connection)
        .await
    {
        Ok(blog_id) => blog_id,
        Err(err) => {
            error!("{err}");

            if let diesel_error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                return Ok(HttpResponse::Conflict().finish());
            }

            return Err(api_error::ApiError::DbError {
                message: "create_user failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Created().body(blog_id.to_string()))
}

pub async fn login_user(
    pool: Arc<DbPool>,
    client_login_user: ClientLoginUser,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let user_result: User = match users
        .filter(users_fields::username.eq(&client_login_user.username))
        .first(database_connection)
        .await
    {
        Ok(blog_result) => blog_result,
        Err(err) => {
            error!("{err}");

            if err == diesel_error::NotFound {
                return Err(api_error::ApiError::Unauthorized {
                    message: "username not found".to_string(),
                }
                .into());
            }

            return Err(api_error::ApiError::DbError {
                message: "get_blog failed".to_string(),
            }
            .into());
        }
    };

    if !(bcrypt::verify(client_login_user.password, &user_result.passwordhash)
        .map_err(|_| ApiError::TokenValidationError)?)
    {
        return Err(ApiError::Unauthorized {
            message: "password incorrect".to_string(),
        }
        .into());
    }

    let jwt_token = JwtToken::new(JwtCustomClaims {
        user_id: user_result.id,
        username: client_login_user.username,
    })
    .map_err(|_| ApiError::TokenCreationError)?;

    Ok(HttpResponse::Ok().json(jwt_token))
}

pub async fn update_user_airthings(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_update_user_airthings: ClientUpdateUserAirthings,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;

    let device_ids: Vec<String> = client_update_user_airthings
        .device_ids
        .into_iter()
        .collect();

    match diesel::insert_into(user_airthings)
        .values(CreateUserAirthings {
            userid: authenticated_claims.user_id,
            clientid: client_update_user_airthings.client_id.to_owned(),
            clientsecret: client_update_user_airthings.client_secret.to_owned(),
            groupid: client_update_user_airthings.group_id.to_owned(),
            deviceids: device_ids.to_owned(),
        })
        .on_conflict(user_airthings_fields::userid)
        .do_update()
        .set(UpdateUserAirthings {
            clientid: client_update_user_airthings.client_id,
            clientsecret: client_update_user_airthings.client_secret,
            groupid: client_update_user_airthings.group_id,
            deviceids: device_ids,
        })
        .execute(database_connection)
        .await
    {
        Ok(blog_id) => blog_id,
        Err(err) => {
            error!("{err}");

            return Err(api_error::ApiError::DbError {
                message: "update_user_airthings failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_user_gray_wolf(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_update_user_gray_wolf: ClientUpdateUserGrayWolf,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;

    let device_ids: Vec<String> = client_update_user_gray_wolf
        .device_ids
        .into_iter()
        .collect();

    match diesel::insert_into(user_gray_wolfs)
        .values(CreateUserGrayWolf {
            userid: authenticated_claims.user_id,
            apikey: client_update_user_gray_wolf.api_key.clone(),
            deviceids: device_ids.to_owned(),
        })
        .on_conflict(user_gray_wolfs_fields::userid)
        .do_update()
        .set(UpdateUserGrayWolf {
            apikey: client_update_user_gray_wolf.api_key,
            deviceids: device_ids,
        })
        .execute(database_connection)
        .await
    {
        Ok(_) => (),
        Err(err) => {
            error!("{err}");

            return Err(api_error::ApiError::DbError {
                message: "update_user_gray_wolf failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_user_uhoo_aura(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_update_user_uhoo_aura: ClientUpdateUserUhooAura,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;

    let device_ids: Vec<String> = client_update_user_uhoo_aura
        .device_ids
        .into_iter()
        .collect();

    match diesel::insert_into(user_uhoo_auras)
        .values(CreateUserUhooAura {
            userid: authenticated_claims.user_id,
            clientsecret: client_update_user_uhoo_aura.client_secret.to_owned(),
            deviceids: device_ids.to_owned(),
        })
        .on_conflict(user_uhoo_auras_fields::userid)
        .do_update()
        .set(UpdateUserUhooAura {
            clientsecret: client_update_user_uhoo_aura.client_secret,
            deviceids: device_ids,
        })
        .execute(database_connection)
        .await
    {
        Ok(blog_id) => blog_id,
        Err(err) => {
            error!("{err}");

            return Err(api_error::ApiError::DbError {
                message: "update_user_uhoo_aura failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_airthings_users(connection: &mut AsyncPgConnection) -> anyhow::Result<Vec<UserAirthings>> {
    info!("HERE");
    let airthings_users: Vec<UserAirthings> = user_airthings
        .select(UserAirthings::as_select())
        .load(connection)
        .await?;

    Ok(airthings_users)
}
