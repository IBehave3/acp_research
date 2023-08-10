use chrono::Utc;
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    Scope,
    TokenUrl, StandardTokenResponse, EmptyExtraTokenFields, TokenResponse
};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use reqwest::{Client, Response};
use std::time::Duration;
use std::thread;
use mongodb::{bson, bson::oid::ObjectId};

use crate::infra::collection::BaseCollection;
use crate::model::auth::IdMapping;
use crate::model::push_data::PushData;

const QUERY_FREQ_SECS: u64 = 300; 

pub async fn get_token(client_id: &str, client_secret: &str) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Box<dyn std::error::Error>> {
    let access_token_url = "https://accounts-api.airthings.com/v1/token";

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new(access_token_url.to_string())?,
        Some(TokenUrl::new(access_token_url.to_string())?)
    );

    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new("read:device:current_values".to_string()))
        .request_async(async_http_client).await?;

    Ok(token_result)
}

pub async fn get_device_data(token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, group_id: &str, dev_serial: &str) -> Result<Response, Box<dyn std::error::Error>>{
    let device_info_base_url = "https://ext-api.airthings.com/v1/devices/";

    let response = Client::new().get(format!("{device_info_base_url}{dev_serial}/latest-samples?userGroupId={group_id}"))
                                        .bearer_auth(token.access_token().secret())
                                        .send()
                                        .await?;
    
    if response.status() == 401 {
        println!("token has expired");
    }

    Ok(response)
}  

pub fn start_airthings_poll() {
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build().unwrap();

        loop {
            let id_mappings = rt.block_on(IdMapping::get_airthings_users()).unwrap();

            for id_mapping in id_mappings {
                for device_id_mapping in id_mapping.data_structure_device_id_mapping {
                    if device_id_mapping.data_structure_id.eq("airthings") {
                        let client_id = device_id_mapping.auth.as_ref().unwrap().client_id.clone().unwrap();
                        let client_secret = device_id_mapping.auth.as_ref().unwrap().client_secret.clone().unwrap();
                        let group_id = device_id_mapping.auth.as_ref().unwrap().group_id.clone().unwrap();

                        for device_id in device_id_mapping.device_ids.unwrap() {
                            let token = rt.block_on(get_token(&client_id, &client_secret)).unwrap();
                            let response = rt.block_on(get_device_data(token, &group_id, &device_id)).unwrap();

                            if response.status() == 200 {
                                println!("writing aithings_data to push_data");
                                let bytes = rt.block_on(response.bytes()).unwrap();
                                rt.block_on(PushData::add(PushData {
                                    _id: ObjectId::new(),
                                    device_id: Some(device_id),
                                    created_at: bson::DateTime::from_chrono(Utc::now()),
                                    data_structure_id: "airthings".to_string(),
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