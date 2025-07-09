pub use sea_orm_migration::prelude::*;

pub mod db;
pub mod entities;

pub(crate) mod table_idens;
pub struct Migrator;

mod m20250709_133300_create_sats_airdrop_data;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20250709_133300_create_sats_airdrop_data::Migration,
        )]
    }
}
