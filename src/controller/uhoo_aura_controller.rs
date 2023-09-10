use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{schema::uhoo_auras::dsl::uhoo_auras, model::uhoo_aura_model::{ClientUhooAura, CreateUhooAura}};

pub async fn create_uhoo_aura(connection: &mut AsyncPgConnection, client_uhoo_aura: ClientUhooAura, user_id: i32, device_id: String) -> anyhow::Result<()> {
    let data = match client_uhoo_aura.data.get(0) {
        Some(data) => data,
        None => {
            return Err(anyhow::format_err!("no data found for uhoo aura"));
        }
    };

    diesel::insert_into(uhoo_auras)
    .values(CreateUhooAura {
        userid: user_id,
        deviceid: device_id,
        virusindex: data.virusindex,
        temperature: data.temperature,
        humidity: data.humidity,
        pm25: data.pm25,
        tvoc: data.tvoc,
        co2: data.co2,
        airpressure: data.airpressure,
        ozone: data.ozone,
        no2: data.no2,
        pm1: data.pm1,
        pm4: data.pm4,
        pm10: data.pm10,
        ch2o: data.ch2o,
        light: data.light,
        sound: data.sound,
        h2s: data.h2s,
        no: data.no,
        so2: data.so2,
        nh3: data.nh3,
        oxygen: data.oxygen,
        timestamp: data.timestamp,
        temperatureunit: client_uhoo_aura.usersettings.temperature,
        tempunit: client_uhoo_aura.usersettings.temp,
        humidityunit: client_uhoo_aura.usersettings.humidity,
        pm25unit: client_uhoo_aura.usersettings.pm25,
        dustunit: client_uhoo_aura.usersettings.dust,
        tvocunit: client_uhoo_aura.usersettings.tvoc,
        vocunit: client_uhoo_aura.usersettings.voc,
        co2unit: client_uhoo_aura.usersettings.co2,
        counit: client_uhoo_aura.usersettings.co,
        airpressureunit: client_uhoo_aura.usersettings.airpressure,
        pressureunit: client_uhoo_aura.usersettings.pressure,
        ozoneunit: client_uhoo_aura.usersettings.ozone,
        no2unit: client_uhoo_aura.usersettings.no2,
        pm1unit: client_uhoo_aura.usersettings.pm1,
        pm4unit: client_uhoo_aura.usersettings.pm4,
        pm10unit: client_uhoo_aura.usersettings.pm10,
        ch2ounit: client_uhoo_aura.usersettings.ch2o,
        lightunit: client_uhoo_aura.usersettings.light,
        h2sunit: client_uhoo_aura.usersettings.h2s,
        nounit: client_uhoo_aura.usersettings.no,
        so2unit: client_uhoo_aura.usersettings.so2,
        nh3unit: client_uhoo_aura.usersettings.nh3,
        oxygenunit: client_uhoo_aura.usersettings.oxygen,
    })
    .execute(connection)
    .await?;

    Ok(())
}