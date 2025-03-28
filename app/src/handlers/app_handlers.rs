use actix_web::{web, get, Responder};
use httpstatus::StatusCode;
use serde::Deserialize;
use sea_orm::EntityTrait;

use crate::utils::app_state::AppState;
use crate::utils::api_response::response;
use crate::models::app_models;

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

    response(StatusCode::Ok, None, Some(app))
}

#[get("/")]
async fn index(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            response(StatusCode::Ok, None, Some(data))
        },
        None => {
            response::<Option<String>>(StatusCode::Ok, None, None)
        }
    }
}

#[get("/bad_request")]
async fn bad_request(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            response(StatusCode::BadRequest, None, Some(data))
        },
        None => {
            response::<Option<String>>(StatusCode::BadRequest, None, None)
        }
    }
}

#[get("/not_found")]
async fn not_found(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            response(StatusCode::NotFound, None, Some(data))
        },
        None => {
            response::<Option<String>>(StatusCode::NotFound, None, None)
        }
    }
}

#[get("/internal_server_error")]
async fn internal_server_error(query: web::Query<TextQuery>) -> impl Responder {
    match query.data.clone() {
        Some(data) => {
            response(StatusCode::InternalServerError, None, Some(data))
        },
        None => {
            response::<Option<String>>(StatusCode::InternalServerError, None, None)
        }
    }
}

// TODO: remove this endpoint in production
#[get("/users")]
async fn users(app_state: web::Data::<AppState>) -> impl Responder {
    let model_list = entity::users::Entity::find()
        .all(&app_state.db)
        .await
        .unwrap();

    let user_list = model_list.into_iter()
        .map(|user| {
            app_models::UserResponseModel {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at.to_string(),
            }
        })
        .collect::<Vec<_>>();

    response(StatusCode::Ok, None, Some(user_list))
}