use sea_orm_migration::sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum SatsAirdropData {
    Table,
    UserPrincipal,
    LastAirdropAt,
    TotalSats,
}
