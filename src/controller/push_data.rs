use crate::controller::auth::IdMapping;
use crate::controller::auth::UserIdQueryExtractor;
use crate::infra::{collection, collection::BaseCollection, database};
use actix_web::web::Json;
use actix_web::web::Query;
use actix_web::{get, post, HttpResponse, Responder, Result};
use chrono::Utc;
use mongodb::{bson, bson::doc, bson::oid::ObjectId, bson::Document};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserPushData {
    #[serde(flatten)]
    data: Document,
}

#[derive(Deserialize, Serialize)]
struct PushData {
    _id: ObjectId,
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    created_at: bson::DateTime,
    #[serde(rename(serialize = "idMappingRefId", deserialize = "idMappingRefId"))]
    id_mapping_ref_id: ObjectId,
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    data_structure_id: String,
    data: UserPushData,
}

impl BaseCollection for PushData {
    type DocumentType = PushData;

    fn get_collection() -> mongodb::Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::PUSH_DATA_COLLECTION_NAME)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PushDataQueryExtractor {
    #[serde(flatten)]
    user_id_query_extractor: UserIdQueryExtractor,
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    data_structure_id: String,
    #[serde(rename(serialize = "deviceId", deserialize = "deviceId"))]
    device_id: Option<String>,
}

#[get("/push-data")]
pub async fn push_data_get_handler(query: Query<PushDataQueryExtractor>) -> Result<impl Responder> {
    let id_mapping =
        IdMapping::get_id_mapping_by_user_id(&query.user_id_query_extractor.user_id).await;

    if id_mapping.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let id_mapping = id_mapping.unwrap();

    let filter = doc! { "idMappingRefId": id_mapping._id, "dataStructureId": query.data_structure_id.to_owned() };
    let results = PushData::get_all_options(Some(filter), None).await;

    Ok(HttpResponse::Ok().json(results))
}

#[post("/push-data")]
pub async fn push_data_post_handler(
    query: Query<PushDataQueryExtractor>,
    json: Json<UserPushData>,
) -> Result<impl Responder> {
    let id_mapping =
        IdMapping::get_id_mapping_by_user_id(&query.user_id_query_extractor.user_id).await;

    if id_mapping.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let id_mapping = id_mapping.unwrap();

    PushData::add(PushData {
        _id: ObjectId::new(),
        created_at: bson::DateTime::from_chrono(Utc::now()),
        data_structure_id: query.data_structure_id.to_owned(),
        id_mapping_ref_id: id_mapping._id,
        data: json.into_inner(),
    })
    .await;

    Ok(HttpResponse::Ok().finish())
}
