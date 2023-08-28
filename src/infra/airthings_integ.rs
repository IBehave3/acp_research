use bson::{DateTime, Document};
use chrono::Utc;
use log::{error, info};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, Scope, StandardTokenResponse,
    TokenResponse, TokenUrl,
};
use reqwest::{Client, Response};
use std::thread;
use std::time::Duration;

use crate::model::airthings::Airthings;
use crate::infra::collection::BaseCollection;
use crate::model::auth::IdMapping;

const QUERY_FREQ_SECS: u64 = 300;

pub async fn get_token(
    client_id: &str,
    client_secret: &str,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Box<dyn std::error::Error>>
{
    let access_token_url = "https://accounts-api.airthings.com/v1/token";

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new(access_token_url.to_string())?,
        Some(TokenUrl::new(access_token_url.to_string())?),
    );

    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new("read:device:current_values".to_string()))
        .request_async(async_http_client)
        .await?;

    Ok(token_result)
}

pub async fn get_device_data(
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    group_id: &str,
    dev_serial: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    let device_info_base_url = "https://ext-api.airthings.com/v1/devices/";

    let response = Client::new()
        .get(format!(
            "{device_info_base_url}{dev_serial}/latest-samples?userGroupId={group_id}"
        ))
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
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(err) => {
                error!("{err}");
                panic!("unable to start airthings poll");
            }
        };

        loop {
            let id_mappings = match rt.block_on(IdMapping::get_airthings_users()) {
                Ok(id_mappings) => id_mappings,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

            for id_mapping in id_mappings {
                let airthings = match id_mapping.airthings {
                    Some(airthings) => airthings,
                    None => {
                        error!("user {} airthings was null", id_mapping.id);
                        continue;
                    },
                };

                let client_id = airthings.client_id;
                let client_secret = airthings.client_secret;
                let group_id = airthings.group_id;

                for device_id in airthings.device_ids{
                    let token = match rt.block_on(get_token(&client_id, &client_secret)) {
                        Ok(token) => token,
                        Err(err) => {
                            error!("{err}");
                            continue;
                        }
                    };
                    let response = match rt.block_on(get_device_data(token, &group_id, &device_id))
                    {
                        Ok(response) => response,
                        Err(err) => {
                            error!("{err}");
                            continue;
                        }
                    };

                    if response.status() == 200 {
                        let user_ref_id = id_mapping.id;
                        info!("airthings writing data for (user_id, device_id): ({user_ref_id}, {device_id})");

                        let bytes = match rt.block_on(response.bytes()) {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };
                        let data: Document  = match serde_json::from_slice(&bytes[..]) {
                            Ok(data) => data,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        let airthings = Airthings {
                            user_ref_id: id_mapping.id,
                            created_at: DateTime::from_chrono(Utc::now()),
                            data,
                        };

                        match rt.block_on(Airthings::add(airthings)) {
                            Ok(_) => (),
                            Err(err) => {
                                error!("{err}");
                                
                            }
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}
