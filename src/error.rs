use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NanoError {
    #[error("unable to connect to db")]
    InvalidUrlOrPort(#[from] reqwest::Error),
    #[error("Status Code: {1}, Meaning: {}, the reason is: {}",.0.error, .0.reason)]
    Unauthorized(CouchDBError, u16),
    #[error("Unable to parse json: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("{0}")]
    GenericCouchdbError(Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchDBError {
    pub error: String,
    pub reason: String,
}
