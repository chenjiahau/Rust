use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct RoleRequestModel {
    #[validate (length(min = 1, max = 32))]
    pub title: String,
    #[validate (range(min = 0, max = 255))]
    pub value: i32
}

#[derive(Serialize)]
pub struct RoleModel {
    pub id: Uuid,
    pub title: String,
    pub value: i32
}

