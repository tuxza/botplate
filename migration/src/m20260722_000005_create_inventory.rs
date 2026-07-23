use crate::m20260706_000002_create_users::Users;
use crate::m20260722_000004_create_items::Items;
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
                    .col(ColumnDef::new(Inventory::ItemId).big_integer().not_null())
                    .col(ColumnDef::new(Inventory::Quantity).big_integer().not_null())
                    // what THIS user paid when they acquired it -- a snapshot, not
                    // a live price, since the shop's listing price can move on
                    // without changing what past buyers already paid.
                    .col(
                        ColumnDef::new(Inventory::AcquiredPrice)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Inventory::CanResell)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .primary_key(Index::create().col(Inventory::Uid).col(Inventory::ItemId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-inventory-uid")
                            .from(Inventory::Table, Inventory::Uid)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-inventory-item_id")
                            .from(Inventory::Table, Inventory::ItemId)
                            .to(Items::Table, Items::Id)
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
    ItemId,
    Quantity,
    AcquiredPrice,
    CanResell,
}
