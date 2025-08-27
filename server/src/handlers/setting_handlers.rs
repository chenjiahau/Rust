use actix_web::{web, get, post, put, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set};
use uuid::Uuid;
use validator::Validate;
use chrono::{Datelike, Utc};

use entity::{settings, monthly_settings};
use crate::models::setting_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;

#[utoipa::path(
    get,
    path = "/api/setting",
    security(("bearerAuth" = [])),
    tag = "Setting",
)]
#[get("")]
async fn get_setting(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Check if the setting exist
    let option_model = settings::Entity::find()
        .filter(
            Condition::all()
                .add(settings::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        let error_message = message::ErrorMessage::SettingNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the setting
    let model = option_model.unwrap();
    let res = setting_models::SettingResponseModel {
        default_income: model.default_income,
        target_deposit: model.target_deposit,
        updated_at: model.updated_at.to_string(),
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
    path = "/api/setting",
    security(("bearerAuth" = [])),
    request_body = inline(setting_models::SettingRequestModel),
    tag = "Setting",
)]
#[put("")]
async fn update_setting(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    data: web::Json<setting_models::SettingRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
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

    // Check if the setting exist
    let option_model = settings::Entity::find()
        .filter(
            Condition::all()
                .add(settings::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        let error_message = message::ErrorMessage::SettingNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Update the setting
    let mut active_model = option_model.unwrap().into_active_model();
    active_model.default_income = Set(req.default_income);
    active_model.target_deposit = Set(req.target_deposit);
    active_model.updated_at = Set(chrono::Utc::now().naive_utc());

    let result = active_model.update(&app_state.db).await;
    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the updated setting
    let model = result.unwrap();
    let res = setting_models::SettingResponseModel {
        default_income: model.default_income,
        target_deposit: model.target_deposit,
        updated_at: model.updated_at.to_string(),
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
    post,
    path = "/api/setting/current",
    security(("bearerAuth" = [])),
    tag = "Setting",
)]
#[post("/current")]
async fn update_s_to_current_month(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Get settings
    let result = settings::Entity::find()
        .filter(
            Condition::all()
                .add(settings::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .one(&app_state.db)
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

    let settings = result.unwrap();
    if settings.is_none() {
        let error_message = message::ErrorMessage::SettingNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Delete the current year and month of monthly_settings
    let now = Utc::now();
    let current_year = now.year();
    let current_month = now.month();

    let result = monthly_settings::Entity::delete_many()
        .filter(
            Condition::all()
                .add(entity::monthly_settings::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(entity::monthly_settings::Column::Year.eq(Utc::now().year()))
                .add(entity::monthly_settings::Column::Month.eq(Utc::now().month()))
        )
        .exec(&app_state.db)
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

    // Insert the settings to monthly_settings
    let settings = settings.unwrap();
    let new_monthly_setting = monthly_settings::ActiveModel {
        user_id: Set(Uuid::parse_str(user_id.as_str()).unwrap()),
        year: Set(current_year),
        month: Set(current_month as i32),
        income: Set(settings.default_income),
        deposit: Set(settings.target_deposit as f64),
        ..Default::default()
    };

    let result = new_monthly_setting.insert(&app_state.db).await;
    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the response
    let success_message = message::SuccessMessage::Success;
    let res = setting_models::SettingResponseModel {
        default_income: settings.default_income,
        target_deposit: settings.target_deposit,
        updated_at: settings.updated_at.to_string(),
    };

    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}