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
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Settings::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Settings::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Settings::DefaultIncome)
                            .big_integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Settings::TargetDeposit)
                            .big_integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Settings::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Settings::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .to_owned(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_settings_users")
                            .from(Settings::Table, Settings::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Id,
    UserId,
    DefaultIncome,
    TargetDeposit,
    CreatedAt,
    UpdatedAt,
}
