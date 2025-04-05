pub use sea_orm_migration::prelude::*;

mod m20250328_062705_create_user_table;
mod m20250329_003732_create_token_table;
mod m20250402_235932_create_setting_table;
mod m20250404_233234_create_place_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250328_062705_create_user_table::Migration),
            Box::new(m20250329_003732_create_token_table::Migration),
            Box::new(m20250402_235932_create_setting_table::Migration),
            Box::new(m20250404_233234_create_place_table::Migration),
        ]
    }
}
