
use crate::infra::{collection, collection::BaseCollection, database};
use crate::model::airthings::AirthingsAuth;
use crate::model::auth::{CreateIdMapping, IdMapping, LoginIdMapping};
use crate::model::gray_wolf::GrayWolfAuth;
use crate::model::jwt::{JwtClaims, JwtToken};
use crate::model::uhoo_aura::UhooAuraAuth;
use actix_web::{HttpResponse, Responder, Result};
use bcrypt::hash_with_result;

use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::{bson::doc, bson::DateTime, Collection};

const BCRYPT_ITERATIONS: u32 = 12;

impl BaseCollection for IdMapping {
    type DocumentType = IdMapping;

    fn get_collection() -> Collection<Self::DocumentType> {
        let db = database::get_db_connection();

        db.collection(collection::ID_MAPPING_COLLECTION_NAME)
    }
}

impl IdMapping {
    pub async fn update_airthings(
        email: &str,
        airthings_auth: AirthingsAuth,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let airthings_update = bson::to_document(&airthings_auth)?;
        let update = doc! {
            "$set": {
                "airthings": airthings_update
            }
        };
        let filter = doc! {
            "email": email,
        };

        IdMapping::update_options(filter, update, None).await?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn update_gray_wolf(
        email: &str,
        gray_wolf_auth: GrayWolfAuth,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let gray_wolf_update = bson::to_document(&gray_wolf_auth)?;
        let update = doc! {
            "$set": {
                "grayWolf": gray_wolf_update
            }
        };
        let filter = doc! {
            "email": email,
        };

        IdMapping::update_options(filter, update, None).await?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn update_uhoo_aura(
        email: &str,
        uhoo_aura_auth: UhooAuraAuth,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let uhoo_aura_update = bson::to_document(&uhoo_aura_auth)?;
        let update = doc! {
            "$set": {
                "uhooAura": uhoo_aura_update
            }
        };
        let filter = doc! {
            "email": email,
        };

        IdMapping::update_options(filter, update, None).await?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn get_by_email(
        email: &str,
    ) -> Result<Option<IdMapping>, Box<dyn std::error::Error>> {
        let filter = doc! {
            "email": email,
        };

        IdMapping::get_options(Some(filter), None).await
    }

    pub async fn get_by_http_response_by_email(email: &str) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let id_mapping = Self::get_by_email(email).await?;

        if let Some(id_mapping) = id_mapping {
            Ok(HttpResponse::Ok().json(id_mapping))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    pub async fn create(
        create_id_mapping: CreateIdMapping,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        if Self::get_by_email(&create_id_mapping.email)
            .await?
            .is_some()
        {
            return Ok(HttpResponse::Conflict().finish());
        }

        let hash = hash_with_result(create_id_mapping.password, BCRYPT_ITERATIONS)?;

        let inserted_id = IdMapping::add(IdMapping {
            id: ObjectId::new(),
            email: create_id_mapping.email,
            created_at: DateTime::from_chrono(Utc::now()),
            password_hash: hash.to_string(),
            salt: hash.get_salt(),
            airthings: create_id_mapping.airthings,
            gray_wolf: create_id_mapping.gray_wolf,
            uhoo_aura: create_id_mapping.uhoo_aura,
        })
        .await?;

        Ok(HttpResponse::Ok().json(inserted_id))
    }

    pub async fn login(
        login_id_mapping: LoginIdMapping,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let id_mapping = match Self::get_by_email(&login_id_mapping.email).await? {
            Some(id_mapping) => id_mapping,
            None => {
                return Ok(HttpResponse::NotFound().finish());
            }
        };

        if !(bcrypt::verify(login_id_mapping.password, &id_mapping.password_hash)?) {
            return Ok(HttpResponse::Unauthorized().finish());
        }

        Ok(HttpResponse::Ok().json(JwtToken::new(JwtClaims {
            email: login_id_mapping.email,
        })?))
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
