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
                    .table(SpendingCategories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SpendingCategories::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SpendingCategories::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SpendingCategories::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SpendingCategories::Order)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SpendingCategories::Budget)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SpendingCategories::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SpendingCategories::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SpendingCategories::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .to_owned(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_spending_category_users")
                            .from(SpendingCategories::Table, SpendingCategories::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SpendingCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SpendingCategories {
    Table,
    Id,
    UserId,
    Name,
    Order,
    Budget,
    IsDefault,
    CreatedAt,
    UpdatedAt,
}
