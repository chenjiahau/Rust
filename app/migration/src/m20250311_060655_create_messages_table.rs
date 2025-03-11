use sea_orm_migration::prelude::*;

use crate::m20250307_030859_create_user_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Messages::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Messages::UserId).uuid().not_null())
                    .col(ColumnDef::new(Messages::Message).string().not_null())
                    .col(ColumnDef::new(Messages::CreatedAt).date_time().not_null().default(Expr::current_timestamp())).
                    foreign_key(
                        ForeignKey::create()
                            .name("fk_message_user")
                            .from(Messages::Table, Messages::UserId)
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
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    Id,
    UserId,
    Message,
    CreatedAt,
}