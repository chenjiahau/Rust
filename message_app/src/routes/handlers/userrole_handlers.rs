use actix_web::{get, post, put, web, Error, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ColumnTrait, QueryFilter, Set};
use serde::Deserialize;
use validator::Validate;
use uuid::Uuid;

use crate::models::userrole_model;
use crate::utils::{api_response, app_state::AppState};

#[derive(Debug, Deserialize)]
struct Param {
    user_id: Uuid
}

#[get("/{user_id}")]
async fn get_userrole(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>
) -> impl Responder {
    let model = entity::user_roles::Entity::find()
        .filter(entity::user_roles::Column::UserId.eq(param.user_id))
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User role not found")).to_http_response();
    }

    let model = model.unwrap();
    if model.is_none() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User role not found")).to_http_response();
    }

    let model = model.unwrap();
    let userrole_model = userrole_model::UserRoleModel {
        id: model.id,
        user_id: model.user_id,
        role_id: model.role_id,
    };

    api_response::ApiResponse::ok(userrole_model).to_http_response()
}

#[post("")]
async fn create_userrole(
    app_state: web::Data::<AppState>,
    data: web::Json<userrole_model::UserRoleRequestModel>,
) -> impl Responder {
    let data = data.into_inner();

    if data.validate().is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
    }

    let model = entity::user_roles::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(data.user_id),
            role_id: Set(data.role_id),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Failed to create user role")).to_http_response();
    }

    let model = model.unwrap();
    let userrole_model = userrole_model::UserRoleModel {
        id: model.id,
        user_id: model.user_id,
        role_id: model.role_id,
    };

    api_response::ApiResponse::ok(userrole_model).to_http_response()
}

#[put("/{user_id}")]
async fn update_userrole(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>,
    data: web::Json<userrole_model::UserRoleUpdateRequestModel>,
) -> impl Responder {
    let data = data.into_inner();

    if data.validate().is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
    }

    let model = entity::user_roles::Entity::find()
        .filter(entity::user_roles::Column::UserId.eq(param.user_id))
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User role not found")).to_http_response();
    }

    let model = model.unwrap();
    if model.is_none() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User role not found")).to_http_response();
    }

    let mut model = model.unwrap().into_active_model();
    model.role_id = Set(data.role_id);

    let model = model.update(&app_state.db).await;
    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Failed to update user role")).to_http_response();
    }

    let model = model.unwrap();
    let userrole_model = userrole_model::UserRoleModel {
        id: model.id,
        user_id: model.user_id,
        role_id: model.role_id,
    };

    api_response::ApiResponse::ok(userrole_model).to_http_response()
}