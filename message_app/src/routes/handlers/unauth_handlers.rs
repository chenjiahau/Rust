use actix_web::http::header::map;
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
    data: web::Json<register_model::RegisterRequestModel>
) -> impl Responder {
    let data = data.into_inner();
    if data.validate().is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
    }

    let model = entity::users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(data.name.clone()),
            email: Set(data.email.clone()),
            password: Set(digest(&data.password)), // Hash the password
            is_active: Set(true),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Failed to create user")).to_http_response();
    }

    let model = model.unwrap();
    let register_model = register_model::RegisterModel {
        name: model.name.clone(),
        email: model.email.clone(),
    };

    api_response::ApiResponse
        ::ok(api_response::generate_response(201, "User created", Some(register_model))).to_http_response()
}

#[post("/login")]
async fn login(
    app_state: web::Data::<AppState>,
    data: web::Json<login_model::LoginRequestModel>,
) -> impl Responder {
    let data = data.into_inner();
    if data.validate().is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Invalid input")).to_http_response();
    }

    let model = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::Email.eq(&data.email))
                .add(entity::users::Column::Password.eq(digest(&data.password))) // Hash the password
                .add(entity::users::Column::IsActive.eq(true))
        )
        .one(&app_state.db)
        .await;

    if model.is_err() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "Failed to login")).to_http_response();
    }

    let model = model.unwrap();
    if model.is_none() {
        return api_response::ApiResponse
            ::error(api_response::DefaultErrorResponse::new(400, "User not found")).to_http_response();
    }

    let model = model.unwrap();
    let mut login_model = login_model::LoginModel {
        id: model.id,
        name: model.name.clone(),
        email: model.email.clone(),
        token: "".to_string(),
    };

    let token = encode_jwt(login_model.id, login_model.email.clone()).unwrap();
    login_model.token = token;

    api_response::ApiResponse::ok(login_model).to_http_response()
}