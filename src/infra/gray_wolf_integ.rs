/*use chrono::Utc;

use core::panic;
use log::{error, info};
use mongodb::{bson, bson::oid::ObjectId};
use reqwest::{Client, Response};

use std::thread;
use std::time::Duration;

use crate::{
    infra::collection::BaseCollection,
    model::{auth::IdMapping, push_data::{PushData, UserPushData}},
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
            let id_mappings = match rt.block_on(IdMapping::get_airthings_users()) {
                Ok(id_mappings) => id_mappings,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

            for id_mapping in id_mappings {
                for device_id_mapping in id_mapping.data_structure_device_id_mapping {
                    if device_id_mapping.data_structure_id.eq("gray_wolf") {
                        let device_id_mapping_ref = match device_id_mapping.auth.as_ref() {
                            Some(device_id_mapping_ref) => device_id_mapping_ref,
                            None => {
                                error!("no gray_wolf data structure found for user");
                                continue;
                            }
                        };
                        let api_key = match device_id_mapping_ref.api_key.clone() {
                            Some(api_key) => api_key,
                            None => {
                                error!("no api_key found for gray_wolf");
                                continue;
                            }
                        };

                        let device_ids = match device_id_mapping.device_ids {
                            Some(device_ids) => device_ids,
                            None => {
                                error!("no device ids found for gray_wolf");
                                continue;
                            }
                        };

                        for device_id in device_ids {
                            let response = match rt.block_on(get_device_data(&api_key, &device_id))
                            {
                                Ok(response) => response,
                                Err(err) => {
                                    error!("{err}");
                                    continue;
                                }
                            };

                            if response.status() == 200 {
                                let user_ref_id = id_mapping._id;
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

                                let json: UserPushData = match serde_json::from_str(&string) {
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
                                    data_structure_id: "gray_wolf".to_string(),
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
}*/
