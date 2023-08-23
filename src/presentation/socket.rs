use actix_web::web;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::model::socket;

pub async fn socket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let params = web::Query::<socket::SocketQuery>::from_query(req.query_string()).unwrap();
    let resp = ws::start(socket::SocketRequest {
        socket_query: params.into_inner(),
    }, &req, stream);

    resp
}