use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResponseModel {
    pub name: String,
    pub version: String,
}