use std::str::FromStr;

use chrono::DateTime;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{schema::keychains::dsl::keychains, model::keychain_model::{ClientKeychain, CreateKeychain}};

pub async fn create_keychain(connection: &mut AsyncPgConnection, client_keychain: ClientKeychain, user_id: i32, device_mac: &str) -> anyhow::Result<()> {

    for item in client_keychain.data.items {
        let time = DateTime::from_str(&item.time)?;

        diesel::insert_into(keychains)
        .values(CreateKeychain {
            userid: user_id,
            time,
            devmac: device_mac.to_owned(),
            voc: item.voc,
            pm1: item.pm1,
            pm25: item.pm25,
            pm10: item.pm10,
            t: item.t,
            h: item.h,
            p: item.p,
            lat: item.coords.lat,
            lon: item.coords.lon,
        })
        .execute(connection)
        .await?;
    }

    Ok(())
}