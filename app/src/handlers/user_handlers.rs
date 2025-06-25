use std::fs::File;
use std::io::Write;

use actix_web::{web, get, post, put, HttpRequest, Responder};
use actix_multipart::Multipart;
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use entity::users;
use sha256::digest;
use validator::Validate;
use futures_util::TryStreamExt;
use futures_util::stream::StreamExt as _;

use crate::models::user_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;
use crate::utils::constants;

const MAX_SIZE: usize = 1024 * 1024; // 1 MB
const ALLOWED_FILE_TYPES: [&str; 2] = ["image/png", "image/jpeg"];

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
        avatar: model.avatar,
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
        avatar: model.avatar,
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
        avatar: model.avatar,
    };
    let success_message = message::SuccessMessage::Success;

    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[post("/avatar")]
async fn upload_user_avatar(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    mut payload: Multipart,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

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

    // Upload the avatar ane save it to the server
    let static_path = &constants::STATIC_PATH;
    let upload_dir = "{}/avatars".replace("{}", static_path);
    std::fs::create_dir_all(&upload_dir).unwrap();

    while let Ok(Some(item)) = payload.try_next().await {
        let mut field = item;

        // Check if the field is named "avatar"
        if field.name() != Some("avatar") {
            continue;
        }

        let content_type = field.content_type().map(|mime| mime.essence_str()).unwrap_or("");

        if !ALLOWED_FILE_TYPES.contains(&content_type) {
            let error_message = message::ErrorMessage::UserAvatarFileTypeNotAllowed;
            return response(
                StatusCode::BadRequest,
                error_message.to_code(),
                Some(error_message.to_string()),
                Option::<()>::None,
            );
        }

        let filepath = format!("{}/{}.png", upload_dir, user_id);
        let mut file = match File::create(&filepath) {
            Ok(f) => f,
            Err(_) => {
                let error_message = message::ErrorMessage::InternalServerError;
                return response(
                    StatusCode::InternalServerError,
                    error_message.to_code(),
                    Some(error_message.to_string()),
                    Option::<()>::None,
                );
            },
        };

        let mut total_size = 0;
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(_) => {
                    let error_message = message::ErrorMessage::InternalServerError;
                    return response(
                        StatusCode::InternalServerError,
                        error_message.to_code(),
                        Some(error_message.to_string()),
                        Option::<()>::None,
                    );
                }
            };

            total_size += data.len();
            if total_size > MAX_SIZE {
                let error_message = message::ErrorMessage::UserAvatarFileSizeLimitExceeded;
                return response(
                    StatusCode::BadRequest,
                    error_message.to_code(),
                    Some(error_message.to_string()),
                    Option::<()>::None,
                );
            }

            if file.write_all(&data).is_err() {
                let error_message = message::ErrorMessage::InternalServerError;
                return response(
                    StatusCode::InternalServerError,
                    error_message.to_code(),
                    Some(error_message.to_string()),
                    Option::<()>::None,
                );
            }
        }

        // Update the user's avatar in the database
        let user_model: users::ActiveModel = users::ActiveModel {
            id: Set(uuid::Uuid::parse_str(user_id.as_str()).unwrap()),
            avatar: Set(Some(format!("{}.png", user_id))),
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
            avatar: model.avatar,
        };
        let success_message = message::SuccessMessage::Success;

        return response(
            StatusCode::Ok,
            success_message.to_code(),
            Some(success_message.to_string()),
            Some(res),
        );
    }

    // If it reaches here, it means no file was uploaded
    let error_message = message::ErrorMessage::UserAvatarFileFailedToUpload;
    response(
        StatusCode::BadRequest,
        error_message.to_code(),
        Some(error_message.to_string()),
        Option::<()>::None,
    )
}