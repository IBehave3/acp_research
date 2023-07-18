

use crate::model::device::DeviceIdQueryExtractor;

use actix_web::{delete, post, web::Query, HttpResponse, Responder, Result};


#[post("/add-device")]
pub async fn add_device_get_handler(
    _query: Query<DeviceIdQueryExtractor>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[delete("/remove-device")]
pub async fn remove_device_delete_hanlder(
    _query: Query<DeviceIdQueryExtractor>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
