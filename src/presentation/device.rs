use crate::{infra::collection::BaseCollection, model::device::DeviceIdQueryExtractor};

use crate::model::auth::IdMapping;
use actix_web::{delete, post, web::Query, HttpResponse, Responder, Result};

#[post("/add-device")]
pub async fn add_device_get_handler(
    query: Query<DeviceIdQueryExtractor>,
) -> Result<impl Responder> {
    let mut id_mapping = IdMapping::get_id_mapping_by_user_id(
        &query
            .data_structure_query_extractor
            .user_id_query_extractor
            .user_id,
    )
    .await?;

    // NOTE: checking that the user exists
    let id_mapping = match &mut id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    // NOTE: checking that the data structure and removin the given device_id if it exists
    match id_mapping
        .data_structure_device_id_mapping
        .iter_mut()
        .find(|x| x.data_structure_id == query.data_structure_query_extractor.data_structure_id)
    {
        Some(data_structure_device_id_mapping) => {
            match &mut data_structure_device_id_mapping.device_ids {
                Some(device_ids) => device_ids.retain(|device_id| *device_id != query.device_id),
                None => return Ok(HttpResponse::NotFound().finish()),
            }
        }
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    IdMapping::replace(id_mapping._id, &id_mapping).await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/remove-device")]
pub async fn remove_device_delete_hanlder(
    _query: Query<DeviceIdQueryExtractor>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
