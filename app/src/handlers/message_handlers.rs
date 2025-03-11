use actix_web::{
    web,
    get,
    post,
    put,
    delete,
    Responder
};
use httpstatus::StatusCode;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    ColumnTrait,
    EntityTrait,
    IntoActiveModel,
    JoinType,
    QueryFilter,
    QueryOrder,
    QuerySelect,
    RelationTrait,
    Set,
};
use serde::Deserialize;
use validator::Validate;
use uuid::Uuid;

use crate::utils::app_state::AppState;
use crate::utils::api_response::{
    response_with_data,
    response_bad_request
};

use crate::models::message_models;

#[derive(Debug, Deserialize)]
struct UserIdParam {
    user_id: Uuid,
}

#[derive(Debug, Deserialize)]
struct UserIdAndMessageIdParam {
    user_id: Uuid,
    message_id: i32,
}

#[post("/create")]
async fn create_message(
    app_state: web::Data::<AppState>,
    data: web::Json<message_models::MessageRequestModel>,
) -> impl Responder {
    // Validate input
    let json_data = data.into_inner();

    if json_data.validate().is_err() {
        return response_bad_request(StatusCode::BadRequest, "Invalid input");
    }

    // Create message
    let result = entity::messages::ActiveModel {
        user_id: Set(json_data.user_id),
        message: Set(json_data.message),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await;

    // Check if message is created
    if result.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to create message");
    }

    // Return created message
    let model = result.unwrap();
    let message = message_models::MessageResponseModel {
        id: model.id,
        user_id: model.user_id,
        message: model.message,
    };

    response_with_data(StatusCode::Ok, "Success", Some(message))
}

#[get("/{user_id}")]
async fn get_messages_by_user(
    app_state: web::Data::<AppState>,
    param: web::Path<UserIdParam>
) -> impl Responder {
    // Fetch messages
    let results = entity::messages::Entity::find()
        .join(JoinType::InnerJoin, entity::messages::Relation::Users.def())
        .select_also(entity::users::Entity)
        .filter(entity::users::Column::Id.eq(param.user_id))
        .order_by_desc(entity::messages::Column::Id)
        .all(&app_state.db)
        .await;

    // Check if messages are fetched
    if results.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to get message");
    }

    let options = results.unwrap();
    if options.is_empty() {
        return response_bad_request(StatusCode::BadRequest, "No message found");
    }

    // Map messages to response model
    let messages_list: Vec<message_models::MessageWithUserModel> = options.iter().map(|option| {
        let message = option.0.clone();
        let user = option.1.clone().unwrap();

        message_models::MessageWithUserModel {
            id: message.id,
            user: message_models::UserModel {
                id: user.id,
                name: user.name,
                email: user.email,
            },
            message: message.message,
        }
    }).collect();

    // Return messages
    response_with_data(StatusCode::Ok, "Success", Some(messages_list))
}

#[put("/{user_id}/{message_id}")]
async fn update_message(
    app_state: web::Data::<AppState>,
    param: web::Path<UserIdAndMessageIdParam>,
    data: web::Json<message_models::UpdateMessageRequestModel>,
) -> impl Responder {
    // Validate input
    let json_data = data.into_inner();

    if json_data.validate().is_err() {
        return response_bad_request(StatusCode::BadRequest, "Invalid input");
    }

    // Check if message exists
    let result = entity::messages::Entity::find()
        .filter(entity::messages::Column::UserId.eq(param.user_id))
        .filter(entity::messages::Column::Id.eq(param.message_id))
        .one(&app_state.db)
        .await;

    if result.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to get message");
    }

    let option = result.unwrap();
    if option.is_none() {
        return response_bad_request(StatusCode::BadRequest, "Message not found");
    }

    // Update message
    let mut message = option.unwrap().into_active_model();
    message.message = ActiveValue::set(json_data.message);

    let result = message.save(&app_state.db).await;
    if result.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to update message");
    }

    // Fetch updated message
    let updated_message = entity::messages::Entity::find()
        .filter(entity::messages::Column::UserId.eq(param.user_id))
        .filter(entity::messages::Column::Id.eq(param.message_id))
        .one(&app_state.db)
        .await;
    if updated_message.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to get updated message");
    }

    let message = updated_message.unwrap().unwrap();
    let updated_message = message_models::UpdateMessageResponseModel {
        id: message.id,
        user_id: message.user_id,
        message: message.message,
    };

    // Return updated message
    response_with_data(StatusCode::Ok, "Success", Some(updated_message))
}

#[delete("/{user_id}")]
async fn delete_messages_by_user(
    app_state: web::Data::<AppState>,
    param: web::Path<UserIdParam>
) -> impl Responder {
    // Delete messages
    let result = entity::messages::Entity::delete_many()
        .filter(entity::messages::Column::UserId.eq(param.user_id))
        .exec(&app_state.db)
        .await;

    // Check if delete is successful
    if result.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to delete message");
    }

    // Return deleted message count
    let deleted_result = result.unwrap();
    let deleted_message = message_models::DeletedMessagesModel {
        user_id: param.user_id,
        count: deleted_result.rows_affected,
    };

    response_with_data(StatusCode::Ok, "Success", Some(deleted_message))
}

#[delete("/{user_id}/{message_id}")]
async fn delete_message_by_user(
    app_state: web::Data::<AppState>,
    param: web::Path<UserIdAndMessageIdParam>
) -> impl Responder {
    // Find message
    let result = entity::messages::Entity::find()
        .filter(entity::messages::Column::UserId.eq(param.user_id))
        .filter(entity::messages::Column::Id.eq(param.message_id))
        .one(&app_state.db)
        .await;

    if result.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to get message");
    }

    let option = result.unwrap();
    if option.is_none() {
        return response_bad_request(StatusCode::BadRequest, "Message not found");
    }

    // Delete message
    let message = option.unwrap().into_active_model();
    let result = message.delete(&app_state.db).await;

    if result.is_err() {
        return response_bad_request(StatusCode::BadRequest, "Failed to delete message");
    }

    // Return deleted message
    let deleted_message = message_models::DeletedMessageModel {
        user_id: param.user_id,
    };

    response_with_data(StatusCode::Ok, "Success", Some(deleted_message))
}