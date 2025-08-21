use actix_web::{web, get, Responder};
use httpstatus::StatusCode;
use serde::Deserialize;

use crate::models::app_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::response;
use crate::utils::message;

#[derive(Debug, Deserialize)]
struct TextQuery {
    data: Option<String>
}

#[get("")]
async fn app(app_state: web::Data::<AppState>) -> impl Responder {
    let app = app_models::AppResponseModel {
        name: app_state.name.clone(),
        version: app_state.version.clone()
    };

    let message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        message.to_code(),
        Some(message.to_string()),
        Some(app),
    )
}

#[get("/")]
async fn index(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            let message = message::SuccessMessage::Success;
            response(
                StatusCode::Ok,
                message.to_code(),
                Some(message.to_string()),
                Some(data),
            )
        },
        None => {
            let message = message::ErrorMessage::BadRequest;
            response(
                StatusCode::BadRequest,
                message.to_code(),
                Some(message.to_string()),
                Option::<()>::None,
            )
        }
    }
}

#[get("/bad_request")]
async fn bad_request(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            let message = message::ErrorMessage::BadRequest;
            response(
                StatusCode::BadRequest,
                message.to_code(),
                Some(message.to_string()),
                Some(data),
            )
        },
        None => {
            let message = message::ErrorMessage::BadRequest;
            response(
                StatusCode::BadRequest,
                message.to_code(),
                Some(message.to_string()),
                Option::<()>::None,
            )
        }
    }
}

#[get("/not_found")]
async fn not_found(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            let message = message::ErrorMessage::NotFound;
            response(
                StatusCode::NotFound,
                message.to_code(),
                Some(message.to_string()),
                Some(data),
            )
        },
        None => {
            let message = message::ErrorMessage::NotFound;
            response(
                StatusCode::NotFound,
                message.to_code(),
                Some(message.to_string()),
                Option::<()>::None,
            )
        }
    }
}

#[get("/internal_server_error")]
async fn internal_server_error(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            let message = message::ErrorMessage::InternalServerError;
            response(
                StatusCode::InternalServerError,
                message.to_code(),
                Some(message.to_string()),
                Some(data),
            )
        },
        None => {
            let message = message::ErrorMessage::InternalServerError;
            response(
                StatusCode::InternalServerError,
                message.to_code(),
                Some(message.to_string()),
                Option::<()>::None,
            )
        }
    }
}