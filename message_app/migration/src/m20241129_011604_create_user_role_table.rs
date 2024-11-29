use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241120_023913_create_users_table::Users;
use crate::m20241128_082308_create_roles_table::Roles;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoles::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(UserRoles::UserId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_user_id")
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .col(
                        ColumnDef::new(UserRoles::RoleId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_role_id")
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                    )
                    .col(
                        date_time(UserRoles::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRoles {
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
}
