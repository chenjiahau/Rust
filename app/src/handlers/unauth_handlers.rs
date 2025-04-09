use actix_web::{post, web, Responder};
use httpstatus::StatusCode;
use sea_orm::{Set, ActiveModelTrait, EntityTrait, QueryFilter, Condition, ColumnTrait};
use validator::Validate;
use sha256::digest;
use uuid::Uuid;
use std::fs::File;
use std::io::BufReader;

use crate::models::{unauth_models, spending_category_models};
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

    if user_result.is_err() {
        let error = user_result.err().unwrap().to_string();
        return response(StatusCode::InternalServerError, None, Some(error));
    }

    // Insert the user to settings table
    let setting_result = entity::settings::ActiveModel {
        user_id: Set(user_id.clone()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await;

    if setting_result.is_err() {
        let error = setting_result.err().unwrap().to_string();
        return response(StatusCode::InternalServerError, None, Some(error));
    }

    let spending_category_data = match File::open("data/spending_categories.json") {
        Ok(file) => file,
        Err(err) => {
            let error = format!("Failed to open spending categories file: {}", err);
            return response(StatusCode::InternalServerError, None, Some(error));
        }
    };
    let reader = BufReader::new(spending_category_data);
    let mut spending_categories: Vec<spending_category_models::SpendingCategoryModel> = match serde_json::from_reader(reader) {
        Ok(categories) => categories,
        Err(err) => {
            let error = format!("Failed to parse spending categories: {}", err);
            return response(StatusCode::InternalServerError, None, Some(error));
        }
    };

    // Insert the default spending categories into the spending_categories table
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
            let error = spending_category_result.err().unwrap().to_string();
            return response(StatusCode::InternalServerError, None, Some(error));
        }
    }

    let entity = user_result.unwrap();
    let res = unauth_models::SignupResponseModel {
        name: entity.name.clone(),
        email: entity.email.clone(),
    };

    response(StatusCode::Ok, None, Some(res))
}

#[post("/signin")]
async fn signin(
    app_state: web::Data::<AppState>,
    data: web::Json<unauth_models::SigninRequestModel>,
) -> impl Responder {
    let req = data.into_inner();

    if req.validate().is_err() {
        let error = req.validate().err().unwrap().to_string();
        return response(StatusCode::BadRequest, None, Some(error));
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
        return response(StatusCode::BadRequest, None, Some("Invalid credentials".to_string()));
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
        let error = result.err().unwrap().to_string();
        return response(StatusCode::InternalServerError, None, Some(error));
    }

    response(StatusCode::Ok, None, Some(res))
}