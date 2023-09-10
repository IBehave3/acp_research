


use diesel_async::{AsyncPgConnection, RunQueryDsl};


use crate::model::gray_wolf_model::{ClientGrayWolf, CreateGrayWolf, CreateGrayWolfSensor, GrayWolf};
use crate::schema::gray_wolf_sensors::dsl::gray_wolf_sensors;
use crate::schema::gray_wolfs::dsl::gray_wolfs;

pub async fn create_gray_wolf(
    connection: &mut AsyncPgConnection,
    client_gray_wolf: ClientGrayWolf,
    user_id: i32,
    device_id: String,
) -> anyhow::Result<()> {
    let gray_wolf: GrayWolf = diesel::insert_into(gray_wolfs)
        .values(CreateGrayWolf {
            userid: user_id,
            deviceid: device_id,
            version: client_gray_wolf.version,
            generator: client_gray_wolf.generator,
            api: client_gray_wolf.api,
            error: client_gray_wolf.error,
            battery: client_gray_wolf.battery,
            status: client_gray_wolf.status,
            serialnumber: client_gray_wolf.serial_number,
            timestamp: client_gray_wolf.timestamp,
        })
        .get_result::<GrayWolf>(connection)
        .await?;

    for gray_wolf_sensor in client_gray_wolf.data {
        diesel::insert_into(gray_wolf_sensors)
            .values(CreateGrayWolfSensor {
                graywolfsid: gray_wolf.id,
                sensor: gray_wolf_sensor.sensor,
                unit: gray_wolf_sensor.unit,
                value: gray_wolf_sensor.value,
                sensorid: gray_wolf_sensor.id,
                status: gray_wolf_sensor.status,
            })
            .execute(connection)
            .await?;
    }

    Ok(())
}
