
use log::{error, info};

use reqwest::{Client, Response};




use crate::api::controller::user_controller;
use crate::polling::controller::gray_wolf_controller;
use crate::polling::infra::database::CONNECTION_POOL;
use crate::polling::model::gray_wolf_model::ClientGrayWolf;

const QUERY_FREQ_SECS: u64 = 60;

pub async fn get_device_data(
    api_key: &str,
    dev_id: &str,
) -> anyhow::Result<Response> {
    let info_url = format!("https://graywolfliveapi.com/api/LiveReadings/?parameters={{\"apikey\":\"{api_key}\",\"deviceID\":\"{dev_id}\"}}");

    let response = Client::new().get(info_url).send().await?;

    Ok(response)
}

pub async fn gray_wolf_poll() {
    if let Err(err) = exec_gray_wolf_poll().await {
        error!("{err}");
    }
}

pub async fn exec_gray_wolf_poll() -> anyhow::Result<()> {
    info!("polling gray_wolf users");
    
    let mut conn = CONNECTION_POOL
        .get()
        .ok_or(anyhow::anyhow!("Connection pool not found"))?
        .get()
        .await?;
    let user_gray_wolfs = user_controller::get_gray_wolf_users(&mut conn).await?;

    for user_gray_wolf in user_gray_wolfs {
        let device_ids = match user_gray_wolf.deviceids {
            Some(device_ids) => device_ids,
            None => {
                continue;
            }
        };

        let api_key = user_gray_wolf.apikey;

        for device_id in device_ids {
            let device_id = match device_id {
                Some(device_id) => device_id,
                None => {
                    continue;
                }
            };

            let response = get_device_data(&api_key, &device_id).await?;

            if response.status() == 200 {
                let user_ref_id = user_gray_wolf.userid;
                info!(
                    "gray_wolf writing data for (user_id, device_id): ({user_ref_id}, {device_id})"
                );
 
                let string_json = response.text().await?;

                let string: String = match serde_json::from_str(&string_json) {
                    Ok(string) => string,
                    Err(err) => {
                        error!("{err}");
                        continue;
                    }
                };

                let client_gray_wolf: ClientGrayWolf = serde_json::from_str(&string)?;

                let mut conn = CONNECTION_POOL
                .get()
                .ok_or(anyhow::anyhow!("Connection pool not found"))?
                .get()
                .await?;
                gray_wolf_controller::create_gray_wolf(
                    &mut conn,
                    client_gray_wolf,
                    user_ref_id,
                    device_id,
                ).await?;
            }
        }
    }

    Ok(())
}
