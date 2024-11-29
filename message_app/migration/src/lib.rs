pub use sea_orm_migration::prelude::*;

// mod m20220101_000001_create_table;
mod m20241120_023913_create_users_table;
mod m20241128_082308_create_roles_table;
mod m20241129_011604_create_user_role_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241120_023913_create_users_table::Migration),
            Box::new(m20241128_082308_create_roles_table::Migration),
            Box::new(m20241129_011604_create_user_role_table::Migration),
        ]
    }
}
