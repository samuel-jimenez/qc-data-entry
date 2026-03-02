use thiserror::Error;

use crate::db;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseLogicError(#[from] db::Error),
    #[error("database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error(transparent)]
    VersionParse(#[from] semver::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
// pub type Result<T, E = Error> = core::result::Result<T, E>;
