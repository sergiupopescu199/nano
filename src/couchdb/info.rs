use crate::error::NanoError;
use crate::nano::Nano;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchDBInfo {
    pub couchdb: String,
    pub version: String,
    pub git_sha: String,
    pub uuid: String,
    pub features: Vec<String>,
    pub vendor: Vendor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    pub name: String,
}

impl CouchDBInfo {
    pub fn to_string_pretty(&self) -> Result<String, NanoError> {
        let u = serde_json::to_value(&self)?;
        Ok(serde_json::to_string_pretty(&u)?)
    }
    pub fn to_string(&self) -> Result<String, NanoError> {
        let u = serde_json::to_value(&self)?;
        Ok(serde_json::to_string(&u)?)
    }

    pub fn to_json(&self) -> Result<Value, NanoError> {
        Ok(serde_json::to_value(&self)?)
    }

    #[cfg(feature = "color")]
    pub fn to_colored_string(&self) -> Result<String, NanoError> {
        let u = serde_json::to_value(&self)?;
        Ok(colored_json::to_colored_json_auto(&u)?)
    }
}

pub async fn info(db: &Nano) -> Result<CouchDBInfo, NanoError> {
    let response = match db.client.get(&db.url).send().await {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };

    let body = match response.json::<CouchDBInfo>().await {
        Ok(body) => body,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    Ok(body)
}
