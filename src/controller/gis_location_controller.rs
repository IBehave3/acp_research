use std::sync::Arc;

use actix_web::{HttpResponse, Responder};

use diesel::sql_types::{Double, Integer};
use diesel::{sql_query, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use log::{error, info};

use crate::infra::api_error::{self, ApiError};
use crate::infra::jwt_middleware::AuthenticatedClaims;
use crate::model::gis_location_model::{CreateGisLocation, GisLocationRawQueryResult};
use crate::model::survey_model::{CreateGisLocationResponse, HourlySurvey};
use crate::schema::gis_locations::dsl::gis_locations;
use crate::schema::gis_locations::{self as gis_locations_fields};
use crate::schema::hourly_surveys::dsl::hourly_surveys;
use crate::schema::hourly_surveys::{self as hourly_surveys_fields};
use crate::{infra::database::DbPool, model::gis_location_model::ClientCreateGisLocation};
use diesel::result::{DatabaseErrorKind, Error as diesel_error};

const SURVEY_DELAY_TIME_IN_SECONDS: i32 = 60 * 7;
const SURVEY_RANGE_TIME_IN_SECONDS: i32 = 60;
const MAX_DISTANCE_RANGE_IN_FEET: i32 = 40;
const MIN_TIME_SINCE_LAST_SURVEY_IN_SECONDS: i32 = 60 * 60;

pub async fn create_gis_location(
    pool: Arc<DbPool>,
    authenticated_claims: AuthenticatedClaims,
    client_create_gis_location: ClientCreateGisLocation,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;

    // NOTE: inserting new timestamp
    match diesel::insert_into(gis_locations)
        .values(CreateGisLocation {
            timestamp: client_create_gis_location.timestamp,
            userid: authenticated_claims.user_id,
            longitude: client_create_gis_location.longitude,
            latitude: client_create_gis_location.latitude,
        })
        .execute(database_connection)
        .await
    {
        Ok(blog_id) => blog_id,
        Err(err) => {
            error!("{err}");
            if let diesel_error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                return Ok(HttpResponse::Conflict().finish());
            }

            return Err(api_error::ApiError::DbError {
                message: "create_gis_location failed".to_string(),
            }
            .into());
        }
    };

    let end_time = client_create_gis_location.timestamp - SURVEY_DELAY_TIME_IN_SECONDS;
    let start_time = end_time - SURVEY_RANGE_TIME_IN_SECONDS;
    let latitude = client_create_gis_location.latitude;
    let longitude = client_create_gis_location.longitude;
    let user_id = authenticated_claims.user_id;
    let max_distance = MAX_DISTANCE_RANGE_IN_FEET;

    // NOTE: checking if user is in the same location he was in x seconds ago
    let res = sql_query("
        WITH time_interval AS (
            SELECT ($1) AS start_time,
                ($2) AS end_time
        ),
        distance_calculation AS (
            SELECT
                gl.id AS location_id,
                gl.userId AS user_id,
                gl.timestamp AS location_timestamp,
                earth_distance(
                    ll_to_earth($3, $4),
                    ll_to_earth(gl.latitude, gl.longitude)
                ) AS distance_in_feet
            FROM
                public.gis_locations gl
            WHERE
                gl.userId = $5 AND gl.checked = false
        )
        SELECT
            dc.location_id,
            dc.user_id,
            dc.location_timestamp as ts,
            dc.distance_in_feet
        FROM
            distance_calculation dc
        JOIN
            time_interval ti ON dc.location_timestamp > ti.start_time AND dc.location_timestamp < ti.end_time
        WHERE 
            dc.distance_in_feet < $6
        LIMIT 1;
    ");

    match res
        .bind::<Integer, _>(start_time)
        .bind::<Integer, _>(end_time)
        .bind::<Double, _>(latitude)
        .bind::<Double, _>(longitude)
        .bind::<Integer, _>(user_id)
        .bind::<Integer, _>(max_distance)
        .get_result::<GisLocationRawQueryResult>(database_connection)
        .await
    {
        Ok(res) => {
            info!("{:#?}", res);
        }
        Err(err) => {
            if let diesel_error::NotFound = err {
                return Ok(HttpResponse::Ok().json(CreateGisLocationResponse {
                    init_hourly_survey: false,
                    reason: "user not in the same location".to_string(),
                }));
            }

            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // NOTE ensuring last survey was not taken to soon
    let last_survey = match hourly_surveys
        .filter(hourly_surveys_fields::userid.eq(authenticated_claims.user_id))
        .limit(1)
        .order_by(hourly_surveys_fields::timestamp)
        .get_result::<HourlySurvey>(database_connection)
        .await
    {
        Ok(last_survey) => Some(last_survey),
        Err(err) => {
            if let diesel_error::NotFound = err {
                info!("user has never created hourly survey");
                None
            } else {
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
    };

    if let Some(last_survey) = last_survey {
        let last_survey_time = last_survey.timestamp;
        let current_time = client_create_gis_location.timestamp;

        if current_time - last_survey_time <= MIN_TIME_SINCE_LAST_SURVEY_IN_SECONDS {
            return Ok(HttpResponse::Ok().json(CreateGisLocationResponse {
                init_hourly_survey: false,
                reason: "to soon to submit repeated survey".to_string(),
            }));
        }
    } else {
        info!("user has never submitted hourly survey");
    }

    // NOTE: marking old timestamps as checked
    match diesel::update(gis_locations)
        .filter(gis_locations_fields::userid.eq(authenticated_claims.user_id))
        .set(gis_locations_fields::checked.eq(true))
        .execute(database_connection)
        .await
    {
        Ok(_) => (),
        Err(err) => {
            error!("{}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    return Ok(HttpResponse::Ok().json(CreateGisLocationResponse {
        init_hourly_survey: true,
        reason: "user is in same location and has not taken survey within min time".to_string(),
    }));
}
