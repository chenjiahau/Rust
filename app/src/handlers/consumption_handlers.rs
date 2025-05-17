use actix_web::{web, get, post, put, delete, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set};
use entity::{places, spending_categories, consumptions};
use uuid::Uuid;
use serde::Deserialize;
use validator::Validate;
use chrono;

use crate::models::{place_models, spending_category_models, consumption_models};
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;

#[derive(Debug, Deserialize)]
struct IdParam {
    id: i64,
}

#[get("/list")]
async fn get_consumptions(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Get consumptions
    let result = consumptions::Entity::find()
        .join(
            sea_orm::JoinType::InnerJoin,
            consumptions::Relation::Places.def(),
        )
        .select_also(places::Entity)
        .join(
            sea_orm::JoinType::InnerJoin,
            consumptions::Relation::SpendingCategories.def(),
        )
        .select_also(spending_categories::Entity)
        .filter(
            Condition::all()
                .add(consumptions::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .order_by_desc(consumptions::Column::CreatedAt)
        .all(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::ConsumptionNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the consumptions
    let models = result.unwrap();
    let mut res: Vec<consumption_models::WholeConsumptionModel> = Vec::new();

    for model in &models {
        let consumption = model.0.clone();
        let place = model.1.clone().unwrap();
        let spending_category = model.2.clone().unwrap();

        let res_model = consumption_models::WholeConsumptionModel {
            id: Some(consumption.id),
            place: place_models::PlaceModel {
                id: place.id,
                user_id: Some(place.user_id),
                name: place.name.clone(),
                created_at: place.created_at.to_string(),
                updated_at: place.updated_at.to_string(),
            },
            spending_category: spending_category_models::SpendingCategoryModel {
                id: Some(spending_category.id),
                user_id: Some(spending_category.user_id),
                name: spending_category.name.clone(),
                order: spending_category.order,
                budget: spending_category.budget,
                is_default: spending_category.is_default,
                created_at: Some(spending_category.created_at.to_string()),
                updated_at: Some(spending_category.updated_at.to_string()),
            },
            amount: consumption.amount,
            created_at: Some(consumption.created_at.to_string()),
            updated_at: Some(consumption.updated_at.to_string()),
        };

        res.push(res_model);
    }

    let response_model = consumption_models::WholeConsumptionsResponseModel {
        consumptions: res,
    };

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(response_model),
    )
}

#[get("/{id}")]
async fn get_consumption(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let id = param.id;

    // Check if the consumption exist
    let result = consumptions::Entity::find()
        .join(
            sea_orm::JoinType::InnerJoin,
            consumptions::Relation::Places.def(),
        )
        .select_also(places::Entity)
        .join(
            sea_orm::JoinType::InnerJoin,
            consumptions::Relation::SpendingCategories.def(),
        )
        .select_also(spending_categories::Entity)
        .filter(
            Condition::all()
                .add(consumptions::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(consumptions::Column::Id.eq(id))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::ConsumptionNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the consumption
    let model = result.unwrap();
    if model.is_none() {
        let error_message = message::ErrorMessage::ConsumptionNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let model = model.unwrap();
    let consumption = model.0.clone();
    let place = model.1.clone().unwrap();
    let spending_category = model.2.clone().unwrap();
    let res = consumption_models::ConsumptionResponseModel {
        id: consumption.id,
        user_id: consumption.user_id,
        place: place_models::PlaceModel {
            id: place.id,
            user_id: Some(place.user_id),
            name: place.name.clone(),
            created_at: place.created_at.to_string(),
            updated_at: place.updated_at.to_string(),
        },
        spending_category: spending_category_models::SpendingCategoryModel {
            id: Some(spending_category.id),
            user_id: Some(spending_category.user_id),
            name: spending_category.name.clone(),
            order: spending_category.order,
            budget: spending_category.budget,
            is_default: spending_category.is_default,
            created_at: Some(spending_category.created_at.to_string()),
            updated_at: Some(spending_category.updated_at.to_string()),
        },
        amount: consumption.amount,
        created_at: Some(consumption.created_at.to_string()),
        updated_at: Some(consumption.updated_at.to_string()),
    };

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[post("")]
async fn create_consumption(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    data: web::Json<consumption_models::ConsumptionRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Validate the request
    let mut req = data.into_inner();
    req.user_id = Some(Uuid::parse_str(user_id.as_str()).unwrap());
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

    // Create the consumption
    let active_model = consumptions::ActiveModel {
        user_id: Set(req.user_id.unwrap()),
        place_id: Set(req.place_id),
        spending_category_id: Set(req.spending_category_id),
        amount: Set(req.amount),
        description: Set(req.description),
        ..Default::default()
    };

    let result = active_model.insert(&app_state.db).await;
    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the consumption
    let model = result.unwrap();
    let res = consumption_models::ConsumptionModel {
        id: Some(model.id),
        user_id: Some(model.user_id),
        place_id: model.place_id,
        spending_category_id: model.spending_category_id,
        amount: model.amount,
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

#[put("/{id}")]
async fn update_consumption(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
    data: web::Json<consumption_models::ConsumptionRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let id = param.id;

    // Validate the request
    let mut req = data.into_inner();
    req.user_id = Some(Uuid::parse_str(user_id.as_str()).unwrap());
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

    // Check if the consumption exist
    let result = consumptions::Entity::find()
        .filter(
            Condition::all()
                .add(consumptions::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(consumptions::Column::Id.eq(id))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::ConsumptionNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let option_model = result.unwrap();
    if option_model.is_none() {
        let error_message = message::ErrorMessage::ConsumptionNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Update the consumption
    let mut active_model = option_model.unwrap().into_active_model();
    active_model.user_id = Set(req.user_id.unwrap());
    active_model.place_id = Set(req.place_id);
    active_model.spending_category_id = Set(req.spending_category_id);
    active_model.amount = Set(req.amount);
    active_model.description = Set(req.description);
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

    // Return the consumption
    let model = result.unwrap();
    let res = consumption_models::ConsumptionModel {
        id: Some(model.id),
        user_id: Some(model.user_id),
        place_id: model.place_id,
        spending_category_id: model.spending_category_id,
        amount: model.amount,
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

#[delete("/{id}")]
async fn delete_consumption(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let id = param.id;

    // Check if the consumption exist
    let result = consumptions::Entity::find()
        .filter(
            Condition::all()
                .add(consumptions::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(consumptions::Column::Id.eq(id))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::ConsumptionNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let option_model = result.unwrap();
    if option_model.is_none() {
        let error_message = message::ErrorMessage::ConsumptionNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Delete the consumption
    let active_model = option_model.unwrap().into_active_model();
    let result = active_model.delete(&app_state.db).await;

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
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Option::<()>::None,
    )
}