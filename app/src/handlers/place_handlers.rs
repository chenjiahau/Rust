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

#[derive(Debug, Deserialize)]
struct IdParam {
    id: i64,
}

#[get("/list")]
async fn get_places(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Check if the setting exist
    let result = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .order_by_asc(places::Column::Name)
        .all(&app_state.db)
        .await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // Return the setting
    let models = result.unwrap();
    let res = place_models::PlacesResponseModel {
        places: models.iter().map(|model| {
            place_models::PlaceModel {
                id: model.id,
                user_id: Some(Uuid::parse_str(user_id.as_str()).unwrap()),
                name: model.name.clone(),
                created_at: model.created_at.to_string(),
                updated_at: model.updated_at.to_string(),
            }
        }).collect(),
    };

    response(StatusCode::Ok, None, Some(res))
}

#[post("")]
async fn create_place(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    data: web::Json<place_models::SettingRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let req = data.into_inner();

    if req.validate().is_err() {
        let error = req.validate().err().unwrap().to_string();
        return response(StatusCode::BadRequest, None, Some(error));
    }

    // Check if the place exist
    let option_model = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::Name.eq(req.name.clone()))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if option_model.is_some() {
        return response::<Option<String>>(StatusCode::Conflict, None, None);
    }

    // Create the place
    let active_model = places::ActiveModel {
        user_id: Set(Uuid::parse_str(user_id.as_str()).unwrap()),
        name: Set(req.name),
        ..Default::default()
    };

    let result = active_model.insert(&app_state.db).await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::InternalServerError, None, None);
    }

    // Return the setting
    let model = result.unwrap();
    let res = place_models::PlaceModel {
        id: model.id,
        user_id: Some(Uuid::parse_str(user_id.as_str()).unwrap()),
        name: model.name.clone(),
        created_at: model.created_at.to_string(),
        updated_at: model.updated_at.to_string(),
    };

    response(StatusCode::Created, None, Some(res))
}

#[put("/{id}")]
async fn update_place(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    param: web::Path<IdParam>,
    data: web::Json<place_models::SettingRequestModel>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let req = data.into_inner();

    if req.validate().is_err() {
        let error = req.validate().err().unwrap().to_string();
        return response(StatusCode::BadRequest, None, Some(error));
    }

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
        return response::<Option<String>>(StatusCode::NotFound, None, None);
    }

    // Check if the place name already exist
    let duplicated_option_model = places::Entity::find()
        .filter(
            Condition::all()
                .add(places::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(places::Column::Name.eq(req.name.clone()))
                .add(places::Column::Id.ne(param.id))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if duplicated_option_model.is_some() {
        return response::<Option<String>>(StatusCode::Conflict, None, None);
    }

    // Update the place
    let mut active_model = option_model.unwrap().into_active_model();
    active_model.name = Set(req.name);
    active_model.updated_at = Set(chrono::Utc::now().naive_utc());

    let result = active_model.update(&app_state.db).await;

    if result.is_err() {
        return response::<Option<String>>(StatusCode::InternalServerError, None, None);
    }

    // Return the setting
    let model = result.unwrap();
    let res = place_models::PlaceModel {
        id: model.id,
        user_id: Some(Uuid::parse_str(user_id.as_str()).unwrap()),
        name: model.name.clone(),
        created_at: model.created_at.to_string(),
        updated_at: model.updated_at.to_string(),
    };

    response(StatusCode::Ok, None, Some(res))
}

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
        return response::<Option<String>>(StatusCode::NotFound, None, None);
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
        return response::<Option<String>>(StatusCode::InternalServerError, None, None);
    }

    response::<Option<String>>(StatusCode::Ok, None, None)
}