use actix_web::{web, get, post, put, delete, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set};
use entity::spending_categories;
use uuid::Uuid;
use serde::Deserialize;
use validator::Validate;
use chrono;

use crate::models::spending_category_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};

#[derive(Debug, Deserialize)]
struct IdParam {
    id: i64,
}

#[get("/list")]
async fn get_spending_categories(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    
    // Get spending categories
    let result = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .order_by_asc(spending_categories::Column::Order)
        .all(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // Return the spending categories
    let models = result.unwrap();
    let res = spending_category_models::SpendingCategoriesResponseModel {
        spending_categories: models.iter().map(|model| {
            spending_category_models::SpendingCategoryModel {
                id: Some(model.id),
                user_id: Some(model.user_id),
                name: model.name.clone(),
                order: model.order,
                budget: model.budget,
                is_default: model.is_default,
                created_at: Some(model.created_at.to_string()),
                updated_at: Some(model.updated_at.to_string()),
            }
        }).collect(),
    };

    response(StatusCode::Ok, None, Some(res))
}

#[get("/{id}")]
async fn get_spending_category(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let id = param.id;

    // Check if the spending category exist
    let result = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Id.eq(id))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // Return the spending category
    let model = result.unwrap();
    if model.is_none() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    let model = model.unwrap();
    let res = spending_category_models::SpendingCategoryModel {
        id: Some(model.id),
        user_id: Some(model.user_id),
        name: model.name,
        order: model.order,
        budget: model.budget,
        is_default: model.is_default,
        created_at: Some(model.created_at.to_string()),
        updated_at: Some(model.updated_at.to_string()),
    };

    response(StatusCode::Ok, None, Some(res))
}

#[post("")]
async fn create_spending_category(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    data: web::Json<spending_category_models::SpendingCategoryRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Validate the request
    let mut req = data.into_inner();
    req.user_id = Some(Uuid::parse_str(user_id.as_str()).unwrap());
    req.validate().unwrap();

    if req.validate().is_err() {
        return response::<Option<String>>(StatusCode::BadRequest, None, None);
    }

    // Check order is unique and larger than 8 and less than 99
    let order = req.order;

    if order < 8 || order > 99 {
        return response::<Option<String>>(StatusCode::BadRequest, None, None);
    }

    let result = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Order.eq(order))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    if result.unwrap().is_some() {
        return response::<Option<String>>(StatusCode::Conflict, None, None);
    }

    // Create the spending category
    let active_model = spending_categories::ActiveModel {
        user_id: Set(req.user_id.unwrap()),
        name: Set(req.name),
        order: Set(req.order),
        budget: Set(req.budget),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    let result = active_model
        .insert(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::InternalServerError, None, None);
    }

    // Return the spending category
    let model = result.unwrap();
    let res = spending_category_models::SpendingCategoryModel {
        id: Some(model.id),
        user_id: Some(model.user_id),
        name: model.name,
        order: model.order,
        budget: model.budget,
        is_default: model.is_default,
        created_at: Some(model.created_at.to_string()),
        updated_at: Some(model.updated_at.to_string()),
    };

    response(StatusCode::Created, None, Some(res))
}

#[put("/{id}")]
async fn update_spending_category(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
    data: web::Json<spending_category_models::SpendingCategoryRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let id = param.id;

    // Validate the request
    let mut req = data.into_inner();
    req.user_id = Some(Uuid::parse_str(user_id.as_str()).unwrap());
    req.validate().unwrap();

    // Check order is unique and larger than 8 and less than 99
    if req.order < 8 || req.order > 99 {
        return response::<Option<String>>(StatusCode::BadRequest, None, None);
    }

    // Check if the spending category exists
    let result = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Id.eq(id))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    let option_model = result.unwrap();
    if option_model.is_none() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // If it is default category, its order is not allowed to be changed
    let model = option_model.clone().unwrap();
    if model.is_default {
        req.order = model.order;
    }
    
    // Update the spending category
    let mut active_model = option_model.unwrap().into_active_model();
    active_model.name = Set(req.name);
    active_model.order = Set(req.order);
    active_model.budget = Set(req.budget);
    active_model.updated_at = Set(chrono::Utc::now().naive_utc());

    let result = active_model
        .update(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::InternalServerError, None, None);
    }

    // Return the spending category
    let model = result.unwrap();
    let res = spending_category_models::SpendingCategoryModel {
        id: Some(model.id),
        user_id: Some(model.user_id),
        name: model.name,
        order: model.order,
        budget: model.budget,
        is_default: model.is_default,
        created_at: Some(model.created_at.to_string()),
        updated_at: Some(model.updated_at.to_string()),
    };

    response(StatusCode::Ok, None, Some(res))
}

#[delete("/{id}")]
async fn delete_spending_category(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let id = param.id;

    // Check if the spending category exists
    let result = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Id.eq(id))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    let option_model = result.unwrap();
    if option_model.is_none() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // If it is default, it is not allowed to be deleted
    let model = option_model.unwrap();
    if model.is_default {
        return response::<Option<String>>(StatusCode::Forbidden, None, None);
    }

    // Delete the spending category
    let result = spending_categories::Entity::delete_many()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Id.eq(id))
        )
        .exec(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::InternalServerError, None, None);
    }

    // Return the spending category
    response::<Option<String>>(StatusCode::Ok, None, None)
}