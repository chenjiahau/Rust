use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct UserRoleRequestModel {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct UserRoleUpdateRequestModel {
    pub role_id: Uuid,
}

#[derive(Serialize)]
pub struct UserRoleModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
}

