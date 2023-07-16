use crate::infra::{collection, collection::BaseCollection, database};
use actix_web::{delete, get, post, web::Json, web::Query, HttpResponse, Responder, Result};
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};

impl BaseCollection for IdMapping {
    type DocumentType = IdMapping;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::ID_MAPPING_COLLECTION_NAME)
    }
}

impl IdMapping {
    pub async fn get_id_mapping_by_user_id(user_id: &str) -> Option<Self> {
        let filter = doc! { "userId": user_id };
        return IdMapping::get_options(Some(filter), None).await;
    }

    pub async fn id_mapping_exists(user_id: &str) -> bool {
        Self::get_id_mapping_by_user_id(user_id).await.is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataStructureDeviceMapping {
    #[serde(rename(serialize = "dataStructureId", deserialize = "dataStructureId"))]
    pub data_structure_id: String,
    #[serde(rename(serialize = "deviceIds", deserialize = "deviceIds"))]
    pub device_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct IdMapping {
    pub _id: ObjectId,
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(
        serialize = "dataStructureDeviceIdMapping",
        deserialize = "dataStructureDeviceIdMapping"
    ))]
    pub data_structure_device_id_mapping: Vec<DataStructureDeviceMapping>,
}

#[derive(Deserialize, Serialize)]
pub struct UserIdQueryExtractor {
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    pub user_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserPostJsonExtractor {
    #[serde(rename(
        serialize = "dataStructureDeviceMapping",
        deserialize = "dataStructureDeviceMapping"
    ))]
    data_structure_device_mapping: Vec<DataStructureDeviceMapping>,
}

#[post("/create-user")]
pub async fn create_user_post_handler(
    query: Query<UserIdQueryExtractor>,
    json: Json<CreateUserPostJsonExtractor>,
) -> Result<impl Responder> {
    if IdMapping::id_mapping_exists(&query.user_id).await {
        return Ok(HttpResponse::Conflict().finish());
    }

    let id_mapping = IdMapping {
        _id: ObjectId::new(),
        user_id: query.user_id.to_owned(),
        data_structure_device_id_mapping: json.into_inner().data_structure_device_mapping,
    };

    IdMapping::add(id_mapping).await;

    Ok(HttpResponse::Ok().finish())
}

#[get("/login-user")]
pub async fn login_user_get_handler(query: Query<UserIdQueryExtractor>) -> Result<impl Responder> {
    let id_mapping = IdMapping::get_id_mapping_by_user_id(&query.user_id).await;

    if id_mapping.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(id_mapping.unwrap()))
}

#[delete("remove-user")]
pub async fn remove_user_delete_handler() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
