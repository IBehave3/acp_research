use crate::infra::collection::BaseCollection;
use crate::model::auth::{CreateUserPostJsonExtractor, IdMapping, UserIdQueryExtractor};
use crate::model::{notification::Notification, push_data::PushData};
use actix_web::{delete, get, post, web::Json, web::Query, HttpResponse, Responder, Result};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

#[post("/create-user")]
pub async fn create_user_post_handler(
    query: Query<UserIdQueryExtractor>,
    json: Json<CreateUserPostJsonExtractor>,
) -> Result<impl Responder> {
    if IdMapping::id_mapping_exists(&query.user_id).await? {
        return Ok(HttpResponse::Conflict().finish());
    }

    let id_mapping = IdMapping {
        _id: ObjectId::new(),
        user_id: query.user_id.to_owned(),
        data_structure_device_id_mapping: json.into_inner().data_structure_device_mapping,
    };

    IdMapping::add(id_mapping).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/login-user")]
pub async fn login_user_get_handler(query: Query<UserIdQueryExtractor>) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await?;
    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok().json(id_mapping))
}

#[delete("remove-user")]
pub async fn remove_user_delete_handler(
    query: Query<UserIdQueryExtractor>,
) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await?;
    let id_mapping = match id_mapping {
        Some(id_mapping) => id_mapping,
        None => return Ok(HttpResponse::NotFound().finish()),
    };
    let ref_filter = doc! { "idMappingRefId": id_mapping._id };

    IdMapping::delete(id_mapping._id).await?;
    Notification::delete_all_options(ref_filter.clone(), None).await?;
    PushData::delete_all_options(ref_filter, None).await?;

    Ok(HttpResponse::Ok().finish())
}
