use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use crate::model::socket;

use log::info;

impl Actor for socket::SocketRequest {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for socket::SocketRequest {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        match msg {
            Ok(message) => {
                info!("message from {}", self.socket_query.user_id);
                
                match message {
                    ws::Message::Text(byte_string) => {
                        let message: socket::FitbitMessage = match serde_json::from_slice(byte_string.as_bytes()) {
                            Ok(message) => message,
                            Err(err) => {
                                eprint!("{err}");
                                return;
                            }
                        };

                        info!("{:?}", message);
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
}