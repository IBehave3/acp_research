use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{schema::airthings::dsl::airthings, model::airthings_model::{ClientAirthings, CreateAirthings}};

pub async fn create_airthings(connection: &mut AsyncPgConnection, client_airthings: ClientAirthings, user_id: i32, device_id: String) -> anyhow::Result<()> {
    diesel::insert_into(airthings)
    .values(CreateAirthings {
        userid: user_id,
        deviceid: device_id,
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