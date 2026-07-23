use sea_orm_migration::prelude::*;

use crate::m20260722_000003_create_channels::Channels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Items::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Items::Name).string().not_null())
                    .col(ColumnDef::new(Items::Description).string().not_null())
                    .col(ColumnDef::new(Items::ItemType).string().not_null())
                    .col(
                        ColumnDef::new(Items::Quantity)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Items::Price).big_integer().not_null())
                    .col(ColumnDef::new(Items::OriginCid).big_integer())
                    // nullable on purpose
                    // if the origin channel/shop is deleted item itself survives
                    // (cause people may still own the item)
                    // it just loses track of where it originally came from.
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-items-origin_cid")
                            .from(Items::Table, Items::OriginCid)
                            .to(Channels::Table, Channels::Cid)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-items-name-origin_cid")
                    .table(Items::Table)
                    .col(Items::Name)
                    .col(Items::OriginCid)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Items {
    Table,
    Id,
    Name,
    Description,
    ItemType,
    Quantity,
    Price,
    OriginCid,
}
