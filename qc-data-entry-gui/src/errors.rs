use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("missing product")]
    MissingProduct,
    #[error("missing lot number")]
    MissingLot,
    #[error("missing tester")]
    MissingTester,
    #[error(transparent)]
    DatabaseLogicError(#[from] nwg::NwgError),
}

pub type Result<T> = std::result::Result<T, Error>;
// pub type Result<T, E = Error> = core::result::Result<T, E>;
