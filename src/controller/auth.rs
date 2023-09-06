use crate::infra::{collection, collection::BaseCollection, database};
use crate::model::airthings::AirthingsAuth;
use crate::model::auth::{
    CreateIdMapping, IdMapping, IdMappingSensorAuth, IdMappingUserAuth, IdMappingUserInformation,
    LoginIdMapping,
};
use crate::model::gray_wolf::GrayWolfAuth;
use crate::model::jwt::{JwtCustomClaims, JwtToken};
use crate::model::uhoo_aura::UhooAuraAuth;
use actix_web::{HttpResponse, Responder, Result};
use bcrypt::hash_with_result;

use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::{bson::doc, bson::DateTime, Collection};

const BCRYPT_ITERATIONS: u32 = 12;
const MIN_PASSWORD_LEN: usize = 8;
const MIN_USERNAME_LEN: usize = 8;

impl BaseCollection for IdMapping {
    type DocumentType = IdMapping;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::ID_MAPPING_COLLECTION_NAME)
    }
}

impl IdMapping {
    pub async fn update_gray_wolf(
        username: &str,
        gray_wolf_auth: GrayWolfAuth,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let gray_wolf_update = bson::to_document(&gray_wolf_auth)?;
        let update = doc! {
            "$set": {
                "grayWolf": gray_wolf_update
            }
        };
        let filter = doc! {
            "username": username,
        };

        IdMapping::update_options(filter, update, None).await?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn update_user_information(
        username: &str,
        user_information: IdMappingUserInformation
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let user_information_update = bson::to_document(&user_information)?;
        let update = doc! {
            "$set": user_information_update
        };
        let filter = doc! {
            "username": username,
        };

        IdMapping::update_options(filter, update, None).await?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn update_uhoo_aura(
        username: &str,
        uhoo_aura_auth: UhooAuraAuth,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let uhoo_aura_update = bson::to_document(&uhoo_aura_auth)?;
        let update = doc! {
            "$set": {
                "uhooAura": uhoo_aura_update
            }
        };
        let filter = doc! {
            "username": username,
        };

        IdMapping::update_options(filter, update, None).await?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn get_by_username(
        username: &str,
    ) -> Result<Option<IdMapping>, Box<dyn std::error::Error>> {
        let filter = doc! {
            "username": username,
        };

        IdMapping::get_options(Some(filter), None).await
    }

    pub async fn get_by_http_response_by_email(
        username: &str,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let id_mapping = Self::get_by_username(username).await?;

        if let Some(id_mapping) = id_mapping {
            Ok(HttpResponse::Ok().json(id_mapping))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    pub async fn get_airthings_users() -> Result<Vec<IdMapping>, Box<dyn std::error::Error>> {
        let filter = doc! { "airthings": { "$ne": null } };
        let results = IdMapping::get_all_options(Some(filter), None).await?;

        Ok(results)
    }

    pub async fn get_gray_wolf_users() -> Result<Vec<IdMapping>, Box<dyn std::error::Error>> {
        let filter = doc! { "grayWolf": { "$ne": null } };
        let results = IdMapping::get_all_options(Some(filter), None).await?;

        Ok(results)
    }

    pub async fn get_uhoo_aura_users() -> Result<Vec<IdMapping>, Box<dyn std::error::Error>> {
        let filter = doc! { "uhooAura": { "$ne": null } };
        let results = IdMapping::get_all_options(Some(filter), None).await?;

        Ok(results)
    }
}
