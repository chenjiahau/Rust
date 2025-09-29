use utoipa::openapi::security::{HttpAuthScheme, SecurityScheme, HttpBuilder};
use utoipa::{Modify, OpenApi};

use crate::handlers;
use crate::models;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::unauth_handlers::signup,
        handlers::unauth_handlers::signin,
        handlers::unauth_handlers::create_new_password_by_email,
        handlers::user_handlers::get_user_profile,
        handlers::user_handlers::update_user_profile,
        handlers::user_handlers::update_user_password,
        handlers::setting_handlers::get_setting,
        handlers::setting_handlers::update_setting,
        handlers::setting_handlers::update_s_to_current_month,
        handlers::spending_category_handlers::get_spending_categories,
        handlers::spending_category_handlers::get_spending_category,
        handlers::spending_category_handlers::create_spending_category,
        handlers::spending_category_handlers::update_spending_category,
        handlers::spending_category_handlers::delete_spending_category,
        handlers::spending_category_handlers::update_sc_to_current_month,
        handlers::place_handlers::get_places,
        handlers::place_handlers::create_place,
        handlers::place_handlers::update_place,
        handlers::place_handlers::delete_place,
        handlers::consumption_handlers::get_consumptions,
        handlers::consumption_handlers::get_consumption,
        handlers::consumption_handlers::get_consumptions_by_year_and_month,
        handlers::consumption_handlers::create_consumption,
        handlers::consumption_handlers::update_consumption,
        handlers::consumption_handlers::delete_consumption,
        handlers::monthly_budget_handlers::get_monthly_budget_count,
        handlers::monthly_budget_handlers::get_monthly_budget,
        handlers::monthly_budget_handlers::create_monthly_budget,
        handlers::monthly_setting_handlers::get_monthly_setting_count,
        handlers::monthly_setting_handlers::get_monthly_setting,
        handlers::monthly_setting_handlers::create_monthly_setting,
        handlers::statistic_handlers::get_monthly_report,
    ),
    components(
        schemas(
            models::unauth_models::TokenModel,
            models::unauth_models::SignupRequestModel,
            models::unauth_models::SignupResponseModel,
            models::unauth_models::SigninRequestModel,
            models::unauth_models::SigninResponseModel,
            models::unauth_models::ForgotPasswordRequestModel,
            models::user_models::UserProfileUpdateRequestModel,
            models::user_models::UserPasswordUpdateRequestModel,
            models::setting_models::SettingRequestModel,
            models::spending_category_models::SpendingCategoryRequestModel,
            models::place_models::PlaceRequestModel,
            models::consumption_models::ConsumptionRequestModel,
            models::statistic_models::MonthlyReportResponseModel,
        )
    ),
    modifiers(&SecurityAddon) 
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut components = openapi.components.take().unwrap_or_default();

        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            ),
        );

        openapi.components = Some(components);
    }
}