use actix_web::{get, post, HttpResponse, Responder, Result};

#[post("/device")]
pub async fn device_post_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/reset-device")]
pub async fn reset_device_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
