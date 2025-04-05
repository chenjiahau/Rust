use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize)]
pub struct PlaceModel {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct PlacesResponseModel {
    pub places: Vec<PlaceModel>,
}

#[derive(Deserialize, Validate)]
pub struct SettingRequestModel {
    pub name: String,
}