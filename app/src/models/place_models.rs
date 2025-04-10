use serde::{Serialize, Deserialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceModel {
    pub id: i64,
    pub user_id: Option<Uuid>,
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