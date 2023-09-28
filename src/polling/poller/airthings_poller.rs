
use log::{info, error};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, Scope, StandardTokenResponse,
    TokenResponse, TokenUrl,
};
use reqwest::{Client, Response};

use crate::api::model::user_model::UserAirthings;
use crate::api::controller::user_controller;
use crate::polling::controller::airthings_controller;
use crate::polling::infra::database::CONNECTION_POOL;
use crate::polling::model::airthings_model::ClientAirthings;

pub async fn get_token(
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>
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
) -> anyhow::Result<Response> {
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

pub async fn airthings_poll() {
    if let Err(err) = exec_airthings_poll().await {
        error!("{err}");
    }
}

pub async fn exec_airthings_poll() -> anyhow::Result<()> {   
    info!("polling airthings users");

    let mut conn = CONNECTION_POOL
        .get()
        .ok_or(anyhow::anyhow!("Connection pool not found"))?
        .get()
        .await?;
    let user_airthings: Vec<UserAirthings> = user_controller::get_airthings_users(&mut conn).await?;

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

            let token = get_token(&client_id, &client_secret).await?;
            let response = get_device_data(token, &group_id, &device_id).await?;

            if response.status() == 200 {
                let user_ref_id = user_airthing.userid;
                info!("airthings writing data for (user_id, device_id): ({user_ref_id}, {device_id})");

                let bytes = response.bytes().await?;
                let client_airthings: ClientAirthings  = serde_json::from_slice(&bytes[..])?;

                let mut conn = CONNECTION_POOL
                    .get()
                    .ok_or(anyhow::anyhow!("Connection pool not found"))?
                    .get()
                    .await?;
                airthings_controller::create_airthings(&mut conn, client_airthings, user_ref_id, device_id).await?;
            }
        }
    }

    Ok(())
}
