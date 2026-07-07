use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table_name("inventory")
                    .col(ColumnDef::new(Inventory::UID).big_integer().not_null())
                    .col(ColumnDef::new(Inventory::Item).string().not_null())
                    .col(ColumnDef::new(Inventory::Description).string().not_null())
                    .col(ColumnDef::new(Inventory::Type).string().not_null())
                    .col(ColumnDef::new(Inventory::Price).decimal().not_null())
                    .col(ColumnDef::new(Inventory::Quantity).integer().not_null())
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

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityName)]
struct Inventory {
    uid: i64,
    item: String,
    description: String,
    type_: String,
    price: Decimal,
    quantity: i32,
}
