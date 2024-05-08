use std::{result, str::Utf8Error};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PHPer(#[from] phper::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::Anyhow(e.into())
    }
}

pub type Result<T> = result::Result<T, Error>;