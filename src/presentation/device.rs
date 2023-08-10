use crate::{/*infra::collection::BaseCollection,*/ model::device::DeviceIdQueryExtractor};

//use crate::model::auth::IdMapping;
use actix_web::{delete, post, web::Query, HttpResponse, Responder, Result};
//use mongodb::bson::doc;
//use mongodb::options::UpdateOptions;

#[post("/device")]
pub async fn add_device_get_handler(
    _query: Query<DeviceIdQueryExtractor>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}

#[delete("/device")]
pub async fn device_delete_handler(_query: Query<DeviceIdQueryExtractor>) -> Result<impl Responder> {
    /*let id_mapping = IdMapping::get_id_mapping_by_user_id(
        &query
            .data_structure_query_extractor
            .user_id_query_extractor
            .user_id,
    )
    .await?;

    // NOTE: checking that the user exists
    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let filter = doc! { "_id": id_mapping._id };
    let update = doc! { "$pull":
                            { "dataStructureDeviceIdMapping":
                                { query.data_structure_query_extractor.data_structure_id.clone():
                                    {
                                        "deviceIds": [query.device_id.to_owned()]
                                    }
                                }
                            },
                            UpdateOptions::array_filters(doc! {})
    };

    IdMapping::update(filter, update).await?;*/

    Ok(HttpResponse::Ok().finish())
}
