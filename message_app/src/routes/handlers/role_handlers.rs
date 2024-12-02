use actix_web::{delete, get, post, put, web, Error, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;
use validator::Validate;
use uuid::Uuid;

use crate::models::role_model;
use crate::utils::{api_response, app_state::AppState, jwt::Claims};

#[derive(Debug, Deserialize)]
struct Param {
    id: Uuid
}

#[get("/all")]
pub async fn get_all_role(
    app_state: web::Data::<AppState>
) -> impl Responder {
    let model = entity::roles::Entity::find().all(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Roles not found")).to_http_response();
    }

    let models = model.unwrap().into_iter().map(|model| {
        role_model::RoleModel {
            id: model.id,
            title: model.title,
            value: model.value
        }
    }).collect::<Vec<role_model::RoleModel>>();

    api_response::ApiResponse::ok(models).to_http_response()
}

#[get("/{id}")]
pub async fn get_role(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>
) -> impl Responder {
    let model = entity::roles::Entity::find_by_id(param.id).one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Role not found")).to_http_response();
    }

    let model = model.unwrap().unwrap();
    let role_model = role_model::RoleModel {
        id: model.id,
        title: model.title,
        value: model.value
    };

    api_response::ApiResponse::ok(role_model).to_http_response()
}

#[post("")]
async fn create_role(
    app_state: web::Data::<AppState>,
    data: web::Json<role_model::RoleRequestModel>
) -> impl Responder {
    let data = data.into_inner();

    if data.validate().is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
    }

    let model = entity::roles::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(data.title.clone()),
            value: Set(data.value.clone()),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Failed to create role")).to_http_response();
    }

    let model = model.unwrap();
    let role_model = role_model::RoleModel {
        id: model.id,
        title: model.title,
        value: model.value
    };

    api_response::ApiResponse::ok(role_model).to_http_response()
}

#[put("/{id}")]
pub async fn update_role(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>,
    data: web::Json<role_model::RoleRequestModel>
) -> impl Responder {
    let data = data.into_inner();

    if data.validate().is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
    }

    let model = entity::roles::Entity::find_by_id(param.id)
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Role not found")).to_http_response();
    }

    let mut model = model.unwrap().unwrap().into_active_model();
    model.title = Set(data.title.clone());
    model.value = Set(data.value.clone());

    let model = model.update(&app_state.db).await;
    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Failed to update role")).to_http_response();
    }

    let model = model.unwrap();
    let role_model = role_model::RoleModel {
        id: model.id,
        title: model.title,
        value: model.value
    };

    api_response::ApiResponse::ok(role_model).to_http_response()
}

#[delete("/{id}")]
pub async fn delete_role(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>
) -> impl Responder {
    let model = entity::roles::Entity::find_by_id(param.id)
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Role not found")).to_http_response();
    }

    let model = model.unwrap();
    if model.is_none() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Role not found")).to_http_response();
    }

    let model = model.unwrap().into_active_model();
    match model.delete(&app_state.db).await {
        Ok(_) => {
            return api_response::ApiResponse::ok("Role deleted").to_http_response();
        },
        Err(e) => {
            return api_response::ApiResponse
                ::error(api_response::DefaultErrorResponse::new(400, &e.to_string())).to_http_response();
        }
    };
}