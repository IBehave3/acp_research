use actix_web::{delete, get, post, HttpResponse, Responder, Result};

#[post("/create-user")]
pub async fn create_user_post_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/login-user")]
pub async fn login_user_get_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[delete("remove-user")]
pub async fn remove_user_delete_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
