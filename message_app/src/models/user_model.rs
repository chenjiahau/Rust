use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String
}

