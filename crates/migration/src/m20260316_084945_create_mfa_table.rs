use sea_orm_migration::prelude::*;

use crate::m20260316_082539_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Mfa::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Mfa::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Mfa::UserId).string().not_null())
                    .col(ColumnDef::new(Mfa::Secret).string().not_null())
                    .col(
                        ColumnDef::new(Mfa::Enabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Mfa::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Mfa::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Mfa::Table, Mfa::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_mfa_user_id_unique")
                    .table(Mfa::Table)
                    .col(Mfa::UserId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Mfa::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Mfa {
    Table,
    Id,
    UserId,
    Secret,
    Enabled,
    CreatedAt,
    UpdatedAt,
}
