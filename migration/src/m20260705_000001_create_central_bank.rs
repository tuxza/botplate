use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CentralBank::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CentralBank::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CentralBank::Balance)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Sqlite,
                r#"
                INSERT OR IGNORE INTO central_bank (id, balance)
                VALUES (1, 0);
                "#,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CentralBank::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum CentralBank {
    Table,
    Id,
    Balance,
}
