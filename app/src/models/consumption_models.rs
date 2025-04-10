use serde::{Serialize, Deserialize};
use uuid::Uuid;
use validator::Validate;

use super::place_models::PlaceModel;
use super::spending_category_models::SpendingCategoryModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionModel {
    pub id: Option<i64>,
    pub user_id: Option<Uuid>,
    pub place_id: i64,
    pub spending_category_id: i64,
    pub amount: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeConsumptionModel {
    pub id: Option<i64>,
    pub place: PlaceModel,
    pub spending_category: SpendingCategoryModel,
    pub amount: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct ConsumptionRequestModel {
    pub user_id: Option<Uuid>,
    pub place_id: i64,
    pub spending_category_id: i64,
    #[validate(range(min = 0.0))]
    pub amount: f64,
    #[validate(length(min = 1, max = 255))]
    pub description: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionsResponseModel {
    pub consumptions: Vec<ConsumptionModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WholeConsumptionsResponseModel {
    pub consumptions: Vec<WholeConsumptionModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumptionResponseModel {
    pub id: i64,
    pub user_id: Uuid,
    pub place: PlaceModel,
    pub spending_category: SpendingCategoryModel,
    pub amount: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}