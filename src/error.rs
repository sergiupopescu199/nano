use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// Nano Error
#[derive(Error, Debug)]
pub enum NanoError {
    /// Error from reqwest crate which is used to make HTTP request to CouchDB server
    #[error("{0}")]
    InvalidRequest(#[from] reqwest::Error),
    /// Specific CouchDB errors which include status code and it's meaning
    #[error("Status Code: {1}, Meaning: {}, the reason is: {}",.0.error, .0.reason)]
    Unauthorized(CouchDBError, u16),
    /// Serde json Errors when parsing
    #[error("Unable to parse json: {0}")]
    InvalidJson(#[from] serde_json::Error),
    /// Generic CouchDB errors which does not include statusc code
    #[error("{0}")]
    GenericCouchdbError(Value),
}

/// CouchDB HTTP Error
#[derive(Debug, Serialize, Deserialize)]
pub struct CouchDBError {
    /// Meaning of the error gave bt CouchdB server
    pub error: String,
    /// Reason of the error in a more human redable way
    pub reason: String,
}
