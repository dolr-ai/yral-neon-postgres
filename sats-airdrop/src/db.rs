use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::Migrator;

#[derive(Debug, Clone)]
pub struct SatsAirdrop(pub DatabaseConnection);

impl SatsAirdrop {
    pub async fn connect_and_migrate(config: impl Into<ConnectOptions>) -> utils::Result<Self> {
        let db = Database::connect(config).await?;

        Migrator::up(&db, None).await?;

        Ok(Self(db))
    }
}
