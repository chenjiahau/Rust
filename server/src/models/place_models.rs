use serde::{Serialize, Deserialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PlaceModel {
    pub id: i64,
    pub user_id: Option<Uuid>,
    #[serde(rename = "scId")]
    pub spending_category_id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlacesResponseModel {
    pub places: Vec<PlaceModel>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PlaceRequestModel {
    #[serde(rename = "scId")]
    pub spending_category_id: i64,
    pub name: String,
}