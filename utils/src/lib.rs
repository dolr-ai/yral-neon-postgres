#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

pub type Result<T> = std::result::Result<T, Error>;
