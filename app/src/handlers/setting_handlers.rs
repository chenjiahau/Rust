use actix_web::{web, get, put, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set};
use uuid::Uuid;
use validator::Validate;
use chrono;

use crate::models::setting_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};

#[get("")]
async fn get_setting(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Check if the setting exist
    let option_model = entity::settings::Entity::find()
        .filter(
            Condition::all()
                .add(entity::settings::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // Return the setting
    let model = option_model.unwrap();
    let res = setting_models::SettingResponseModel {
        default_income: model.default_income,
        target_deposit: model.target_deposit,
        updated_at: model.updated_at.to_string(),
    };

    response(StatusCode::Ok, None, Some(res))
}

#[put("")]
async fn update_setting(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    data: web::Json<setting_models::SettingRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let req = data.into_inner();

    if req.validate().is_err() {
        let error = req.validate().err().unwrap().to_string();
        return response(StatusCode::BadRequest, None, Some(error));
    }

    // Check if the setting exist
    let option_model = entity::settings::Entity::find()
        .filter(
            Condition::all()
                .add(entity::settings::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // Update the setting
    let mut active_model = option_model.unwrap().into_active_model();
    active_model.default_income = Set(req.default_income);
    active_model.target_deposit = Set(req.target_deposit);
    active_model.updated_at = Set(chrono::Utc::now().naive_utc());

    let result = active_model.update(&app_state.db).await;
    if result.is_err() {
        return response::<Option<String>>(StatusCode::InternalServerError, None, None);
    }

    // Return the updated setting
    let model = result.unwrap();
    let res = setting_models::SettingResponseModel {
        default_income: model.default_income,
        target_deposit: model.target_deposit,
        updated_at: model.updated_at.to_string(),
    };

    response(StatusCode::Ok, None, Some(res))
}