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
    let user_query = entity::users::Entity::find_by_id(claims.id)
        .find_also_related(entity::user_roles::Entity)
        .one(&app_state.db);

    match user_query.await {
        Ok(user_query) => {
            if user_query.is_none() {
                return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
            }

            let user_query = user_query.unwrap();
            let user_model = user_model::UserModel {
                id: user_query.0.id,
                name: user_query.0.name,
                email: user_query.0.email,
                role_id: Some(user_query.1.as_ref().unwrap().role_id)
            };

            return api_response::ApiResponse::new(200, serde_json::to_string(&user_model).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
        }
    }
}

#[get("/all")]
pub async fn get_all_users(
    app_state: web::Data::<AppState>
) -> impl Responder {
    let user_query = entity::users::Entity::find()
        .find_also_related(entity::user_roles::Entity)
        .all(&app_state.db);

    match user_query.await {
        Ok(user_query) => {
            let user_query = user_query.into_iter().map(|user_query| {
                user_model::UserModel {
                    id: user_query.0.id,
                    name: user_query.0.name,
                    email: user_query.0.email,
                    role_id: Some(user_query.1.as_ref().unwrap().role_id)
                }
            }).collect::<Vec<user_model::UserModel>>();

            return api_response::ApiResponse::new(200, serde_json::to_string(&user_query).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
        }
    }
}

#[get("/{id}")]
pub async fn get_user_by_id(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>
) -> impl Responder {
    let user_query = entity::users::Entity::find_by_id(param.id)
        .find_also_related(entity::user_roles::Entity)
        .one(&app_state.db);

    match user_query.await {
        Ok(user_query) => {
            if user_query.is_none() {
                return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
            }

            let user_query = user_query.unwrap();
            let user_model = user_model::UserModel {
                id: user_query.0.id,
                name: user_query.0.name,
                email: user_query.0.email,
                role_id: Some(user_query.1.as_ref().unwrap().role_id)
            };

            return api_response::ApiResponse::new(200, serde_json::to_string(&user_model).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
        }
    }
}

#[post("")]
pub async fn update_user(
    app_state: web::Data::<AppState>,
    claims: Claims,
    data: Result<web::Json<user_model::UserRequestModel>, Error>
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

    let user_query = entity::users::Entity::find_by_id(claims.id).one(&app_state.db);
    let mut user_model = match user_query.await {
        Ok(user_query) => { user_query.unwrap().into_active_model() },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
        }
    };

    user_model.name = Set(data.name.clone());
    user_model.email = Set(data.email.clone());

    match data.password {
        Some(password) => {
            user_model.password = Set(digest(&password));
        },
        None => {}
    }

    match user_model.update(&app_state.db).await {
        Ok(model) => {
            let user_model = user_model::UserModel {
                id: model.id,
                name: model.name.clone(),
                email: model.email.clone(),
                role_id: None
            };

            return api_response::ApiResponse::new(200, serde_json::to_string(&user_model).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
        }
    }
}

#[delete("")]
pub async fn delete_user(
    app_state: web::Data::<AppState>,
    claims: Claims
) -> impl Responder {
    let user_query = entity::users::Entity::find_by_id(claims.id).one(&app_state.db);
    let user_model = match user_query.await {
        Ok(user_query) => { user_query.unwrap().into_active_model() },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
        }
    };

    match user_model.delete(&app_state.db).await {
        Ok(_) => {
            return api_response::ApiResponse::new(200, serde_json::to_string(&api_response::generate_response(200, "User deleted")).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User not found")).unwrap());
        }
    }
}