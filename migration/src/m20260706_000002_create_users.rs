use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer() // remember kids we're using sqlite! this TECHNICALLY doesnt matter! .. i do it anyways
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Tokens)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Users::Debt)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Users::LastDaily).big_integer())
                    .col(ColumnDef::new(Users::LastJob).big_integer())
                    .col(
                        ColumnDef::new(Users::XP)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Users::Level)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Users::Spouse).big_integer())
                    .col(ColumnDef::new(Users::SpouseSince).big_integer())
                    .col(ColumnDef::new(Users::JointBalance).big_integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Tokens,
    Debt,
    LastDaily,
    LastJob,
    XP,
    Level,
    Spouse,
    SpouseSince,
    JointBalance,
}
