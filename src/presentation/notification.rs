use crate::infra::collection::BaseCollection;
use crate::model::auth::{IdMapping, UserIdQueryExtractor};
use crate::model::notification::{Notification, UserNotification};
use actix_web::web::{Json, Query};
use actix_web::{get, post, HttpResponse, Responder, Result};
use mongodb::bson::oid::ObjectId;

#[get("/notification")]
pub async fn notification_get_handler(
    query: Query<UserIdQueryExtractor>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await?;
    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let results = Notification::get_notification_by_id_mapping_id(id_mapping._id).await?;

    Ok(HttpResponse::Ok().json(results))
}

#[post("/notification")]
pub async fn notification_post_handler(
    query: Query<UserIdQueryExtractor>,
    json: Json<UserNotification>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await?;
    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Notification::add(Notification {
        _id: ObjectId::new(),
        id_mapping_ref_id: id_mapping._id,
        user_notification: json.into_inner(),
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}
