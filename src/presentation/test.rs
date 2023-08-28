use actix_web::{get, HttpResponse, Responder, Result, web};

use crate::model::jwt::JwtToken;

#[get("/test")]
pub async fn test_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
