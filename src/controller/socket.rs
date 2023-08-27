/*use actix::{Actor, StreamHandler, WrapFuture, prelude::ContextFutureSpawner};
use actix_web_actors::ws;
use bson::oid::ObjectId;
use chrono::Utc;
use crate::infra::collection::BaseCollection;
use crate::model::socket;
use crate::model::push_data::{PushData, UserPushData};
use crate::model::auth::IdMapping;


use log::info;
use log::error;

impl Actor for socket::SocketRequest {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for socket::SocketRequest {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(message) => {
                info!("message from {}", self.socket_query.user_id);
                
                match message {
                    ws::Message::Text(byte_string) => {
                        let json: socket::FitbitMessage = match serde_json::from_slice(byte_string.as_bytes()) {
                            Ok(message) => message,
                            Err(err) => {
                                eprint!("{err}");
                                return;
                            }
                        };
                        let user_id = self.socket_query.user_id.clone();

                        async move {
                            let id_mapping = match IdMapping::get_id_mapping_by_user_id(&user_id).await {
                                Ok(id_mapping) => id_mapping,
                                Err(err) => {
                                    error!("{err}");
                                    return;
                                }
                            };

                            let id_mapping = match id_mapping {
                                Some(id_mapping) => id_mapping,
                                None => {
                                    error!("user: {user_id} not found");
                                    return;
                                }
                            };

                            match PushData::add(PushData {
                                _id: ObjectId::new(),
                                created_at: bson::DateTime::from_chrono(Utc::now()),
                                data_structure_id: "fitbit".to_string(),
                                id_mapping_ref_id: id_mapping._id,
                                data: UserPushData {
                                    data: json.data,
                                },
                                device_id: None,
                            }).await {
                                Ok(_) => (),
                                Err(err) => error!("{err}"),
                            };
                        }.into_actor(self).wait(ctx)
                    },
                    _ => {
                        eprintln!("unsupported message type");
                    }
                }
            },
            Err(err) => {
                eprint!("{err}");
            },
        }
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        info!("socket finished");
    }
}*/