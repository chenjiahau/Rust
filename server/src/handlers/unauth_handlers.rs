use actix_web::{post, web, Responder};
use httpstatus::StatusCode;
use sea_orm::{Set, ActiveModelTrait, EntityTrait, QueryFilter, Condition, ColumnTrait};
use validator::Validate;
use sha256::digest;
use uuid::Uuid;
use std::fs::File;
use std::io::BufReader;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;

use crate::models::{unauth_models, spending_category_models};
use crate::utils::app_state::AppState;
use crate::utils::api_response::response;
use crate::utils::message;

#[utoipa::path(
    post,
    path = "/api/unauth/signup",
    request_body = inline(unauth_models::SignupRequestModel),
    tag = "Unauth",
)]
#[post("/signup")]
async fn signup(
    app_state: web::Data::<AppState>,
    data: web::Json<unauth_models::SignupRequestModel>,
) -> impl Responder {
    let req = data.into_inner();

    if req.name.is_none()
    || req.email.is_none()
    || req.password.is_none()
    || req.validate().is_err() {
        let error_message = message::ErrorMessage::InvalidRequest;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Check if the email already exists
    let existing_user = entity::users::Entity::find()
        .filter(entity::users::Column::Email.eq(&req.email.clone().unwrap()))
        .one(&app_state.db)
        .await
        .unwrap();

    if !existing_user.is_none() {
        let error_message = message::ErrorMessage::EmailAlreadyRegistered;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Insert the user into the users table
    let user_id = Uuid::new_v4();
    let user_result = entity::users::ActiveModel {
            id: Set(user_id.clone()),
            name: Set(req.name.unwrap()),
            email: Set(req.email.unwrap()),
            password: Set(digest(req.password.unwrap())),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await;

    // Insert the user to settings table
    let default_budget_data = match File::open("data/default_budgets.json") {
        Ok(file) => file,
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

    let reader = BufReader::new(default_budget_data);
    let default_budget: serde_json::Value = match serde_json::from_reader(reader) {
        Ok(data) => data,
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

    let setting_result = entity::settings::ActiveModel {
        user_id: Set(user_id.clone()),
        default_income: Set(default_budget["default_income"].as_i64().unwrap()),
        target_deposit: Set(default_budget["default_target_deposit"].as_i64().unwrap()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await;

    if setting_result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Insert the default spending categories into the spending_categories table
    let spending_category_data = match File::open("data/spending_categories.json") {
        Ok(file) => file,
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
    let reader = BufReader::new(spending_category_data);
    let mut spending_categories: Vec<spending_category_models::SpendingCategoryModel> = match serde_json::from_reader(reader) {
        Ok(categories) => categories,
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

    for category in &mut spending_categories {
        category.user_id = Some(user_id.clone());
        let spending_category_result = entity::spending_categories::ActiveModel {
            user_id: Set(user_id.clone()),
            name: Set(category.name.clone()),
            order: Set(category.order),
            budget: Set(category.budget),
            is_default: Set(true),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await;

        if spending_category_result.is_err() {
            let error_message = message::ErrorMessage::InternalServerError;
            return response(
                StatusCode::InternalServerError,
                error_message.to_code(),
                Some(error_message.to_string()),
                Option::<()>::None,
            );
        }
    }

    let entity = user_result.unwrap();
    let res = unauth_models::SignupResponseModel {
        name: entity.name.clone(),
        email: entity.email.clone(),
    };

    let success_message = message::SuccessMessage::Success;
    response(StatusCode::Ok, success_message.to_code(), Some(success_message.to_string()), Some(res))
}

#[utoipa::path(
    post,
    path = "/api/unauth/signin",
    request_body = inline(unauth_models::SigninRequestModel),
    tag = "Unauth",
)]
#[post("/signin")]
async fn signin(
    app_state: web::Data::<AppState>,
    data: web::Json<unauth_models::SigninRequestModel>,
) -> impl Responder {
    let req = data.into_inner();

    if req.validate().is_err() {
        let error_message = message::ErrorMessage::InvalidRequest;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let result = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::Email.eq(&req.email))
                .add(entity::users::Column::Password.eq(digest(&req.password)))
                .add(entity::users::Column::IsActive.eq(true))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if result.is_none() {
        let error_message = message::ErrorMessage::InvalidCredentials;
        return response(
            StatusCode::Unauthorized,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let entity = result.unwrap();
    let mut res = unauth_models::SigninResponseModel {
        id: entity.id,
        name: entity.name,
        email: entity.email.clone(),
        token: None,
        registered_at: entity.created_at.to_string(),
    };

    let token = crate::utils::jwt::encode_jwt(res.id, res.email.clone()).unwrap();
    res.token = Some(token.clone());

    // Insert token into the database
    let result = entity::tokens::ActiveModel {
        token: Set(token.token),
        email: Set(entity.email.clone()),
        start_time: Set(token.start_time as i64),
        expiration_time: Set(token.expiration_time as i64),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let success_message = message::SuccessMessage::Success;
    response(StatusCode::Ok, success_message.to_code(), Some(success_message.to_string()), Some(res))
}

#[utoipa::path(
    post,
    path = "/api/unauth/forgot-password",
    request_body = inline(unauth_models::ForgotPasswordRequestModel),
    tag = "Unauth",
)]
#[post("/forgot-password")]
async fn create_new_password_by_email(
    app_state: web::Data::<AppState>,
    data: web::Json<unauth_models::ForgotPasswordRequestModel>,
) -> impl Responder {
    let req = data.into_inner();

    if req.validate().is_err() {
        let error_message = message::ErrorMessage::InvalidRequest;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let user = entity::users::Entity::find()
        .filter(entity::users::Column::Email.eq(&req.email))
        .one(&app_state.db)
        .await
        .unwrap();

    // If user is not found, send success response to avoid email enumeration
    if user.is_none() {
        return response(
            StatusCode::Ok,
            message::SuccessMessage::CreatedNewUserPassword.to_code(),
            Some(message::SuccessMessage::CreatedNewUserPassword.to_string()),
            Option::<()>::None,
        );
    }

    let user = user.unwrap();
    let new_password = Uuid::new_v4().to_string()[..8].to_string();
    let hashed_password = digest(new_password.clone());
    let update_result = entity::users::ActiveModel {
        id: Set(user.id),
        password: Set(hashed_password),
        ..Default::default()
    }
    .update(&app_state.db)
    .await;

    if update_result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Send email with the new password
    let from = app_state.smtp_sender.clone();
    let to = user.email.clone();
    let subject = "Your New Password";
    let body = format!("Your new password is: {}\nPlease change it after logging in.", new_password);
    let from_mailbox = match from.parse::<Mailbox>() {
        Ok(mb) => mb,
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
    let to_mailbox = match to.parse::<Mailbox>() {
        Ok(mb) => mb,
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
    let email = match Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(subject)
        .body(body.to_string()) {
            Ok(email) => email,
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

    let smtp_host = app_state.smtp_host.clone();
    let smtp_port = app_state.smtp_port;
    let smtp_user = app_state.smtp_username.clone();
    let smtp_pass = app_state.smtp_password.clone();

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = match AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host) {
        Ok(builder) => builder
            .credentials(creds)
            .port(smtp_port)
            .build(),
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

    if let Err(_) = mailer.send(email).await {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    response(
        StatusCode::Ok,
        message::SuccessMessage::CreatedNewUserPassword.to_code(),
        Some(message::SuccessMessage::CreatedNewUserPassword.to_string()),
        Option::<()>::None,
    )
}