/*use bson::Document;
use chrono::Utc;
use bson::DateTime;

use core::panic;
use log::{error, info};
use mongodb::bson;
use reqwest::{Client, Response};

use std::thread;
use std::time::Duration;

use crate::{
    infra::collection::BaseCollection,
    model::{
        auth::IdMapping, gray_wolf::GrayWolf,
    },
};

const QUERY_FREQ_SECS: u64 = 60;

pub async fn get_device_data(
    api_key: &str,
    dev_id: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    let info_url = format!("https://graywolfliveapi.com/api/LiveReadings/?parameters={{\"apikey\":\"{api_key}\",\"deviceID\":\"{dev_id}\"}}");

    let response = Client::new().get(info_url).send().await?;

    Ok(response)
}

pub fn start_gray_wolf_poll() {
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
            let id_mappings = match rt.block_on(IdMapping::get_gray_wolf_users()) {
                Ok(id_mappings) => id_mappings,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

            for id_mapping in id_mappings {
                let gray_wolf = match id_mapping.sensor_auth.gray_wolf {
                    Some(gray_wolf) => gray_wolf,
                    None => {
                        error!("user {} gray_wolf was null", id_mapping.id);
                        continue;
                    },
                };

                let api_key = gray_wolf.api_key;

                for device_id in gray_wolf.device_ids {
                    let response = match rt.block_on(get_device_data(&api_key, &device_id)) {
                        Ok(response) => response,
                        Err(err) => {
                            error!("{err}");
                            continue;
                        }
                    };

                    if response.status() == 200 {
                        let user_ref_id = id_mapping.id;
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

                        let data: Document = match serde_json::from_str(&string) {
                            Ok(data) => data,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        match rt.block_on(GrayWolf::add(GrayWolf {
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
}*/
