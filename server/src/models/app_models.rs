use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResponseModel {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponseModel {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: String,
}