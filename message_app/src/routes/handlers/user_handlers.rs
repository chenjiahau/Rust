use actix_web::{delete, get, post, web, Error, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;
use validator::Validate;
use sha256::digest;
use uuid::Uuid;

use crate::models::user_model;
use crate::utils::{api_response, app_state::AppState, jwt::Claims};

#[derive(Debug, Deserialize)]
struct Param {
    id: Uuid
}

#[get("")]
pub async fn user(
    app_state: web::Data::<AppState>,
    claims: Claims
) -> impl Responder {
    let model = entity::users::Entity::find_by_id(claims.id)
        .find_also_related(entity::user_roles::Entity)
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
    }

    let model = model.unwrap().unwrap();
    let user_model = user_model::UserModel {
        id: model.0.id,
        name: model.0.name,
        email: model.0.email,
        role_id: Some(model.1.as_ref().unwrap().role_id)
    };

    api_response::ApiResponse::ok(user_model).to_http_response()
}

#[get("/all")]
pub async fn get_all_users(
    app_state: web::Data::<AppState>
) -> impl Responder {
    let model = entity::users::Entity::find()
        .find_also_related(entity::user_roles::Entity)
        .all(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Users not found")).to_http_response();
    }

    let models = model.unwrap().into_iter().map(|model| {
        user_model::UserModel {
            id: model.0.id,
            name: model.0.name,
            email: model.0.email,
            role_id: Some(model.1.as_ref().unwrap().role_id)
        }
    }).collect::<Vec<user_model::UserModel>>();

    api_response::ApiResponse::ok(models).to_http_response()
}

#[get("/{id}")]
pub async fn get_user_by_id(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>
) -> impl Responder {
    let model = entity::users::Entity::find_by_id(param.id)
        .find_also_related(entity::user_roles::Entity)
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
    }

    let model = model.unwrap().unwrap();
    let user_model = user_model::UserModel {
        id: model.0.id,
        name: model.0.name,
        email: model.0.email,
        role_id: Some(model.1.as_ref().unwrap().role_id)
    };

    api_response::ApiResponse::ok(user_model).to_http_response()
}

#[post("")]
pub async fn update_user(
    app_state: web::Data::<AppState>,
    claims: Claims,
    data: web::Json<user_model::UserRequestModel>,
) -> impl Responder {
    let data = data.into_inner();
    if data.validate().is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
    }

    let model = entity::users::Entity::find_by_id(claims.id).one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
    }

    let mut model = model.unwrap().unwrap().into_active_model();

    model.name = Set(data.name.clone());
    model.email = Set(data.email.clone());
    if let Some(password) = data.password {
        model.password = Set(digest(&password));
    }

    let model = model.update(&app_state.db).await;
    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Failed to update user")).to_http_response();
    }

    let model = model.unwrap();
    let user_model = user_model::UserModel {
        id: model.id,
        name: model.name.clone(),
        email: model.email.clone(),
        role_id: None
    };

    api_response::ApiResponse::ok(user_model).to_http_response()
}

#[delete("")]
pub async fn delete_user(
    app_state: web::Data::<AppState>,
    claims: Claims
) -> impl Responder {
    let model = entity::users::Entity::find_by_id(claims.id)
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
    }

    let model = model.unwrap().unwrap().into_active_model();
    match model.delete(&app_state.db).await {
        Ok(_) => {
            return api_response::ApiResponse::ok("User deleted").to_http_response();
        },
        Err(_) => {
            return api_response::ApiResponse
                ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
        }
    }
}