use actix_web::{web, get, post, put, delete, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set};
use entity::places;
use uuid::Uuid;
use serde::Deserialize;
use validator::Validate;
use chrono;

use crate::models::place_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;

#[derive(Debug, Deserialize)]
struct SpendingCategoryId {
    spending_category_id: i64,
}

#[derive(Debug, Deserialize)]
struct IdParam {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/place/list/{spending_category_id}",
    params(
        ("spending_category_id" = i64, Path, description = "Spending category id")
    ),
    security(("bearerAuth" = [])),
    tag = "Place",
)]
#[get("/list/{spending_category_id}")]
async fn get_places(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<SpendingCategoryId>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Check if the setting exist
    let result = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::SpendingCategoryId.eq(param.spending_category_id))
        )
        .order_by_asc(places::Column::Name)
        .all(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::PlaceNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the setting
    let models = result.unwrap();
    let res = place_models::PlacesResponseModel {
        places: models.iter().map(|model| {
            place_models::PlaceModel {
                id: model.id,
                user_id: Some(Uuid::parse_str(user_id.as_str()).unwrap()),
                spending_category_id: model.spending_category_id,
                name: model.name.clone(),
                created_at: model.created_at.to_string(),
                updated_at: model.updated_at.to_string(),
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
    post,
    path = "/api/place",
    request_body = inline(place_models::PlaceRequestModel),
    security(("bearerAuth" = [])),
    tag = "Place",
)]
#[post("")]
async fn create_place(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    data: web::Json<place_models::PlaceRequestModel>,
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

    // Check if the place exist
    let option_model = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::SpendingCategoryId.eq(req.spending_category_id))
                .add(places::Column::Name.eq(req.name.clone()))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_some() {
        let error_message = message::ErrorMessage::PlaceAlreadyExists;
        return response(
            StatusCode::Conflict,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Create the place
    let active_model = places::ActiveModel {
        user_id: Set(Uuid::parse_str(user_id.as_str()).unwrap()),
        spending_category_id: Set(req.spending_category_id),
        name: Set(req.name),
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

    // Return the setting
    let model = result.unwrap();
    let res = place_models::PlaceModel {
        id: model.id,
        user_id: Some(Uuid::parse_str(user_id.as_str()).unwrap()),
        spending_category_id: model.spending_category_id,
        name: model.name.clone(),
        created_at: model.created_at.to_string(),
        updated_at: model.updated_at.to_string(),
    };

    let success_message = message::SuccessMessage::CreatedSuccessfully;
    response(
        StatusCode::Created,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[utoipa::path(
    put,
    path = "/api/place/{id}",
    params(
        ("id" = i64, Path, description = "Place id")
    ),
    request_body = inline(place_models::PlaceRequestModel),
    security(("bearerAuth" = [])),
    tag = "Place",
)]
#[put("/{id}")]
async fn update_place(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
    data: web::Json<place_models::PlaceRequestModel>,
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

    // Check if the place exist
    let option_model = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::SpendingCategoryId.eq(req.spending_category_id))
                .add(places::Column::Id.eq(param.id))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        let error_message = message::ErrorMessage::PlaceNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Check if the place name already exist
    let duplicated_option_model = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::SpendingCategoryId.eq(req.spending_category_id))
                .add(places::Column::Name.eq(req.name.clone()))
                .add(places::Column::Id.ne(param.id))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if duplicated_option_model.is_some() {
        let error_message = message::ErrorMessage::PlaceAlreadyExists;
        return response(
            StatusCode::Conflict,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Update the place
    let mut active_model = option_model.unwrap().into_active_model();
    active_model.spending_category_id = Set(req.spending_category_id);
    active_model.name = Set(req.name);
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

    // Return the setting
    let model = result.unwrap();
    let res = place_models::PlaceModel {
        id: model.id,
        user_id: Some(Uuid::parse_str(user_id.as_str()).unwrap()),
        spending_category_id: model.spending_category_id,
        name: model.name.clone(),
        created_at: model.created_at.to_string(),
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
    delete,
    path = "/api/place/{id}",
    params(
        ("id" = i64, Path, description = "Place id")
    ),
    security(("bearerAuth" = [])),
    tag = "Place",
)]
#[delete("/{id}")]
async fn delete_place(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Check if the place exist
    let option_model = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::Id.eq(param.id))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_none() {
        let error_message = message::ErrorMessage::PlaceNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Delete the place
    let result = places::Entity::delete_many()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::Id.eq(param.id))
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

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Option::<()>::None,
    )
}