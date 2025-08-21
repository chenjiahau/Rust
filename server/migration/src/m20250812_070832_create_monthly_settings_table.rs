use sea_orm_migration::prelude::*;

use crate::m20250328_062705_create_user_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MonthlySettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MonthlySettings::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MonthlySettings::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlySettings::Month)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlySettings::Year)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlySettings::Income)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlySettings::Deposit)
                            .double()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(MonthlySettings::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MonthlySettings::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_monthly_settings_users")
                            .from(MonthlySettings::Table, MonthlySettings::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
         manager
            .drop_table(Table::drop().table(MonthlySettings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MonthlySettings {
    Table,
    Id,
    UserId,
    Month,
    Year,
    Income,
    Deposit,
    CreatedAt,
    UpdatedAt,
}
