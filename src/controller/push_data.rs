use actix_web::{get, post, HttpResponse, Responder, Result};

#[get("/push-data")]
pub async fn push_data_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/push-data")]
pub async fn push_data_post_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
