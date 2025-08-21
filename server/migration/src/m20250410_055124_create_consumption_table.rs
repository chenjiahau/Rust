use sea_orm_migration::prelude::*;

use crate::m20250328_062705_create_user_table::Users;
use crate::m20250404_233234_create_place_table::Places;
use crate::m20250409_014820_create_spending_category::SpendingCategories;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Consumptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Consumptions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Consumptions::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Consumptions::PlaceId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Consumptions::SpendingCategoryId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Consumptions::Amount)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Consumptions::Description)
                            .string()
                            .string_len(255)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Consumptions::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Consumptions::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .to_owned(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_consumptions_users")
                            .from(Consumptions::Table, Consumptions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_consumptions_places")
                            .from(Consumptions::Table, Consumptions::PlaceId)
                            .to(Places::Table, Places::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_consumptions_spending_categories")
                            .from(Consumptions::Table, Consumptions::SpendingCategoryId)
                            .to(SpendingCategories::Table, SpendingCategories::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Consumptions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Consumptions {
    Table,
    Id,
    UserId,
    PlaceId,
    SpendingCategoryId,
    Amount,
    Description,
    CreatedAt,
    UpdatedAt,
}
