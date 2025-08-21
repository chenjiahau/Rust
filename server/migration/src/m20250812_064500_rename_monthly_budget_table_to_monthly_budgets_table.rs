use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table(MonthlyBudgetOld::Table, MonthlyBudgetsNew::Table)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table(MonthlyBudgetsNew::Table, MonthlyBudgetOld::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum MonthlyBudgetOld {
    #[sea_orm(iden = "monthly_budget")]
    Table,
}

#[derive(DeriveIden)]
enum MonthlyBudgetsNew {
    #[sea_orm(iden = "monthly_budgets")]
    Table,
}