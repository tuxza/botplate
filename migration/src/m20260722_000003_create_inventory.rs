use crate::m20260706_000002_create_users::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Inventory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Inventory::Uid).big_integer().not_null())
                    .col(ColumnDef::new(Inventory::Item).string().not_null())
                    .col(ColumnDef::new(Inventory::Description).string().not_null())
                    .col(ColumnDef::new(Inventory::Type).string().not_null())
                    .col(ColumnDef::new(Inventory::Price).decimal().not_null())
                    .col(ColumnDef::new(Inventory::Quantity).integer().not_null())
                    .primary_key(Index::create().col(Inventory::Uid).col(Inventory::Item))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-inventory-uid")
                            .from(Inventory::Table, Inventory::Uid)
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
            .drop_table(Table::drop().table(Inventory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Inventory {
    Table,
    Uid,
    Item,
    Description,
    Type,
    Price,
    Quantity,
}
