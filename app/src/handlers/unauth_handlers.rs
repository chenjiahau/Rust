use actix_web::{post, web, Responder};
use httpstatus::StatusCode;
use sea_orm::{Set, ActiveModelTrait};
use validator::Validate;
use sha256::digest;
use uuid::Uuid;

use crate::models::unauth_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::response;

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
        let error = req.validate().err().unwrap().to_string();
        return response(StatusCode::BadRequest, None, Some(error));
    }

    let result = entity::users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(req.name.unwrap()),
            email: Set(req.email.unwrap()),
            password: Set(digest(req.password.unwrap())),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await;

    if result.is_err() {
        let error = result.err().unwrap().to_string();
        return response(StatusCode::InternalServerError, None, Some(error));
    }

    let entity = result.unwrap();
    let res = unauth_models::SignupResponseModel {
        name: entity.name.clone(),
        email: entity.email.clone(),
    };

    response(StatusCode::Ok, None, Some(res))
}