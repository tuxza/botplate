pub use sea_orm_migration::prelude::*;

mod m20260705_000001_create_central_bank;
mod m20260706_000002_create_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260705_000001_create_central_bank::Migration),
            Box::new(m20260706_000002_create_users::Migration),
        ]
    }
}
