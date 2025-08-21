use sea_orm_migration::prelude::*;

use crate::m20250409_014820_create_spending_category::SpendingCategories;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Add column
        manager
            .alter_table(
                Table::alter()
                    .table(Places::Table)
                    .add_column(
                        ColumnDef::new(Places::SpendingCategoryId)
                            .big_integer()
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .to_owned(),
        )
        .await?;

        // 2. Add foreign key constraint separately
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_places_sc_id")
                    .from(Places::Table, Places::SpendingCategoryId)
                    .to(SpendingCategories::Table, SpendingCategories::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
         // 1. Drop the foreign key constraint
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_places_sc_id")
                    .table(Places::Table)
                    .to_owned(),
        )
        .await?;

        // 2. Drop the column
        manager
            .alter_table(
                Table::alter()
                    .table(Places::Table)
                    .drop_column(Places::SpendingCategoryId)
                    .to_owned(),
        )
        .await
    }
}

#[derive(DeriveIden)]
enum Places {
    Table,
    SpendingCategoryId,
}
