use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{model::airthings_model::{ClientAirthings, CreateAirthings}};
use crate::schema::airthings::dsl::airthings;

pub async fn create_airthings(connection: &mut AsyncPgConnection, client_airthings: ClientAirthings, user_id: i32) -> anyhow::Result<()> {
    diesel::insert_into(airthings)
    .values(CreateAirthings {
        userid: user_id,
        battery: client_airthings.data.battery,
        co2: client_airthings.data.co2,
        humidity: client_airthings.data.humidity,
        pm1: client_airthings.data.pm1,
        pm25: client_airthings.data.pm25,
        pressure: client_airthings.data.pressure,
        radonshorttermavg: client_airthings.data.radonshorttermavg,
        temp: client_airthings.data.temp,
        time: client_airthings.data.time,
        voc: client_airthings.data.voc,
        relaydevicetype: client_airthings.data.relaydevicetype,
    })
    .execute(connection)
    .await?;

    Ok(())
}