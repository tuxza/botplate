// okay i ACTUALLY might not need this.
// im gonna keep it AROUND but
// yea

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Items::Table)
                    .add_column(big_integer(Items::ItemId).default(0))
                    .to_owned(),
            )
            .await?;
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_column(
                Table::drop()
                    .table(Items::Table)
                    .column(Items::ItemId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Items {
    Table,
    ItemId,
}
