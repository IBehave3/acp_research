use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{schema::uhoo_homes::dsl::uhoo_homes, model::uhoo_home_model::{ClientUhooHome, CreateUhooHome}};

pub async fn create_uhoo_home(connection: &mut AsyncPgConnection, client_uhoo_homes: ClientUhooHome, user_id: i32, device_id: String) -> anyhow::Result<()> {
    let data = match client_uhoo_homes.data.get(0) {
        Some(data) => data,
        None => {
            return Err(anyhow::format_err!("no data found for uhoo home"));
        }
    };

    diesel::insert_into(uhoo_homes)
    .values(CreateUhooHome {
        userid: user_id,
        deviceid: device_id,
        virusindex: data.virusindex,
        temperature: data.temperature,
        humidity: data.humidity,
        pm25: data.pm25,
        tvoc: data.tvoc,
        co2: data.co2,
        co: data.co,
        airpressure: data.airpressure,
        ozone: data.ozone,
        no2: data.no2,

        timestamp: data.timestamp,
        temperatureunit: client_uhoo_homes.usersettings.temperature,
        tempunit: client_uhoo_homes.usersettings.temp,
        humidityunit: client_uhoo_homes.usersettings.humidity,
        pm25unit: client_uhoo_homes.usersettings.pm25,
        dustunit: client_uhoo_homes.usersettings.dust,
        tvocunit: client_uhoo_homes.usersettings.tvoc,
        vocunit: client_uhoo_homes.usersettings.voc,
        co2unit: client_uhoo_homes.usersettings.co2,
        counit: client_uhoo_homes.usersettings.co,
        airpressureunit: client_uhoo_homes.usersettings.airpressure,
        pressureunit: client_uhoo_homes.usersettings.pressure,
        ozoneunit: client_uhoo_homes.usersettings.ozone,
        no2unit: client_uhoo_homes.usersettings.no2,
        pm1unit: client_uhoo_homes.usersettings.pm1,
        pm4unit: client_uhoo_homes.usersettings.pm4,
        pm10unit: client_uhoo_homes.usersettings.pm10,
        ch2ounit: client_uhoo_homes.usersettings.ch2o,
        lightunit: client_uhoo_homes.usersettings.light,
        h2sunit: client_uhoo_homes.usersettings.h2s,
        nounit: client_uhoo_homes.usersettings.no,
        so2unit: client_uhoo_homes.usersettings.so2,
        nh3unit: client_uhoo_homes.usersettings.nh3,
        oxygenunit: client_uhoo_homes.usersettings.oxygen,
    })
    .execute(connection)
    .await?;

    Ok(())
}