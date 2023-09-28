
use log::{error, info};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};




use crate::api::controller::user_controller;
use crate::api::model::user_model::UserUhooHome;
use crate::polling::controller::uhoo_home_controller;
use crate::polling::infra::database::CONNECTION_POOL;
use crate::polling::model::uhoo_home_model::ClientUhooHome;

const QUERY_FREQ_SECS: u64 = 60;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub async fn get_token(client_secret: &str) -> anyhow::Result<AccessTokenResponse> {
    let params = [("code", client_secret)];
    let access_token_url = "https://api.uhooinc.com/v1/generatetoken";

    let response = Client::new()
        .post(access_token_url)
        .form(&params)
        .send()
        .await?;

    let bytes = &response.bytes().await?[..];
    let access_token_response: AccessTokenResponse = serde_json::from_slice(bytes)?;

    Ok(access_token_response)
}

pub async fn get_device_data(token: &str, dev_serial: &str) -> anyhow::Result<Response> {
    let device_info_base_url = "https://api.uhooinc.com/v1/devicedata";
    let params = [("macAddress", dev_serial), ("mode", "minute")];

    let response = Client::new()
        .post(device_info_base_url)
        .bearer_auth(token)
        .form(&params)
        .send()
        .await?;

    Ok(response)
}

pub async fn uhoo_home_poll() {
    if let Err(err) = exec_uhoo_home_poll().await {
        error!("{err}");
    }
}

pub async fn exec_uhoo_home_poll() -> anyhow::Result<()> {
    info!("polling uhoo_home users");

    let mut conn = CONNECTION_POOL
        .get()
        .ok_or(anyhow::anyhow!("Connection pool not found"))?
        .get()
        .await?;
    let user_uhoo_homes: Vec<UserUhooHome> =
        user_controller::get_uhoo_home_users(&mut conn).await?;

    for user_uhoo_home in user_uhoo_homes {
        let device_ids = match user_uhoo_home.deviceids {
            Some(device_ids) => device_ids,
            None => {
                continue;
            }
        };

        let client_secret = user_uhoo_home.clientsecret;

        for device_id in device_ids {
            let device_id = match device_id {
                Some(device_id) => device_id,
                None => {
                    continue;
                }
            };
            let token = get_token(&client_secret).await?;

            let response = get_device_data(&token.access_token, &device_id).await?;

            if response.status() == 200 {
                let user_ref_id = user_uhoo_home.userid;
                info!(
                    "uhoo_home writing data for (user_id, device_id): ({user_ref_id}, {device_id})"
                );

                let bytes = response.bytes().await?;

                let client_uhoo_home: ClientUhooHome = match serde_json::from_slice(&bytes[..]) {
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
                uhoo_home_controller::create_uhoo_home(
                    &mut conn,
                    client_uhoo_home,
                    user_ref_id,
                    device_id,
                )
                .await?;
            }
        }
    }

    Ok(())
}
