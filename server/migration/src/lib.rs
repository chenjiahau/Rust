pub use sea_orm_migration::prelude::*;

mod m20250328_062705_create_user_table;
mod m20250329_003732_create_token_table;
mod m20250402_235932_create_setting_table;
mod m20250404_233234_create_place_table;
mod m20250409_014820_create_spending_category;
mod m20250410_055124_create_consumption_table;
mod m20250625_055116_add_image_column_to_users_table;
mod m20250726_000853_connect_spending_category_and_place_table;
mod m20250808_055617_add_date_column_to_consumption_table;
mod m20250808_105138_create_monthly_budget_table;
mod m20250812_064500_rename_monthly_budget_table_to_monthly_budgets_table;
mod m20250812_070832_create_monthly_settings_table;
mod m20250912_235136_add_security_password_column_to_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250328_062705_create_user_table::Migration),
            Box::new(m20250329_003732_create_token_table::Migration),
            Box::new(m20250402_235932_create_setting_table::Migration),
            Box::new(m20250404_233234_create_place_table::Migration),
            Box::new(m20250409_014820_create_spending_category::Migration),
            Box::new(m20250410_055124_create_consumption_table::Migration),
            Box::new(m20250625_055116_add_image_column_to_users_table::Migration),
            Box::new(m20250726_000853_connect_spending_category_and_place_table::Migration),
            Box::new(m20250808_055617_add_date_column_to_consumption_table::Migration),
            Box::new(m20250808_105138_create_monthly_budget_table::Migration),
            Box::new(m20250812_064500_rename_monthly_budget_table_to_monthly_budgets_table::Migration),
            Box::new(m20250812_070832_create_monthly_settings_table::Migration),
            Box::new(m20250912_235136_add_security_password_column_to_users_table::Migration),
        ]
    }
}
