use sea_orm_migration::prelude::*;

use crate::m20260706_000002_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Channels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channels::Cid)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Channels::Uid).big_integer().not_null())
                    .col(
                        ColumnDef::new(Channels::InStockMarket)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-4-channels-uid")
                            .from(Channels::Table, Channels::Uid)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade) // remember to make a delete job when a user leaves the server lil jit
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Channels {
    Table,
    Cid,
    Uid,
    InStockMarket,
}
