use actix_web::{get, HttpResponse, Responder, Result};

#[get("/container")]
pub async fn container_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/container-name")]
pub async fn container_name_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
