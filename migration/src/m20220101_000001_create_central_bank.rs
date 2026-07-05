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

        // Legacy rescue: the central bank used to live in system_settings.
        // Only run the data migration if that table actually exists, otherwise
        // a fresh install (no system_settings) would error with
        // "no such table: system_settings" and abort the migration.
        let legacy_exists = manager
            .get_connection()
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                r#"SELECT name FROM sqlite_master WHERE type='table' AND name='system_settings';"#,
            ))
            .await?
            .is_some();

        if legacy_exists {
            manager
                .get_connection()
                .execute(Statement::from_string(
                    DbBackend::Sqlite,
                    r#"
                    INSERT INTO central_bank (id, balance)
                    SELECT
                        1,
                        CAST(value AS INTEGER)
                    FROM system_settings
                    WHERE key = 'central_bank';
                    "#,
                ))
                .await?;
        }

        // Seed a default row if nothing was migrated in.
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
