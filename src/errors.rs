use anyhow::anyhow;
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

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::Anyhow(anyhow!("{}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Anyhow(anyhow!("{}", e))
    }
}

pub type Result<T> = result::Result<T, Error>;
