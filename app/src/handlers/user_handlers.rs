use actix_web::{web, get, post, put, delete, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set};
use entity::users;
use serde::Deserialize;
use sha256::digest;
use validator::Validate;
use chrono;

use crate::models::user_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;

#[get("")]
async fn get_user_profile(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let user_id_str = get_user_id_from_request(&req).unwrap();
    let user_id = uuid::Uuid::parse_str(&user_id_str).unwrap();

    // Get User Profile
    let option_model = users::Entity::find_by_id(user_id)
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        let error_message = message::ErrorMessage::UserNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the user profile
    let model = option_model.unwrap();
    let res = user_models::UserProfileModel {
        name: model.name,
    };
    let success_message = message::SuccessMessage::Success;
  
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[put("")]
async fn update_user_profile(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    data: web::Json<user_models::UserProfileUpdateRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let req = data.into_inner();

    // Validate the input
    if req.validate().is_err() {
        let error_message = message::ErrorMessage::InvalidRequest;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Update the user profile
    let user_model: users::ActiveModel = users::ActiveModel {
        id: Set(uuid::Uuid::parse_str(user_id.as_str()).unwrap()),
        name: Set(req.name.clone()),
        ..Default::default()
    };

    let result = user_model.update(&app_state.db).await;
    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the updated user profile
    let model = result.unwrap();
    let res = user_models::UserProfileModel {
        name: model.name,
    };
    let success_message = message::SuccessMessage::Success;

    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[put("/password")]
async fn update_user_password(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    data: web::Json<user_models::UserPasswordUpdateRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let req = data.into_inner();

    // Validate the input
    if req.validate().is_err() {
        let error_message = message::ErrorMessage::InvalidRequest;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Check if the user exists
    let option_model = users::Entity::find_by_id(uuid::Uuid::parse_str(user_id.as_str()).unwrap())
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        let error_message = message::ErrorMessage::UserNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Check if the old password matches
    let model = option_model.unwrap();
    if !model.password.eq(&digest(&req.old_password)) {
        let error_message = message::ErrorMessage::UserPasswordMismatch;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Update the password
    let user_model: users::ActiveModel = users::ActiveModel {
        id: Set(uuid::Uuid::parse_str(user_id.as_str()).unwrap()),
        password: Set(digest(&req.new_password)),
        ..Default::default()
    };

    let result = user_model.update(&app_state.db).await;
    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the updated user profile
    let model = result.unwrap();
    let res = user_models::UserProfileModel {
        name: model.name,
    };
    let success_message = message::SuccessMessage::Success;

    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}