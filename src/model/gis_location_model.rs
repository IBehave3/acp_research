use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

/*
WITH time_interval AS (
    SELECT NOW() - interval ':y minutes' AS start_time,
           NOW() - interval ':x minutes' AS end_time
),
distance_calculation AS (
    SELECT
        gl.id AS location_id,
        gl.userId AS user_id,
        gl.timestamp AS location_timestamp,
        earth_distance(
            ll_to_earth(:input_latitude, :input_longitude),
            ll_to_earth(gl.latitude, gl.longitude)
        ) AS distance_in_feet
    FROM
        gis_locations gl
    JOIN
        time_interval ti ON gl.timestamp BETWEEN ti.start_time AND ti.end_time
    WHERE
        gl.userId = :input_user_id
)

SELECT
    dc.location_id,
    dc.user_id,
    dc.location_timestamp,
    dc.distance_in_feet
FROM
    distance_calculation dc
WHERE
    dc.distance_in_feet > :z
    AND dc.location_id = (
        SELECT id
        FROM distance_calculation
        ORDER BY location_timestamp DESC
        LIMIT 1
    );
*/
/*
    The provided query returns a result set with the following columns:

    location_id: The unique identifier (ID) of the location record.
    user_id: The ID of the user associated with the location.
    location_timestamp: The timestamp of the location record.
    distance_in_feet: The calculated distance in feet between the user's
    location at the given latitude and longitude and the input latitude and longitude.

    The query filters the results based on the following conditions:

    It considers only location records within a specified time interval, which is
    defined by the :x and :y parameters, relative to the current time.

    It checks if the calculated distance (distance_in_feet) between the user's location
    and the input point (:input_latitude and :input_longitude) is greater than the
    threshold distance specified by the :z parameter.

    It selects the most recent location record within the specified time interval and
    distance threshold for the specified userId.

    The result will include one row for the most recent location record that meets the
    criteria for the given userId. This row will contain the location details
    (ID, user ID, timestamp, and distance) for that specific location record. If no such
    record exists within the specified criteria, the query will return no rows.
*/

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::gis_locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateGisLocation{
    pub timestamp: DateTime<Utc>,
    pub userid: i32,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateGisLocation {
    pub timestamp: DateTime<Utc>,
    pub longitude: f64,
    pub latitude: f64,
}
