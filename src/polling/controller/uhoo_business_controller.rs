use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{schema::uhoo_business::dsl::uhoo_business, polling::model::uhoo_business_model::{ClientUhooBusiness, CreateUhooBusiness}};

pub async fn create_uhoo_business(connection: &mut AsyncPgConnection, client_uhoo_business: ClientUhooBusiness, user_id: i32, device_id: String) -> anyhow::Result<()> {
    let data = match client_uhoo_business.data.get(0) {
        Some(data) => data,
        None => {
            return Err(anyhow::format_err!("no data found for uhoo business"));
        }
    };

    diesel::insert_into(uhoo_business)
    .values(CreateUhooBusiness {
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
        temperatureunit: client_uhoo_business.usersettings.temperature,
        tempunit: client_uhoo_business.usersettings.temp,
        humidityunit: client_uhoo_business.usersettings.humidity,
        pm25unit: client_uhoo_business.usersettings.pm25,
        dustunit: client_uhoo_business.usersettings.dust,
        tvocunit: client_uhoo_business.usersettings.tvoc,
        vocunit: client_uhoo_business.usersettings.voc,
        co2unit: client_uhoo_business.usersettings.co2,
        counit: client_uhoo_business.usersettings.co,
        airpressureunit: client_uhoo_business.usersettings.airpressure,
        pressureunit: client_uhoo_business.usersettings.pressure,
        ozoneunit: client_uhoo_business.usersettings.ozone,
        no2unit: client_uhoo_business.usersettings.no2,
        pm1unit: client_uhoo_business.usersettings.pm1,
        pm4unit: client_uhoo_business.usersettings.pm4,
        pm10unit: client_uhoo_business.usersettings.pm10,
        ch2ounit: client_uhoo_business.usersettings.ch2o,
        lightunit: client_uhoo_business.usersettings.light,
        h2sunit: client_uhoo_business.usersettings.h2s,
        nounit: client_uhoo_business.usersettings.no,
        so2unit: client_uhoo_business.usersettings.so2,
        nh3unit: client_uhoo_business.usersettings.nh3,
        oxygenunit: client_uhoo_business.usersettings.oxygen,
    })
    .execute(connection)
    .await?;

    Ok(())
}