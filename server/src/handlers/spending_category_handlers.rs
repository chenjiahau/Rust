use actix_web::{web, get, post, put, delete, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use entity::spending_categories;
use uuid::Uuid;
use serde::Deserialize;
use validator::Validate;
use chrono;

use crate::models::spending_category_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;

#[derive(Debug, Deserialize)]
struct IdParam {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/spending_category/list",
    security(("bearerAuth" = [])),
    tag = "Spending Category",
)]
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
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
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

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[utoipa::path(
    get,
    path = "/api/spending_category/{id}",
    params(
        ("id" = i64, Path, description = "Spending category id")
    ),
    security(("bearerAuth" = [])),
    tag = "Spending Category",
)]
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
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    // Return the spending category
    let model = result.unwrap();
    if model.is_none() {
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
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
    path = "/api/spending_category",
    security(("bearerAuth" = [])),
    request_body = inline(spending_category_models::SpendingCategoryRequestModel),
    tag = "Spending Category",
)]
#[post("")]
async fn create_spending_category(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    data: web::Json<spending_category_models::SpendingCategoryRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Validate the request
    let req = data.into_inner();
    req.validate().unwrap();

    if req.validate().is_err() {
        let error_message = message::ErrorMessage::InvalidRequest;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Find out the total number of spending categories for the user
    let count = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .count(&app_state.db)
        .await;

    if count.is_err() {
        println!("Error fetching spending categories count: {:?}", count.err());
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    if count.unwrap() >= 20 {
        let error_message = message::ErrorMessage::SpendingCategoryLimitReached;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    // Check if the spending category already exists
    let exists = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Name.eq(req.name.clone()))
        )
        .one(&app_state.db)
        .await;

    if exists.is_err() {
        let error_message = message::ErrorMessage::SpendingCategoryAlreadyExists;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    if exists.unwrap().is_some() {
        let error_message = message::ErrorMessage::SpendingCategoryAlreadyExists;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    // Find out the maximum order number, but not 99 for the user
    let max_order = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Order.lt(99))
        )
        .select_only()
        .column_as(spending_categories::Column::Order.max(), "max_order")
        .into_tuple::<Option<i32>>()
        .one(&app_state.db)
        .await;

    if max_order.is_err() {
        println!("Error fetching max order: {:?}", max_order.err());
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    // Create the spending category
    let next_order = max_order.unwrap().flatten().unwrap_or(0) as i32 + 1;
    let active_model = spending_categories::ActiveModel {
        user_id: Set(Uuid::parse_str(user_id.as_str()).unwrap()),
        name: Set(req.name),
        order: Set(next_order), // Set order to max + 1
        budget: Set(req.budget),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    let result = active_model
        .insert(&app_state.db)
        .await;

    if result.is_err() {
        println!("Error fetching max order1: {}", result.err().unwrap());
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
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

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Created,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[utoipa::path(
    put,
    path = "/api/spending_category/{id}",
    params(
        ("id" = i64, Path, description = "Spending category id")
    ),
    request_body = inline(spending_category_models::SpendingCategoryRequestModel),
    security(("bearerAuth" = [])),
    tag = "Spending Category",
)]
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
    let req = data.into_inner();
    req.validate().unwrap();

    // Check if the name has been created
    let exists = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(spending_categories::Column::Name.eq(req.name.clone()))
                .add(spending_categories::Column::Id.ne(id))
        )
        .one(&app_state.db)
        .await;

    if exists.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    if exists.unwrap().is_some() {
        let error_message = message::ErrorMessage::SpendingCategoryAlreadyExists;
        return response(
            StatusCode::BadRequest,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
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
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    let option_model = result.unwrap();
    if option_model.is_none() {
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    // Update the spending category
    let mut active_model = option_model.unwrap().into_active_model();
    active_model.name = Set(req.name);
    active_model.budget = Set(req.budget);
    active_model.updated_at = Set(chrono::Utc::now().naive_utc());

    let result = active_model
        .update(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
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

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[utoipa::path(
    delete,
    path = "/api/spending_category/{id}",
    params(
        ("id" = i64, Path, description = "Spending category id")
    ),
    security(("bearerAuth" = [])),
    tag = "Spending Category",
)]
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
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    let option_model = result.unwrap();
    if option_model.is_none() {
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    // If it is default, it is not allowed to be deleted
    let model = option_model.unwrap();
    if model.is_default {
        let error_message = message::ErrorMessage::SpendingCategoryNotAllowedToDelete;
        return response(
            StatusCode::Forbidden,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
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
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        )
    }

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Option::<()>::None,
    )
}