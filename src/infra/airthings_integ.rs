use log::{error, info};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, Scope, StandardTokenResponse,
    TokenResponse, TokenUrl,
};
use reqwest::{Client, Response};
use std::sync::Arc;
use std::thread;
use std::time::Duration;


use crate::controller::{user_controller, airthings_controller};
use crate::model::airthings_model::ClientAirthings;

use super::database::DbPool;

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
        info!("token has expired");
    }

    Ok(response)
}

pub fn start_airthings_poll(pool: Arc<DbPool>) {
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
            let connection = &mut rt.block_on(pool.get()).unwrap();

            let user_airthings: Vec<crate::model::user_model::UserAirthings> = match rt.block_on(user_controller::get_airthings_users(connection)) {
                Ok(users) => users,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

            for user_airthing in user_airthings {
                let device_ids = match user_airthing.deviceids {
                    Some(device_ids) => device_ids,
                    None => {
                        continue;
                    }
                };
                let client_id = user_airthing.clientid;
                let client_secret = user_airthing.clientsecret;
                let group_id = user_airthing.groupid;

                for device_id in device_ids {
                    let device_id = match device_id {
                        Some(device_id) => device_id,
                        None => {
                            continue;
                        }
                    };

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
                        let user_ref_id = user_airthing.userid;
                        info!("airthings writing data for (user_id, device_id): ({user_ref_id}, {device_id})");

                        let bytes = match rt.block_on(response.bytes()) {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                error!("retrieving bytes {err}");
                                continue;
                            }
                        };
                        let client_airthings: ClientAirthings  = match serde_json::from_slice(&bytes[..]) {
                            Ok(data) => data,
                            Err(err) => {
                                error!("serde_json {err}");
                                continue;
                            }
                        };

                        if let Err(err) = rt.block_on(airthings_controller::create_airthings(connection, client_airthings, user_ref_id, device_id)) {
                            error!("create_airthings {err}");
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}
