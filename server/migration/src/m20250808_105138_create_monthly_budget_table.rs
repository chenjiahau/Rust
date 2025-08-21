use sea_orm_migration::prelude::*;

use crate::m20250328_062705_create_user_table::Users;
use crate::m20250409_014820_create_spending_category::SpendingCategories;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MonthlyBudget::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MonthlyBudget::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MonthlyBudget::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlyBudget::Month)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlyBudget::Year)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlyBudget::SpendingCategoryId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MonthlyBudget::Budget)
                            .double()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(MonthlyBudget::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MonthlyBudget::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_monthly_budget_users")
                            .from(MonthlyBudget::Table, MonthlyBudget::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_monthly_budget_spending_category")
                            .from(MonthlyBudget::Table, MonthlyBudget::SpendingCategoryId)
                            .to(SpendingCategories::Table, SpendingCategories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MonthlyBudget::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MonthlyBudget {
    Table,
    Id,
    UserId,
    Month,
    Year,
    SpendingCategoryId,
    Budget,
    CreatedAt,
    UpdatedAt,
}
