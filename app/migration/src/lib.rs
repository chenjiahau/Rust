pub use sea_orm_migration::prelude::*;

mod m20250307_030859_create_user_table;
mod m20250311_060655_create_messages_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250307_030859_create_user_table::Migration),
            Box::new(m20250311_060655_create_messages_table::Migration),
        ]
    }
}
