use bson::DateTime;
use chrono::Utc;
use core::panic;
use log::{error, info};
use mongodb::{bson, bson::doc};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

use crate::{
    infra::collection::BaseCollection,
    model::{auth::IdMapping, uhoo_aura::UhooAura},
};

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

pub fn start_uhoo_aura_poll() {
    thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(err) => {
                error!("{err}");
                panic!("unable to start uhoo_aura poll");
            }
        };

        loop {
            let id_mappings = match rt.block_on(IdMapping::get_uhoo_aura_users()) {
                Ok(id_mappings) => id_mappings,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

            for id_mapping in id_mappings {
                let uhoo_aura = match id_mapping.sensor_auth.uhoo_aura {
                    Some(uhoo_aura) => uhoo_aura,
                    None => {
                        error!("user {} uhoo_aura was null", id_mapping.id);
                        continue;
                    },
                };

                let client_secret = uhoo_aura.client_secret;

                for device_id in uhoo_aura.device_ids {
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
                        let user_ref_id = id_mapping.id;
                        info!("uhoo_aura writing data for (user_id, device_id): ({user_ref_id}, {device_id})");

                        let bytes = match rt.block_on(response.bytes()) {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };
                        let data = match serde_json::from_slice(&bytes[..]) {
                            Ok(data) => data,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        match rt.block_on(UhooAura::add(UhooAura {
                            user_ref_id: id_mapping.id,
                            created_at: DateTime::from_chrono(Utc::now()),
                            data,
                        })) {
                            Ok(_) => (),
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}
