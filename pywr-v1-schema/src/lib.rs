pub mod edge;
pub mod model;
pub mod nodes;
pub mod parameters;
pub mod tables;

pub use model::{PywrModel, PywrNetwork};
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PywrSchemaError {
    #[error("An invalid URL was found.")]
    InvalidUrlFound,
    #[error("data store disconnected")]
    IoError(#[from] io::Error),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Resource not found on local host: {0}")]
    LocalResourceNotFound(PathBuf),
    #[error("Invalid Pywr format")]
    InvalidPywrDataFormat,
}
