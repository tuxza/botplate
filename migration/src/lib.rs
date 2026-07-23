pub use sea_orm_migration::prelude::*;

mod m20260705_000001_create_central_bank;
mod m20260706_000002_create_users;
mod m20260722_000003_create_channels;
mod m20260722_000004_create_items;
mod m20260722_000005_create_inventory;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260705_000001_create_central_bank::Migration),
            Box::new(m20260706_000002_create_users::Migration),
            Box::new(m20260722_000003_create_channels::Migration),
            Box::new(m20260722_000004_create_items::Migration),
            Box::new(m20260722_000005_create_inventory::Migration),
        ]
    }
}
