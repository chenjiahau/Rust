use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct UserRequestModel {
    #[validate (length(min = 8, max = 32))]
    pub name: String,
    #[validate (email)]
    pub email: String,
    #[validate (length(min = 8, max = 32))]
    pub password: Option<String>
}

#[derive(Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String
}

