/*use crate::infra::collection::BaseCollection;
use crate::model::auth::IdMapping;
use crate::model::push_data::{PushData, PushDataQueryExtractor, UserPushData};
use actix_web::web::Json;
use actix_web::web::Query;
use actix_web::{get, post, HttpResponse, Responder, Result};
use chrono::Utc;
use mongodb::{bson, bson::doc, bson::oid::ObjectId, bson::Document};

#[get("/push-data")]
pub async fn push_data_get_handler(query: Query<PushDataQueryExtractor>) -> Result<impl Responder> {
    let id_mapping =
        IdMapping::get_id_mapping_by_user_id(&query.user_id_query_extractor.user_id).await?;
    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let filter: Document;

    if let Some(device_id) = query.device_id.to_owned() {
        filter = doc! { "idMappingRefId": id_mapping._id, "dataStructureId": query.data_structure_id.to_owned(), "deviceId": device_id };
    } else {
        filter = doc! { "idMappingRefId": id_mapping._id, "dataStructureId": query.data_structure_id.to_owned() };
    }

    let results = PushData::get_all_options(Some(filter), None).await?;

    Ok(HttpResponse::Ok().json(results))
}

#[post("/push-data")]
pub async fn push_data_post_handler(
    query: Query<PushDataQueryExtractor>,
    json: Json<UserPushData>,
) -> Result<impl Responder> {
    let id_mapping =
        IdMapping::get_id_mapping_by_user_id(&query.user_id_query_extractor.user_id).await?;

    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    PushData::add(PushData {
        _id: ObjectId::new(),
        created_at: bson::DateTime::from_chrono(Utc::now()),
        data_structure_id: query.data_structure_id.to_owned(),
        id_mapping_ref_id: id_mapping._id,
        device_id: None,
        data: json.into_inner(),
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}*/
