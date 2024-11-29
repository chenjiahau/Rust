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
    let roles_query = entity::roles::Entity::find().all(&app_state.db);

    match roles_query.await {
        Ok(roles_query) => {
            let roles_query = roles_query.into_iter().map(|role_query| {
                role_model::RoleModel {
                    id: role_query.id,
                    title: role_query.title,
                    value: role_query.value
                }
            }).collect::<Vec<role_model::RoleModel>>();

            return api_response::ApiResponse::new(200, serde_json::to_string(&roles_query).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "Roles not found")).unwrap());
        }
    }
}

#[get("/{id}")]
pub async fn get_role(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>
) -> impl Responder {
    let role_query = entity::roles::Entity::find_by_id(param.id).one(&app_state.db);

    match role_query.await {
        Ok(role_query) => {
            if role_query.is_none() {
                return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "Role not found")).unwrap());
            }

            let role_query = role_query.unwrap();
            let role_model = role_model::RoleModel {
                id: role_query.id,
                title: role_query.title,
                value: role_query.value
            };

            return api_response::ApiResponse::new(200, serde_json::to_string(&role_model).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "Role not found")).unwrap());
        }
    }
}

#[post("")]
async fn create_role(
    app_state: web::Data::<AppState>,
    data: Result<web::Json<role_model::RoleRequestModel>, Error>
) -> impl Responder {
    let data = match data {
        Ok(json) => {
            if json.validate().is_err() {
                let response = api_response::generate_response(400, "Invalid input");
                return api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap());
            }

            json.into_inner()
        },
        Err(_) => {
            let response = api_response::generate_response(400, "Invalid input");
            return api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap());
        }
    };

    let role = entity::roles::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(data.title.clone()),
        value: Set(data.value.clone()),
        ..Default::default()
    };

    let role = match role.insert(&app_state.db).await {
        Ok(role) => { role},
        Err(e) => {
            let response = api_response::generate_response(400, e.to_string());
            return api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap());
        }
    };

    let role_data = role_model::RoleModel {
        id: role.id,
        title: role.title.clone(),
        value: role.value.clone()
    };

    let response = api_response::generate_response(200, role_data);
    api_response::ApiResponse::new(200, serde_json::to_string(&response).unwrap())
}

#[put("/{id}")]
pub async fn update_role(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>,
    data: Result<web::Json<role_model::RoleRequestModel>, Error>
) -> impl Responder {
    let data = match data {
        Ok(json) => {
            if json.validate().is_err() {
                let response = api_response::generate_response(400, "Invalid input");
                return api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap());
            }

            json.into_inner()
        },
        Err(_) => {
            let response = api_response::generate_response(400, "Invalid input");
            return api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap());
        }
    };

    let role_query = entity::roles::Entity::find_by_id(param.id).one(&app_state.db);
    let mut role_model = match role_query.await {
        Ok(role_query) => { role_query.unwrap().into_active_model() },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "Role not found")).unwrap());
        }
    };

    role_model.title = Set(data.title.clone());
    role_model.value = Set(data.value.clone());

    match role_model.update(&app_state.db).await {
        Ok(model) => {
            let role_data = role_model::RoleModel {
                id: model.id,
                title: model.title.clone(),
                value: model.value.clone()
            };

            let response = api_response::generate_response(200, role_data);
            api_response::ApiResponse::new(200, serde_json::to_string(&response).unwrap())
        },
        Err(e) => {
            let response = api_response::generate_response(400, e.to_string());
            api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap())
        }
    }
}

#[delete("/{id}")]
pub async fn delete_role(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>
) -> impl Responder {
    let role_query = entity::roles::Entity::find_by_id(param.id).one(&app_state.db);
    let role = match role_query.await {
        Ok(role_query) => { role_query.unwrap().into_active_model() },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "Role not found")).unwrap());
        }
    };

    match role.delete(&app_state.db).await {
        Ok(_) => {
            let response = api_response::generate_response(200, "Role deleted");
            api_response::ApiResponse::new(200, serde_json::to_string(&response).unwrap())
        },
        Err(e) => {
            let response = api_response::generate_response(400, e.to_string());
            api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap())
        }
    }
}