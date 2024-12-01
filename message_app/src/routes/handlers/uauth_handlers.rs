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
                return api_response::ApiResponse
                    ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
            }

            json.into_inner()
        },
        Err(_) => {
            return api_response::ApiResponse
                ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
        }
    };

    let new_user = entity::users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(data.name.clone()),
            email: Set(data.email.clone()),
            password: Set(digest(&data.password)), // Hash the password
            is_active: Set(true),
            ..Default::default()
        }
        .insert(&app_state.db);
    
    let new_user = match new_user.await {
        Ok(user) => { user },
        Err(e) => {
            return api_response::ApiResponse
                ::error(api_response::DefaultErrorResponse::new(400, &e.to_string())).to_http_response();
        }
    };

    let user_data = register_model::RegisterModel {
        name: new_user.name.clone(),
        email: new_user.email.clone(),
    };

    api_response::ApiResponse
        ::ok(api_response::generate_response(201, "User created", Some(user_data))).to_http_response()
}

#[post("/login")]
async fn login(
    app_state: web::Data::<AppState>,
    data: Result<web::Json<login_model::LoginRequestModel>, Error>
) -> impl Responder {
    let data = match data {
        Ok(json) => {
            if json.validate().is_err() {
                return api_response::ApiResponse
                    ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
            }

            json.into_inner()
        },
        Err(_) => {
            return api_response::ApiResponse
                ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
        }
    };

    let exist_user = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::Email.eq(&data.email))
                .add(entity::users::Column::Password.eq(digest(&data.password))) // Hash the password
                .add(entity::users::Column::IsActive.eq(true))
        );

    let exist_user = match exist_user.one(&app_state.db).await {
        Ok(user) => {
            if user.is_none() {
                return api_response::ApiResponse
                    ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
            }

            user.unwrap()
        },
        Err(_) => {
            return api_response::ApiResponse
                ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
        }
    };

    let mut login_data = login_model::LoginModel {
        id: exist_user.id,
        name: exist_user.name.clone(),
        email: exist_user.email.clone(),
        token: "".to_string(),
    };

    let token = encode_jwt(login_data.id, login_data.email.clone()).unwrap();
    login_data.token = token;

    api_response::ApiResponse::ok(login_data).to_http_response()
}