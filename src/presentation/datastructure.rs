use crate::infra::collection::BaseCollection;
use crate::model::auth::{DataStructureDeviceMapping, IdMapping, UserIdQueryExtractor};
use crate::model::datastructure::DataStructureQueryExtractor;
use crate::model::push_data::PushData;
use actix_web::{get, post, web::Json, web::Query, HttpResponse, Responder, Result};
use log::error;
use mongodb::bson::doc;

#[post("/data-structure")]
pub async fn datastructure_post_handler(
    query: Query<UserIdQueryExtractor>,
    json: Json<DataStructureDeviceMapping>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await?;
    let mut id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };
    let data_structure_mapping = json.into_inner();
    let res = id_mapping
        .data_structure_device_id_mapping
        .iter()
        .find(|x| x.data_structure_id == data_structure_mapping.data_structure_id);

    if res.is_some() {
        return Ok(HttpResponse::Conflict().finish());
    }

    id_mapping
        .data_structure_device_id_mapping
        .push(data_structure_mapping);

    let bson_data_structure_device_id_mapping =
        match bson::to_bson(&id_mapping.data_structure_device_id_mapping) {
            Ok(bson) => bson,
            Err(err) => {
                error!("{}", err);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        };

    let filter = doc! { "_id": id_mapping._id };
    let update =
        doc! { "$set": { "dataStructureDeviceIdMapping": bson_data_structure_device_id_mapping }};
    IdMapping::update_options(filter, update, None).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/reset-datastructure")]
pub async fn reset_datastructure_get_handler(
    query: Query<DataStructureQueryExtractor>,
) -> Result<impl Responder> {
    let id_mapping =
        IdMapping::get_id_mapping_by_user_id(&query.user_id_query_extractor.user_id).await?;
    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let filter = doc! { "dataStructureId": query.data_structure_id.to_owned(), "idMappingRefId": id_mapping._id };

    PushData::delete_all_options(filter, None).await?;

    Ok(HttpResponse::Ok().finish())
}
