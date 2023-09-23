use actix_web::{
    http::StatusCode,
    HttpResponse, HttpResponseBuilder, error,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use log::error;

use crate::controller::user_controller::{MIN_PASSWORD_LEN, MIN_USERNAME_LEN};

#[derive(Debug, Display, Error, Deserialize, Serialize)]
pub enum ApiError {
    #[display(fmt = "{}", message)]
    Unauthorized { message: String },
    DbError { message: String },
    DbPoolError,
    HashCreationError,
    TokenCreationError,
    TokenValidationError,
    InvalidPasswordError,
    InvalidUsernameError,
    UserNotFoundError
}

impl error::ResponseError<> for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Unauthorized { message } => {
                HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).json(ApiError::Unauthorized {
                    message: message.to_owned(),
                })
            },
            ApiError::HashCreationError => {
                error!("failed to hash password");
                HttpResponse::InternalServerError().finish()
            }
            ApiError::DbError { message } => {
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(ApiError::DbError {
                    message: message.to_owned(),
                })
            },
            ApiError::DbPoolError => {
                error!("failed to get connection from db pool");
                HttpResponse::InternalServerError().finish()
            },
            ApiError::TokenCreationError => {
                error!("failed to create token");
                HttpResponse::InternalServerError().finish()
            },
            ApiError::TokenValidationError => {
                error!("failed to validate token");
                HttpResponse::InternalServerError().finish()
            }
            ApiError::InvalidPasswordError => {
                HttpResponse::BadRequest().body(format!("invalid password: min password length {}", MIN_PASSWORD_LEN))
            },
            ApiError::InvalidUsernameError => {
                HttpResponse::BadRequest().body(format!("invalid username: min username length {}", MIN_USERNAME_LEN))
            },
            ApiError::UserNotFoundError => {
                HttpResponse::NotFound().body("invalid username: user not found")
            }
        }
    }
}