use chrono::Datelike;
use log::{error, info};
use reqwest::{Client, Response};
use urlencoding::encode;

use crate::api::controller::user_controller;
use crate::api::model::user_model::UserKeychain;
use crate::polling::controller::keychain_controller;

use crate::polling::infra::database::CONNECTION_POOL;
use crate::polling::model::keychain_model::ClientKeychain;

pub async fn get_device_data(api_key: &str, dev_mac: &str) -> anyhow::Result<Response> {
    let current_date = chrono::Utc::now();
    let url = format!(
        "https://api.atmotube.com/api/v1/data?api_key={}&mac={}&start_date={}-{}-{}",
        api_key,
        encode(dev_mac),
        current_date.year(),
        current_date.month(),
        current_date.day()
    );

    let response = Client::new().get(url).send().await?;

    Ok(response)
}

pub async fn keychain_poll() {
    if let Err(err) = exec_keychain_poll().await {
        error!("{err}");
    }
}

pub async fn exec_keychain_poll() -> anyhow::Result<()> {
    info!("polling keychain users");

    let mut conn = CONNECTION_POOL
        .get()
        .ok_or(anyhow::anyhow!("Connection pool not found"))?
        .get()
        .await?;
    let user_keychains: Vec<UserKeychain> = user_controller::get_keychain_users(&mut conn).await?;

    for user_keychain in user_keychains {
        let device_macs = match user_keychain.devicemacs {
            Some(device_macs) => device_macs,
            None => {
                continue;
            }
        };

        let api_key = user_keychain.apikey;

        for device_mac in device_macs {
            let device_mac = match device_mac {
                Some(device_mac) => device_mac,
                None => {
                    continue;
                }
            };
            let response = get_device_data(&api_key, &device_mac).await?;

            if response.status() == 200 {
                let user_ref_id = user_keychain.userid;
                info!("keychain writing data for (user_id, device_mac): ({user_ref_id}, {device_mac})");

                let bytes = response.bytes().await?;
                let client_keychain: ClientKeychain = match serde_json::from_slice(&bytes[..]) {
                    Ok(data) => data,
                    Err(err) => {
                        error!("{err}");
                        continue;
                    }
                };

                let mut conn = CONNECTION_POOL
                    .get()
                    .ok_or(anyhow::anyhow!("Connection pool not found"))?
                    .get()
                    .await?;
                keychain_controller::create_keychain(
                    &mut conn,
                    client_keychain,
                    user_ref_id,
                    &device_mac,
                )
                .await?;
            }
        }
    }

    Ok(())
}
