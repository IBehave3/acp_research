use actix_web::{get, HttpResponse, Responder, Result};



#[get("/test")]
pub async fn test_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
