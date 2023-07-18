use crate::model::device::DeviceIdQueryExtractor;

use crate::model::auth::IdMapping;
use actix_web::{delete, post, web::Query, HttpResponse, Responder, Result};

#[post("/add-device")]
pub async fn add_device_get_handler(
    query: Query<DeviceIdQueryExtractor>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(
        &query
            .data_structure_query_extractor
            .user_id_query_extractor
            .user_id,
    )
    .await?;

    // NOTE: checking that the user exists
    let &mut id_mapping = match id_mapping {
        Some(ref mut id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    // NOTE: checking that the data structure exists
    let &mut data_structure_device_id_mapping = match id_mapping
        .data_structure_device_id_mapping
        .iter()
        .find(|x| x.data_structure_id == query.data_structure_query_extractor.data_structure_id)
    {
        Some(ref mut data_structure_device_id_mapping) => data_structure_device_id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    // NOTE: checking that the device id does not exists
    match data_structure_device_id_mapping.device_ids {
        Some(ref mut device_ids) => device_ids.retain(|device_id| *device_id != query.device_id),
        None => return Ok(HttpResponse::NotFound().finish()),
    }

    Ok(HttpResponse::Ok().finish())
}

#[delete("/remove-device")]
pub async fn remove_device_delete_hanlder(
    _query: Query<DeviceIdQueryExtractor>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
