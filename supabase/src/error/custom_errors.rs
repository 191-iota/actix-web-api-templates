use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Database insertion error: {0}")]
    InsertionError(String),

    #[error("Extraction error: {0}")]
    ExtractionError(String),
}

#[derive(Error, Debug)]
pub enum UrlFetchError {
    #[error("Invalid url error: {0}")]
    InvalidUrl(String),

    #[error("Failed fetching url metadata: {0}")]
    FetchError(String),
}
