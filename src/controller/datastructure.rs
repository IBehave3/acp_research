use super::auth::{DataStructureDeviceMapping, IdMapping, UserIdQueryExtractor};
use super::push_data::PushData;
use crate::infra::collection::BaseCollection;
use actix_web::{get, post, web::Json, web::Query, HttpResponse, Responder, Result};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DataStructureQueryExtractor {
    #[serde(flatten)]
    user_id_query_extractor: UserIdQueryExtractor,
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    pub data_structure_id: String,
}

#[post("/data-structure")]
pub async fn datastructure_post_handler(
    query: Query<UserIdQueryExtractor>,
    json: Json<DataStructureDeviceMapping>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await;

    if id_mapping.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut id_mapping = id_mapping.unwrap();
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

    IdMapping::replace(id_mapping._id, id_mapping).await;

    Ok(HttpResponse::Ok().finish())
}

#[get("/reset-datastructure")]
pub async fn reset_datastructure_get_handler(
    query: Query<DataStructureQueryExtractor>,
) -> Result<impl Responder> {
    let id_mapping =
        IdMapping::get_id_mapping_by_user_id(&query.user_id_query_extractor.user_id).await;

    if id_mapping.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let id_mapping = id_mapping.unwrap();
    let filter = doc! { "dataStructureId": query.data_structure_id.to_owned(), "idMappingRefId": id_mapping._id };

    PushData::delete_options(filter, None).await;

    Ok(HttpResponse::Ok().finish())
}
