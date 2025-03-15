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
    ConnectionTrait,
    Statement,
};
use serde::Deserialize;
use validator::Validate;
use chrono;

use crate::utils::app_state::AppState;
use crate::utils::api_response::{
    response_with_data,
    response_bad_request
};

use crate::models::message_score_models;

#[derive(Debug, Deserialize)]
struct IdParam {
    id: i32,
}

#[post("/create")]
async fn create_message_score(
    app_state: web::Data::<AppState>,
    data: web::Json<message_score_models::MessageScoreRequestModel>,
) -> impl Responder {
    // Validate input
    let json_data = data.into_inner();

    if json_data.validate().is_err() {
        return response_bad_request(StatusCode::BadRequest, "Invalid input");
    }

    // Create message score
    let sql = "INSERT INTO message_scores (message_id, score) VALUES ($1, $2) RETURNING id;";
    let result = app_state.db.query_one(
        Statement::from_sql_and_values(
            app_state.db.get_database_backend(),
            sql,
            [
                sea_orm::Value::Int(Some(json_data.message_id)),
                sea_orm::Value::Int(Some(json_data.score)),
            ],
        )
    ).await;

    // Check if there is an error
    if let Err(_) = result {
        return response_bad_request(StatusCode::BadRequest, "Failed to create message score");
    }

    // Return the created message score
    let result = result.unwrap();
    let option = result.unwrap();
    let id: i32 = option.try_get("", "id").unwrap();

    let message_score = message_score_models::MessageScoreResponseModel {
        id,
        message_id: json_data.message_id,
        score: json_data.score as i32,
        created_at: chrono::Utc::now().naive_utc(),
    };

    response_with_data(StatusCode::Ok, "Success", Some(message_score))
}

#[get("/all")]
async fn get_all_message_scores(
    app_state: web::Data::<AppState>,
) -> impl Responder {
    // Get all message scores
    let sql = "
        SELECT
            messages.id as message_id,
            messages.message as message_message,
            messages.created_at as message_created_at,
            message_scores.id as score_id,
            message_scores.score as score_score,
            message_scores.created_at as score_created_at
        FROM message_scores
        INNER JOIN messages ON message_scores.message_id = messages.id;
    ";

    let result = app_state.db.query_all(
        Statement::from_sql_and_values(
            app_state.db.get_database_backend(),
            sql,
            [],
        )
    ).await;

    // Check if there is an error
    if let Err(_) = result {
        return response_bad_request(StatusCode::BadRequest, "Failed to get message scores");
    }

    // Return the message scores
    let result = result.unwrap();
    let mut message_scores: Vec<message_score_models::MessageScoreWithMessageResponseModelList> = Vec::new();

    for row in result {
        let message_id = row.try_get("", "message_id").unwrap();
        let message = row.try_get("", "message_message").unwrap();
        let created_at = row.try_get("", "message_created_at").unwrap();
        let score_id = row.try_get("", "score_id").unwrap();
        let score = row.try_get("", "score_score").unwrap();
        let score_created_at = row.try_get("", "score_created_at").unwrap();

        let message_score = message_score_models::MessageScoreWithMessageResponseModelList {
            id: score_id,
            score,
            created_at: score_created_at,
            message: message_score_models::MessageModal {
                id: message_id,
                message,
                created_at,
            },
        };

        message_scores.push(message_score);
    }

    let message_scores = message_score_models::MessageScoresWithMessageResponseModel {
        message_scores,
    };

    response_with_data(StatusCode::Ok, "Success", Some(message_scores))
}

#[get("/{id}")]
async fn get_message_score(
    app_state: web::Data::<AppState>,
    id: web::Path<IdParam>,
) -> impl Responder {
    // Get message score
    let sql = "
        SELECT
            messages.id as message_id,
            messages.message as message_message,
            messages.created_at as message_created_at,
            message_scores.id as score_id,
            message_scores.score as score_score,
            message_scores.created_at as score_created_at
        FROM message_scores
        INNER JOIN messages ON message_scores.message_id = messages.id
        WHERE message_scores.id = $1;
    ";

    let result = app_state.db.query_one(
        Statement::from_sql_and_values(
            app_state.db.get_database_backend(),
            sql,
            [
                sea_orm::Value::Int(Some(id.id)),
            ],
        )
    ).await;

    // Check if there is an error
    if let Err(_) = result {
        return response_bad_request(StatusCode::BadRequest, "Failed to get message score");
    }

    // Check if message score is not found
    let result = result.unwrap();
    if result.is_none() {
        return response_bad_request(StatusCode::BadRequest, "Message score not found");
    }

    // Return the message score
    let option = result.unwrap();
    let message_id = option.try_get("", "message_id").unwrap();
    let message = option.try_get("", "message_message").unwrap();
    let created_at = option.try_get("", "message_created_at").unwrap();
    let score_id = option.try_get("", "score_id").unwrap();
    let score = option.try_get("", "score_score").unwrap();
    let score_created_at = option.try_get("", "score_created_at").unwrap();

    let message_score = message_score_models::MessageScoreWithMessageResponseModelList {
        id: score_id,
        score,
        created_at: score_created_at,
        message: message_score_models::MessageModal {
            id: message_id,
            message,
            created_at,
        },
    };
    
    response_with_data(StatusCode::Ok, "Success", Some(message_score))
}

#[put("/{id}")]
async fn update_message_score(
    app_state: web::Data::<AppState>,
    id: web::Path<IdParam>,
    data: web::Json<message_score_models::UpdateMessageScoreRequestModel>,
) -> impl Responder {
    // Validate input
    let json_data = data.into_inner();

    if json_data.validate().is_err() {
        return response_bad_request(StatusCode::BadRequest, "Invalid input");
    }

    // Check if message score exists
    let sql = "SELECT * FROM message_scores WHERE id = $1;";
    let result = app_state.db.query_one(
        Statement::from_sql_and_values(
            app_state.db.get_database_backend(),
            sql,
            [
                sea_orm::Value::Int(Some(id.id)),
            ],
        )
    ).await;

    // Check if there is an error
    if let Err(_) = result {
        return response_bad_request(StatusCode::BadRequest, "Failed to get message score");
    }

    // Check if message score is not found
    if let Ok(result) = result {
        if result.is_none() {
            return response_bad_request(StatusCode::BadRequest, "Message score not found");
        }
    }

    // Update message score
    let sql = "UPDATE message_scores SET score = $1 WHERE id = $2;";
    let result = app_state.db.execute(
        Statement::from_sql_and_values(
            app_state.db.get_database_backend(),
            sql,
            [
                sea_orm::Value::Int(Some(json_data.score)),
                sea_orm::Value::Int(Some(id.id)),
            ],
        )
    ).await;

    // Check if there is an error
    if let Err(_) = result {
        return response_bad_request(StatusCode::BadRequest, "Failed to update message score");
    }

    let sql = "
        SELECT
            messages.id as message_id,
            messages.message as message_message,
            messages.created_at as message_created_at,
            message_scores.id as score_id,
            message_scores.score as score_score,
            message_scores.created_at as score_created_at
        FROM message_scores
        INNER JOIN messages ON message_scores.message_id = messages.id
        WHERE message_scores.id = $1;
    ";
    let result = app_state.db.query_one(
        Statement::from_sql_and_values(
            app_state.db.get_database_backend(),
            sql,
            [
                sea_orm::Value::Int(Some(id.id)),
            ],
        )
    ).await;

    // Check if there is an error
    if let Err(_) = result {
        return response_bad_request(StatusCode::BadRequest, "Failed to get message score");
    }

    // Return the updated message score
    let result = result.unwrap();
    let option = result.unwrap();
    let message_id = option.try_get("", "message_id").unwrap();
    let message = option.try_get("", "message_message").unwrap();
    let created_at = option.try_get("", "message_created_at").unwrap();
    let score_id = option.try_get("", "score_id").unwrap();
    let score = option.try_get("", "score_score").unwrap();

    let message_score = message_score_models::MessageScoreWithMessageResponseModelList {
        id: score_id,
        score,
        created_at: chrono::Utc::now().naive_utc(),
        message: message_score_models::MessageModal {
            id: message_id,
            message,
            created_at,
        },
    };

    response_with_data(StatusCode::Ok, "Success", Some(message_score))
}

#[delete("/{id}")]
async fn delete_message_score(
    app_state: web::Data::<AppState>,
    id: web::Path<IdParam>,
) -> impl Responder {
    // Delete message score
    let sql = "DELETE FROM message_scores WHERE id = $1;";
    let result = app_state.db.execute(
        Statement::from_sql_and_values(
            app_state.db.get_database_backend(),
            sql,
            [
                sea_orm::Value::Int(Some(id.id)),
            ],
        )
    ).await;

    // Check if there is an error
    if let Err(_) = result {
        return response_bad_request(StatusCode::BadRequest, "Failed to delete message score");
    }

    // Check if message score is not found
    if let Ok(result) = result {
        if result.rows_affected() == 0 {
            return response_bad_request(StatusCode::BadRequest, "Message score not found");
        }
    }

    // Return the deleted message score
    let deleted_message_score = message_score_models::DeletedMessageScoreResponseModel {
        id: id.id,
    };

    response_with_data(StatusCode::Ok, "Success", Some(deleted_message_score))
}