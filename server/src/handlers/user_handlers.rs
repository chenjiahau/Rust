use std::io::Cursor;

use actix_web::{web, get, post, put, HttpRequest, Responder};
use actix_multipart::Multipart;
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use entity::users;
use sha256::digest;
use validator::Validate;
use futures_util::TryStreamExt;
use aws_sdk_s3::primitives::ByteStream;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;
use image::ColorType;

use crate::models::user_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;

const MAX_WIDTH: u32 = 1024; // 1024 pixels
const MAX_HEIGHT: u32 = 1024; // 1024 pixels
const ALLOWED_FILE_TYPES: [&str; 3] = ["image/png", "image/jpg", "image/jpeg"];

#[utoipa::path(
    get,
    path = "/api/user",
    security(("bearerAuth" = [])),
    tag = "User",
)]
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

#[utoipa::path(
    put,
    path = "/api/user",
    request_body = inline(user_models::UserProfileUpdateRequestModel),
    security(("bearerAuth" = [])),
    responses(),
    tag = "User",
)]
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

#[utoipa::path(
    put,
    path = "/api/user/password",
    request_body = inline(user_models::UserPasswordUpdateRequestModel),
    security(("bearerAuth" = [])),
    tag = "User",
)]
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

    // Check user exists
    let user_uuid = uuid::Uuid::parse_str(&user_id).unwrap();
    let option_model = users::Entity::find_by_id(user_uuid)
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        return response(
            StatusCode::NotFound,
            message::ErrorMessage::UserNotFound.to_code(),
            Some(message::ErrorMessage::UserNotFound.to_string()),
            Option::<()>::None,
        );
    }

    // Loop through multipart payload to find avatar field
    while let Ok(Some(mut field)) = payload.try_next().await {
        if field.name() != Some("avatar") {
            continue;
        }

        let content_type = field
            .content_type()
            .map(|mime| mime.essence_str())
            .unwrap_or("");

        if !ALLOWED_FILE_TYPES.contains(&content_type) {
            return response(
                StatusCode::BadRequest,
                message::ErrorMessage::UserAvatarFileTypeNotAllowed.to_code(),
                Some(message::ErrorMessage::UserAvatarFileTypeNotAllowed.to_string()),
                Option::<()>::None,
            );
        }

        // 3. Read and resize image
        let mut bytes = web::BytesMut::new();
        while let Some(chunk) = field.try_next().await.unwrap_or(None) {
            bytes.extend_from_slice(&chunk);
        }

        let img = match image::load_from_memory(&bytes) {
            Ok(i) => i.resize(MAX_WIDTH, MAX_HEIGHT, image::imageops::FilterType::Lanczos3),
            Err(e) => {
                return response(
                    StatusCode::BadRequest,
                    message::ErrorMessage::UserAvatarFileFailedToUpload.to_code(),
                    Some(message::ErrorMessage::UserAvatarFileFailedToUpload.to_string()),
                    Option::<()>::None,
                );
            }
        };

        let rgba_image = img.to_rgba8();
        let mut out_buf = Cursor::new(Vec::new());
        let encoder = PngEncoder::new(&mut out_buf);
        if encoder
            .write_image(
                &rgba_image,
                rgba_image.width(),
                rgba_image.height(),
                ColorType::Rgba8.into(),
            )
            .is_err()
        {
            return response(
                StatusCode::InternalServerError,
                message::ErrorMessage::InternalServerError.to_code(),
                Some(message::ErrorMessage::InternalServerError.to_string()),
                Option::<()>::None,
            );
        }

        // Upload to S3
        let new_filename = format!("avatars/{}.png", uuid::Uuid::new_v4());
        let bucket = std::env::var("AWS_BUCKET_NAME").unwrap();
        let region = std::env::var("AWS_REGION").unwrap();

        let s3 = crate::utils::s3::get_s3_client().await;
        let upload_result = s3
            .put_object()
            .bucket(&bucket)
            .key(&new_filename)
            .body(ByteStream::from(out_buf.into_inner()))
            .content_type("image/png")
            .send()
            .await;

        if upload_result.is_err() {
            return response(
                StatusCode::InternalServerError,
                message::ErrorMessage::UserAvatarFileFailedToUpload.to_code(),
                Some(message::ErrorMessage::UserAvatarFileFailedToUpload.to_string()),
                Option::<()>::None,
            );
        }

        // 5. Save avatar path to DB
        let public_url = format!("https://{}.s3.{}.amazonaws.com/{}",bucket, region, new_filename);
        let mut user_model: users::ActiveModel = users::ActiveModel {
            id: Set(user_uuid),
            avatar: Set(Some(public_url.clone())),
            ..Default::default()
        };

        let update_result = user_model.update(&app_state.db).await;
        if update_result.is_err() {
            return response(
                StatusCode::InternalServerError,
                message::ErrorMessage::InternalServerError.to_code(),
                Some(message::ErrorMessage::InternalServerError.to_string()),
                Option::<()>::None,
            );
        }

        let public_url = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            bucket, region, new_filename
        );

        let user = update_result.unwrap();
        let res = user_models::UserProfileModel {
            name: user.name,
            avatar: Some(public_url),
        };

        return response(
            StatusCode::Ok,
            message::SuccessMessage::Success.to_code(),
            Some(message::SuccessMessage::Success.to_string()),
            Some(res),
        );
    }

    // 6. No valid file found
    response(
        StatusCode::BadRequest,
        message::ErrorMessage::UserAvatarFileFailedToUpload.to_code(),
        Some(message::ErrorMessage::UserAvatarFileFailedToUpload.to_string()),
        Option::<()>::None,
    )
}