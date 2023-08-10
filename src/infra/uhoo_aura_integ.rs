use chrono::Utc;
use reqwest::{Client, Response};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::thread;
use mongodb::{bson, bson::doc, bson::oid::ObjectId};

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

    let access_token_response: AccessTokenResponse = serde_json::from_slice(&response.bytes().await.unwrap()[..]).unwrap();

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
        let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build().unwrap();

        loop {
            let id_mappings = rt.block_on(IdMapping::get_airthings_users()).unwrap();

            for id_mapping in id_mappings {
                for device_id_mapping in id_mapping.data_structure_device_id_mapping {
                    if device_id_mapping.data_structure_id.eq("uhoo_aura") {
                        let client_secret = device_id_mapping.auth.as_ref().unwrap().client_secret.clone().unwrap();

                        for device_id in device_id_mapping.device_ids.unwrap() {
                            let token = rt.block_on(get_token(&client_secret)).unwrap();
                            let response = rt.block_on(get_device_data(&token.access_token, &device_id)).unwrap();

                            if response.status() == 200 {
                                println!("writing uhoo_aura to push_data");
                                let bytes = rt.block_on(response.bytes()).unwrap();
                                rt.block_on(PushData::add(PushData {
                                    _id: ObjectId::new(),
                                    device_id: Some(device_id),
                                    created_at: bson::DateTime::from_chrono(Utc::now()),
                                    data_structure_id: "uhoo_aura".to_string(),
                                    id_mapping_ref_id: id_mapping._id,
                                    data: serde_json::from_slice(&bytes[..]).unwrap(),
                                })).unwrap();
                            }
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}