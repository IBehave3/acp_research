use actix_web::HttpResponse;
use actix_web::Responder;

use bson::Document;


use crate::infra::database;
use crate::model::fitbit_model::ClientFitbit;