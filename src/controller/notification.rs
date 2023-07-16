use crate::controller::auth::{IdMapping, UserIdQueryExtractor};
use crate::infra::{collection, collection::BaseCollection, database};
use actix_web::web::{Json, Query};
use actix_web::{get, post, HttpResponse, Responder, Result};
use mongodb::{bson, bson::doc, bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserNotification {
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    timestamp: bson::DateTime,
    message: String,
    #[serde(rename(serialize = "statusCode", deserialize = "statusCode"))]
    status_code: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Notification {
    _id: ObjectId,
    #[serde(rename(serialize = "idMappingRefId", deserialize = "idMappingRefId"))]
    id_mapping_ref_id: ObjectId,
    #[serde(flatten)]
    user_notification: UserNotification,
}

impl BaseCollection for Notification {
    type DocumentType = Notification;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::NOTIFICATION_COLLECTION_NAME)
    }
}

impl Notification {
    async fn get_notification_by_id_mapping_id(id_mapping_id: ObjectId) -> Vec<Self> {
        let filter = doc! { "idMappingRefId": id_mapping_id };
        let results = Notification::get_all_options(Some(filter), None).await;

        results
    }
}

#[get("/notification")]
pub async fn notification_get_handler(
    query: Query<UserIdQueryExtractor>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await;

    if id_mapping.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let id_mapping = id_mapping.unwrap();
    let results = Notification::get_notification_by_id_mapping_id(id_mapping._id).await;

    Ok(HttpResponse::Ok().json(results))
}

#[post("/notification")]
pub async fn notification_post_handler(
    query: Query<UserIdQueryExtractor>,
    json: Json<UserNotification>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await;

    if id_mapping.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    Notification::add(Notification {
        _id: ObjectId::new(),
        id_mapping_ref_id: id_mapping.unwrap()._id,
        user_notification: json.into_inner(),
    })
    .await;

    Ok(HttpResponse::Ok().finish())
}
