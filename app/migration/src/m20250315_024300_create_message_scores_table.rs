use sea_orm_migration::prelude::*;

use crate::m20250311_060655_create_messages_table::Messages;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MessageScores::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MessageScores::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(MessageScores::MessageId).integer().not_null())
                    .col(ColumnDef::new(MessageScores::Score).unsigned().integer().not_null())
                    .col(ColumnDef::new(MessageScores::CreatedAt).date_time().not_null().default(Expr::current_timestamp())).
                    foreign_key(
                        ForeignKey::create()
                            .name("fk_message_score_message")
                            .from(MessageScores::Table, MessageScores::MessageId)
                            .to(Messages::Table, Messages::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageScores::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MessageScores {
    Table,
    Id,
    MessageId,
    Score,
    CreatedAt,
}
