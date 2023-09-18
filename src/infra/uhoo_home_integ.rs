use core::panic;
use std::sync::Arc;
use log::{error, info};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

use crate::controller::{user_controller, uhoo_home_controller};
use crate::model::uhoo_home_model::ClientUhooHome;
use crate::model::user_model::UserUhooHome;

use super::database::DbPool;

const QUERY_FREQ_SECS: u64 = 60;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub async fn get_token(
    client_secret: &str,
) -> Result<AccessTokenResponse, Box<dyn std::error::Error>> {
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

pub async fn get_device_data(
    token: &str,
    dev_serial: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
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

pub fn start_uhoo_home_poll(pool: Arc<DbPool>) {
    thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(err) => {
                error!("{err}");
                panic!("unable to start uhoo_home poll");
            }
        };

        loop {
            let connection = &mut rt.block_on(pool.get()).unwrap();

            let user_uhoo_homes: Vec<UserUhooHome> = match rt.block_on(user_controller::get_uhoo_home_users(connection)) {
                Ok(users) => users,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

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
                    let token = match rt.block_on(get_token(&client_secret)) {
                        Ok(token) => token,
                        Err(err) => {
                            error!("{err}");
                            continue;
                        }
                    };

                    let response =
                        match rt.block_on(get_device_data(&token.access_token, &device_id)) {
                            Ok(response) => response,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                    if response.status() == 200 {
                        let user_ref_id = user_uhoo_home.userid;
                        info!("uhoo_home writing data for (user_id, device_id): ({user_ref_id}, {device_id})");

                        let bytes = match rt.block_on(response.bytes()) {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        let client_uhoo_home: ClientUhooHome  = match serde_json::from_slice(&bytes[..]) {
                            Ok(data) => data,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        if let Err(err) = rt.block_on(uhoo_home_controller::create_uhoo_home(connection, client_uhoo_home, user_ref_id, device_id)) {
                            error!("uhoo_home db error {err}");
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}
