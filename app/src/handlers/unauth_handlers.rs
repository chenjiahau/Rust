use actix_web::{post, web, Responder};
use httpstatus::StatusCode;
use sea_orm::{Set, ActiveModelTrait};
use validator::Validate;
use sha256::digest;
use uuid::Uuid;

use crate::models::unauth_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{
  response_with_data,
  response_bad_request
};

#[post("/signup")]
async fn signup(
    app_state: web::Data::<AppState>,
    data: web::Json<unauth_models::SignupRequestModel>,
) -> impl Responder {
    let data = data.into_inner();

    if data.name.is_none()
    || data.email.is_none()
    || data.password.is_none()
    || data.validate().is_err() {
        return response_bad_request(StatusCode::BadRequest, "Invalid input");
    }

    let model = entity::users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(data.name.unwrap()),
            email: Set(data.email.unwrap()),
            password: Set(digest(data.password.unwrap())),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await;

    if model.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to signup");
    }

    let model = model.unwrap();
    let register_model = unauth_models::SignupResponseModel {
        name: model.name.clone(),
        email: model.email.clone(),
    };

    response_with_data(StatusCode::Ok, "Success", Some(register_model))
}

