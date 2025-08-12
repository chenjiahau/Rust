use actix_web::{get, post, web, HttpRequest, Responder};
use entity::spending_categories;
use httpstatus::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::monthly_budget_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;

#[derive(Debug, Deserialize)]
struct YearMonthParam {
    year: i32,
    month: i32,
}

#[utoipa::path(
    get,
    path = "/api/monthly_budget/count/{year}/{month}",
    params(
        ("year" = i32, Path, description = "Year of the monthly budget"),
        ("month" = i32, Path, description = "Month of the monthly budget")
    ),
    security(("bearerAuth" = [])),
    tag = "Monthly Budget",
)]
#[get("/count/{year}/{month}")]
async fn get_monthly_budget_count(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    param: web::Path<YearMonthParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Fetch monthly budgets for the user
    let result = entity::monthly_budgets::Entity::find()
        .filter(
            Condition::all()
                .add(entity::monthly_budgets::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(entity::monthly_budgets::Column::Year.eq(param.year))
                .add(entity::monthly_budgets::Column::Month.eq(param.month)),
        )
        .order_by(entity::monthly_budgets::Column::CreatedAt, sea_orm::Order::Asc)
        .all(&app_state.db)
        .await;

    // Return the count of monthly budgets
    let count = result.unwrap().len();
    let res = monthly_budget_models::MonthlyBudgetCountResponseModel { count }; 
    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[utoipa::path(
    get,
    path = "/api/monthly_budget/{year}/{month}",
    params(
        ("year" = i32, Path, description = "Year of the monthly budget"),
        ("month" = i32, Path, description = "Month of the monthly budget")
    ),
    security(("bearerAuth" = [])),
    tag = "Monthly Budget",
)]
#[get("/{year}/{month}")]
async fn get_monthly_budget(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    param: web::Path<YearMonthParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Fetch monthly budgets for the user
    let result = entity::monthly_budgets::Entity::find()
        .filter(
            Condition::all()
                .add(entity::monthly_budgets::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(entity::monthly_budgets::Column::Year.eq(param.year))
                .add(entity::monthly_budgets::Column::Month.eq(param.month)),
        )
        .order_by(entity::monthly_budgets::Column::CreatedAt, sea_orm::Order::Asc)
        .all(&app_state.db)
        .await;
    
    if result.is_err() {
        let error_message = message::ErrorMessage::MonthlyBudgetNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    // Return the monthly budgets
    let models = result.unwrap();
    let res = monthly_budget_models::MonthlyBudgetsResponseModel {
        monthly_budgets: models.into_iter().map(|model| monthly_budget_models::MonthlyBudgetModel {
            id: model.id,
            user_id: model.user_id,
            month: model.month,
            year: model.year,
            spending_category_id: model.spending_category_id,
            budget: model.budget,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }).collect(),
    };

    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}

#[utoipa::path(
    post,
    path = "/api/monthly_budget/{year}/{month}",
    params(
        ("year" = i32, Path, description = "Year of the monthly budget"),
        ("month" = i32, Path, description = "Month of the monthly budget")
    ),
    security(("bearerAuth" = [])),
    tag = "Monthly Budget",
)]
#[post("/{year}/{month}")]
async fn create_monthly_budget(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    param: web::Path<YearMonthParam>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();

    // Delete the existing monthly budgets for the user in the specified month and year
    let result = entity::monthly_budgets::Entity::delete_many()
        .filter(
            Condition::all()
                .add(entity::monthly_budgets::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(entity::monthly_budgets::Column::Year.eq(param.year))
                .add(entity::monthly_budgets::Column::Month.eq(param.month)),
        )
        .exec(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::MonthlyBudgetNotDeleted;
        return response(
            StatusCode::InternalServerError,
            error_message.to_code(),
            Some(format!("{}: {}", error_message.to_string(), result.err().unwrap())),
            Option::<()>::None,
        );
    }

    // Fetch spending categories by user_id
    // Check if the setting exist
    let result = spending_categories::Entity::find()
        .filter(
            Condition::all()
                .add(spending_categories::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        )
        .order_by_asc(spending_categories::Column::Order)
        .all(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::SpendingCategoryNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let models = result.unwrap();
    let res = monthly_budget_models::MonthlyBudgetsResponseModel {
        monthly_budgets: models.into_iter().map(|model| monthly_budget_models::MonthlyBudgetModel {
            id: model.id,
            user_id: model.user_id,
            month: param.month,
            year: param.year,
            spending_category_id: model.id,
            budget: model.budget,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }).collect(),
    };

    // Create monthly budgets
    for _budget in &res.monthly_budgets {
        let new_monthly_budget = entity::monthly_budgets::ActiveModel {
            user_id: Set(Uuid::parse_str(user_id.as_str()).unwrap()),
            month: Set(param.month),
            year: Set(param.year),
            spending_category_id: Set(_budget.id),
            budget: Set(_budget.budget),
            ..Default::default()
        };

        if let Err(e) = new_monthly_budget.insert(&app_state.db).await {
            let error_message = message::ErrorMessage::MonthlyBudgetNotCreated;
            return response(
                StatusCode::InternalServerError,
                error_message.to_code(),
                Some(format!("{}: {}", error_message.to_string(), e)),
                Option::<()>::None,
            );
        }
    }

    // Return success response
    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Created,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(res),
    )
}