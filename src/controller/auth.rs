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
    pub async fn update_airthings(
        username: &str,
        airthings_auth: AirthingsAuth,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let airthings_update = bson::to_document(&airthings_auth)?;
        let update = doc! {
            "$set": {
                "airthings": airthings_update
            }
        };
        let filter = doc! {
            "username": username,
        };

        IdMapping::update_options(filter, update, None).await?;
        Ok(HttpResponse::Ok().finish())
    }

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

    pub async fn create(
        create_id_mapping: CreateIdMapping,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        if create_id_mapping.username.len() <= MIN_USERNAME_LEN {
            return Ok(HttpResponse::BadRequest().body(format!(
                "invalid username min characters: {MIN_USERNAME_LEN}"
            )));
        }
        if create_id_mapping.password.len() <= MIN_PASSWORD_LEN {
            return Ok(HttpResponse::BadRequest().body(format!(
                "invalid username min characters: {MIN_PASSWORD_LEN}"
            )));
        }

        if Self::get_by_username(&create_id_mapping.username)
            .await?
            .is_some()
        {
            return Ok(HttpResponse::Conflict().finish());
        }

        let hash = hash_with_result(create_id_mapping.password, BCRYPT_ITERATIONS)?;

        let inserted_id = IdMapping::add(IdMapping {
            id: ObjectId::new(),
            created_at: DateTime::from_chrono(Utc::now()),
            user_auth: IdMappingUserAuth {
                username: create_id_mapping.username,
                password_hash: hash.to_string(),
                salt: hash.get_salt(),
            },
            sensor_auth: IdMappingSensorAuth {
                airthings: create_id_mapping.sensor_auth.airthings,
                gray_wolf: create_id_mapping.sensor_auth.gray_wolf,
                uhoo_aura: create_id_mapping.sensor_auth.uhoo_aura,
            },
            user_information: IdMappingUserInformation {
                age: create_id_mapping.user_information.age,
                gender: create_id_mapping.user_information.gender,
                race: create_id_mapping.user_information.race,
                birth_location: create_id_mapping.user_information.birth_location,
                home_original_location: create_id_mapping.user_information.home_original_location,
                home_last_five_years_location: create_id_mapping
                    .user_information
                    .home_last_five_years_location,
                employment_status: create_id_mapping.user_information.employment_status,
                level_of_education: create_id_mapping.user_information.level_of_education,
            },
        })
        .await?;

        Ok(HttpResponse::Ok().json(inserted_id))
    }

    pub async fn login(
        login_id_mapping: LoginIdMapping,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let id_mapping = match Self::get_by_username(&login_id_mapping.username).await? {
            Some(id_mapping) => id_mapping,
            None => {
                return Ok(HttpResponse::NotFound().finish());
            }
        };

        if !(bcrypt::verify(login_id_mapping.password, &id_mapping.user_auth.password_hash)?) {
            return Ok(HttpResponse::Unauthorized().finish());
        }

        Ok(HttpResponse::Ok().json(JwtToken::new(JwtCustomClaims {
            username: login_id_mapping.username,
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
