use core::panic;
use log::{error, info};

use reqwest::{Client, Response};

use std::{thread, sync::Arc};
use std::time::Duration;

use crate::controller::{user_controller, gray_wolf_controller};
use crate::model::gray_wolf_model::ClientGrayWolf;

use super::database::DbPool;

const QUERY_FREQ_SECS: u64 = 60;

pub async fn get_device_data(
    api_key: &str,
    dev_id: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    let info_url = format!("https://graywolfliveapi.com/api/LiveReadings/?parameters={{\"apikey\":\"{api_key}\",\"deviceID\":\"{dev_id}\"}}");

    let response = Client::new().get(info_url).send().await?;

    Ok(response)
}

pub fn start_gray_wolf_poll(pool: Arc<DbPool>) {
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
            let connection = &mut rt.block_on(pool.get()).unwrap();

            let user_gray_wolfs = match rt.block_on(user_controller::get_gray_wolf_users(connection)) {
                Ok(user_gray_wolfs) => user_gray_wolfs,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

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

                    let response = match rt.block_on(get_device_data(&api_key, &device_id)) {
                        Ok(response) => response,
                        Err(err) => {
                            error!("{err}");
                            continue;
                        }
                    };

                    if response.status() == 200 {
                        let user_ref_id = user_gray_wolf.userid;
                        info!("gray_wolf writing data for (user_id, device_id): ({user_ref_id}, {device_id})");

                        let string_json = match rt.block_on(response.text()) {
                            Ok(string) => string,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        let string: String = match serde_json::from_str(&string_json) {
                            Ok(string) => string,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        let client_gray_wolf: ClientGrayWolf = match serde_json::from_str(&string) {
                            Ok(json) => json,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        if let Err(err) = rt.block_on(gray_wolf_controller::create_gray_wolf(connection, client_gray_wolf, user_ref_id, device_id)) {
                            error!("{err}");
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}
