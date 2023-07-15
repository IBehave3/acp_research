use actix_web::{get, post, HttpResponse, Responder, Result};

#[get("/notification")]
pub async fn notification_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/notification")]
pub async fn notification_post_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
