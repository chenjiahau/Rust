pub use sea_orm_migration::prelude::*;

mod m20250328_062705_create_user_table;
mod m20250329_003732_create_token_table;
mod m20250402_235932_create_setting_table;
mod m20250404_233234_create_place_table;
mod m20250409_014820_create_spending_category;
mod m20250410_055124_create_consumption_table;
mod m20250625_055116_add_image_column_to_users_table;

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
        ]
    }
}
