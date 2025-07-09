use sea_orm_migration::{prelude::*, schema::*};

use crate::table_idens::SatsAirdropData;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SatsAirdropData::Table)
                    .if_not_exists()
                    .col(string(SatsAirdropData::UserPrincipal).primary_key())
                    .col(timestamp(SatsAirdropData::LastAirdropAt))
                    .col(big_integer(SatsAirdropData::TotalSats))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SatsAirdropData::Table).to_owned())
            .await
    }
}
