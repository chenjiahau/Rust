use sea_orm::prelude::DateTime;
use serde::{Serialize, Deserialize};
use validator::Validate;

// For creating a message score
#[derive(Debug, Deserialize, Validate)]
pub struct MessageScoreRequestModel {
    pub message_id: i32,
    #[validate(range(min = 1, max = 10))]
    pub score: i32,
}

#[derive(Debug, Serialize)]
pub struct MessageScoreResponseModel {
    pub id: i32,
    pub message_id: i32,
    pub score: i32,
    pub created_at: DateTime,
}

// For getting message scores and messages
#[derive(Debug, Serialize)]
pub struct MessageModal {
    pub id: i32,
    pub message: String,
    pub created_at: DateTime,
}

#[derive(Debug, Serialize)]
pub struct MessageScoreWithMessageResponseModelList {
    pub id: i32,
    pub score: i32,
    pub created_at: DateTime,
    pub message: MessageModal,
}

#[derive(Debug, Serialize)]
pub struct MessageScoresWithMessageResponseModel {
    pub message_scores: Vec<MessageScoreWithMessageResponseModelList>,
}

// For updating a message score
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMessageScoreRequestModel {
    #[validate(range(min = 1, max = 10))]
    pub score: i32,
}

// For deleting a message score
#[derive(Debug, Serialize)]
pub struct DeletedMessageScoreResponseModel {
    pub id: i32,
}
