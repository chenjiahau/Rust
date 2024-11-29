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
    let userrole_query = entity::user_roles::Entity::find()
        .filter(entity::user_roles::Column::UserId.eq(param.user_id))
        .one(&app_state.db);

    match userrole_query.await {
        Ok(userrole_query) => {
            let userrole_query = userrole_query.into_iter().map(|userrole_query| {
                userrole_model::UserRoleModel {
                    id: userrole_query.id,
                    user_id: userrole_query.user_id,
                    role_id: userrole_query.role_id,
                }
            }).collect::<Vec<userrole_model::UserRoleModel>>();

            return api_response::ApiResponse::new(200, serde_json::to_string(&userrole_query).unwrap());
        },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "User roles not found")).unwrap());
        }
    }
}

#[post("")]
async fn create_userrole(
    app_state: web::Data::<AppState>,
    data: Result<web::Json<userrole_model::UserRoleRequestModel>, Error>
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

    let role = entity::user_roles::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(data.user_id),
        role_id: Set(data.role_id),
        ..Default::default()
    };

    let user_role = match role.insert(&app_state.db).await {
        Ok(user_role) => { user_role },
        Err(e) => {
            let response = api_response::generate_response(400, e.to_string());
            return api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap());
        }
    };

    let role_data = userrole_model::UserRoleModel {
        id: user_role.id,
        user_id: user_role.user_id,
        role_id: user_role.role_id,
    };

    let response = api_response::generate_response(200, role_data);
    api_response::ApiResponse::new(200, serde_json::to_string(&response).unwrap())
}

#[put("/{user_id}")]
async fn update_userrole(
    app_state: web::Data::<AppState>,
    param: web::Path<Param>,
    data: Result<web::Json<userrole_model::UserRoleUpdateRequestModel>, Error>
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

    let userrole_query = entity::user_roles::Entity::find()
        .filter(entity::user_roles::Column::UserId.eq(param.user_id))
        .one(&app_state.db);

    let mut userrole_model = match userrole_query.await {
        Ok(userrole_query) => { userrole_query.unwrap().into_active_model() },
        Err(_) => {
            return api_response::ApiResponse::new(400, serde_json::to_string(&api_response::generate_response(400, "Role not found")).unwrap());
        }
    };

    userrole_model.role_id = Set(data.role_id);

    match userrole_model.update(&app_state.db).await {
        Ok(model) => {
            let role_data = userrole_model::UserRoleModel {
                id: model.id,
                user_id: model.user_id,
                role_id: model.role_id,
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