use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Consumptions::Table)
                    .add_column(ColumnDef::new(Consumptions::Date)
                        .char_len(8)
                        .null())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Consumptions::Table)
                    .drop_column(Consumptions::Date)
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Consumptions {
    Table,
    Date,
}