use actix_web::{post, web, Responder, Error};
use sea_orm::{Set, ActiveModelTrait, EntityTrait, QueryFilter, Condition, ColumnTrait};
use crate::utils::{app_state::AppState, api_response};
use validator::Validate;
use sha256::digest;
use uuid::Uuid;

use crate::models::{register_model, login_model};
use crate::utils::jwt::encode_jwt;

#[post("/register")]
async fn register(
    app_state: web::Data::<AppState>,
    data: Result<web::Json<register_model::RegisterRequestModel>, Error>
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

    let user = entity::users::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(data.name.clone()),
        email: Set(data.email.clone()),
        password: Set(digest(&data.password)), // Hash the password
        is_active: Set(true),
        ..Default::default()
    };

    let user = match user.insert(&app_state.db).await {
        Ok(user) => { user},
        Err(e) => {
            let response = api_response::generate_response(400, e.to_string());
            return api_response::ApiResponse::new(400, serde_json::to_string(&response).unwrap());
        }
    };

    let user_data = register_model::RegisterModel {
        name: user.name.clone(),
        email: user.email.clone(),
    };

    let response = api_response::generate_response(200, user_data);
    api_response::ApiResponse::new(200, serde_json::to_string(&response).unwrap())
}

#[post("/login")]
async fn login(
    app_state: web::Data::<AppState>,
    data: Result<web::Json<login_model::LoginRequestModel>, Error>
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

    let user = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::Email.eq(&data.email))
                .add(entity::users::Column::Password.eq(digest(&data.password))) // Hash the password
                .add(entity::users::Column::IsActive.eq(true))
        );

    let user = match user.one(&app_state.db).await {
        Ok(user) => { user},
        Err(_) => {
            let response = api_response::generate_response(404, "User not found");
            return api_response::ApiResponse::new(404, serde_json::to_string(&response).unwrap());
        }
    };

    if user.is_none() {
        let response = api_response::generate_response(404, "User not found");
        return api_response::ApiResponse::new(404, serde_json::to_string(&response).unwrap());
    }

    let mut login_data = login_model::LoginModel {
        id: user.as_ref().unwrap().id,
        name: user.as_ref().unwrap().name.clone(),
        email: user.as_ref().unwrap().email.clone(),
        token: "".to_string(),
    };

    let token = encode_jwt(login_data.id, login_data.email.clone()).unwrap();
    login_data.token = token;

    let response = api_response::generate_response(200, login_data);
    api_response::ApiResponse::new(200, serde_json::to_string(&response).unwrap())
}