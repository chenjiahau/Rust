use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageModel {
    pub user_id: Uuid,
    pub message: String,
}

// For creating a message
#[derive(Debug, Deserialize, Validate)]
pub struct MessageRequestModel {
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 256))]
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponseModel {
    pub id: i32,
    pub user_id: Uuid,
    pub message: String,
}

// For getting messages by user
#[derive(Debug, Serialize)]
pub struct MessageWithUserModel {
    pub id: i32,
    pub user: UserModel,
    pub message: String,
}

// For updating a message
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMessageRequestModel {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateMessageResponseModel {
    pub id: i32,
    pub user_id: Uuid,
    pub message: String,
}

// For deleting messages by user
#[derive(Debug, Serialize)]
pub struct DeletedMessagesModel {
    pub user_id: Uuid,
    pub count: u64,
}

// For deleting message by user
#[derive(Debug, Serialize)]
pub struct DeletedMessageModel {
    pub user_id: Uuid,
}