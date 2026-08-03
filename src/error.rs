use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Ics(#[from] IcsError),

    #[error("failed to create output directory {0}: {1}")]
    CreateOutputDirectory(PathBuf, #[source] io::Error),

    #[error("failed to create HTTP client: {0}")]
    CreateHttpClient(#[from] reqwest::Error),

    #[error("failed to create download thread pool: {0}")]
    CreateThreadPool(#[from] rayon::ThreadPoolBuildError),
}

#[derive(Debug, Error)]
pub enum IcsError {
    #[error("failed to read ICS file {0}: {1}")]
    ReadFile(PathBuf, #[source] io::Error),

    #[error("Invalid ICS file: {0} (missing or empty)")]
    EmptyFile(PathBuf),

    #[error("failed to parse ICS calendar: {0}")]
    Parse(#[from] ical::parser::ParserError),
}

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("failed to request {0}: {1}")]
    Request(String, #[source] reqwest::Error),

    #[error("unsuccessful response from {0}: {1}")]
    Response(String, #[source] reqwest::Error),

    #[error("failed to create output file {0}: {1}")]
    CreateFile(PathBuf, #[source] io::Error),

    #[error("failed to write output file {0}: {1}")]
    WriteFile(PathBuf, #[source] io::Error),

    #[error("failed to remove incomplete output file {0}: {1}")]
    RemoveIncompleteFile(PathBuf, #[source] io::Error),
}

#[derive(Debug, Error)]
pub enum WorkerCountError {
    #[error("workers must be a positive integer")]
    InvalidInteger,

    #[error("workers must be at least 1")]
    Zero,
}
