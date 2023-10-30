use std::sync::Arc;

use actix_web::{Responder, HttpResponse};
use diesel::{QueryDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;
use log::error;
use crate::api::infra::api_error::{ApiError, self};
use crate::api::infra::database::DbPool;
use crate::api::model::user_model::UserFitbitTwo;
use crate::schema::user_fitbit_two::dsl::user_fitbit_two;
use crate::schema::user_fitbit_two::{self as user_fitbit_two_fields};
use diesel::result::Error as diesel_error;

pub async fn verify_fitbit_two(pool: Arc<DbPool>, verify: String) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    if let Err(err) = user_fitbit_two
        .filter(user_fitbit_two_fields::verificationcode.eq(verify))
        .first::<UserFitbitTwo>(database_connection)
        .await
    {
        error!("{err}");

        if err == diesel_error::NotFound {
            return Err(api_error::ApiError::FitbitTwoVerificationCodeNotFound.into());
        }

        return Err(api_error::ApiError::DbError {
            message: "verify_fitbit_two failed".to_string(),
        }
        .into());
    }

    Ok(HttpResponse::NoContent().finish())
}
