use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("invalid address")]
    InvalidAddress,

    #[error("error")]
    Error,
}
