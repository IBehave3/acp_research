use crate::infra::collection;
use crate::model::manage::ContainerGetQueryExtractor;
use actix_web::{get, web::Query, HttpResponse, Responder, Result};

#[get("/container")]
pub async fn container_get_handler(
    query: Query<ContainerGetQueryExtractor>,
) -> Result<impl Responder> {
    let results = collection::get_all_collection(&query.collection_id).await?;
    Ok(HttpResponse::Ok().json(results))
}

#[get("/container-name")]
pub async fn container_name_get_handler() -> Result<impl Responder> {
    let results = collection::get_all_collection_names().await?;
    Ok(HttpResponse::Ok().json(results))
}
