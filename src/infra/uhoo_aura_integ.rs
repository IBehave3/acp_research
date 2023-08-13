use chrono::Utc;
use reqwest::{Client, Response};
use serde::{Serialize, Deserialize};
use core::panic;
use std::time::Duration;
use std::thread;
use mongodb::{bson, bson::doc, bson::oid::ObjectId};
use log::{info, error};

use crate::{model::{auth::IdMapping, push_data::PushData}, infra::collection::BaseCollection};

const QUERY_FREQ_SECS: u64 = 60; 

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64
}

pub async fn get_token(client_secret: &str) -> Result<AccessTokenResponse, Box<dyn std::error::Error>> {
    let params = [("code", client_secret)];
    let access_token_url = "https://api.uhooinc.com/v1/generatetoken";

    let response = Client::new().post(access_token_url)
                    .form(&params)
                    .send()
                    .await?;

    let bytes = &response.bytes().await?[..];
    let access_token_response: AccessTokenResponse = serde_json::from_slice(bytes)?;

    Ok(access_token_response)
}

pub async fn get_device_data(token: &str, dev_serial: &str) -> Result<Response, Box<dyn std::error::Error>>{
    let device_info_base_url = "https://api.uhooinc.com/v1/devicedata";
    let params = [("macAddress", dev_serial), ("mode", "minute")];

    let response = Client::new().post(device_info_base_url)
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
            .build() {
                Ok(rt) => rt,
                Err(err) => {
                    error!("{err}");
                    panic!("unable to start uhoo_aura poll");
                }
            };

        loop {
            let id_mappings = match rt.block_on(IdMapping::get_airthings_users()) {
                Ok(id_mappings) => id_mappings,
                Err(err) => {
                    error!("{err}");
                    continue;
                },
            };

            for id_mapping in id_mappings {
                for device_id_mapping in id_mapping.data_structure_device_id_mapping {
                    if device_id_mapping.data_structure_id.eq("uhoo_aura") {
                        let device_id_mapping_ref = match device_id_mapping.auth.as_ref() {
                            Some(device_id_mapping_ref) => device_id_mapping_ref,
                            None => {
                                error!("no uhoo_ara data structure found for user");
                                continue;
                            }
                        };
                        let client_secret = match device_id_mapping_ref.client_secret.clone() {
                            Some(client_secret) => client_secret,
                            None => {
                                error!("no client secret found for uhoo_aura");
                                continue;
                            }
                        };

                        let device_ids = match device_id_mapping.device_ids {
                            Some(device_ids) => device_ids,
                            None => {
                                error!("no device ids found for uhoo_aura");
                                continue;
                            } 
                        };

                        for device_id in device_ids {
                            let token = match rt.block_on(get_token(&client_secret)) {
                                Ok(token) => token,
                                Err(err) => {
                                    error!("{err}");
                                    continue;
                                }
                            };
                            let response = match rt.block_on(get_device_data(&token.access_token, &device_id)) {
                                Ok(response) => response,
                                Err(err) => {
                                    error!("{err}");
                                    continue;
                                }
                            };

                            if response.status() == 200 {
                                let user_ref_id = id_mapping._id;
                                info!("uhoo_aura writing data for (user_id, device_id): ({user_ref_id}, {device_id})");

                                let bytes = match rt.block_on(response.bytes()) {
                                    Ok(bytes) => bytes,
                                    Err(err) => {
                                        error!("{err}");
                                        continue;
                                    }
                                };
                                let json = match serde_json::from_slice(&bytes[..]) {
                                    Ok(json) => json,
                                    Err(err) => {
                                        error!("{err}");
                                        continue;
                                    }
                                };
                                
                                match rt.block_on(PushData::add(PushData {
                                    _id: ObjectId::new(),
                                    device_id: Some(device_id),
                                    created_at: bson::DateTime::from_chrono(Utc::now()),
                                    data_structure_id: "uhoo_aura".to_string(),
                                    id_mapping_ref_id: user_ref_id,
                                    data: json,
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
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}