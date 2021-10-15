use crate::error::{CouchDBError, NanoError};
use crate::nano::Nano;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBActionSuccess {
    pub ok: bool,
}

impl DBActionSuccess {
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

pub async fn create(db: &Nano, db_name: &str) -> Result<DBActionSuccess, NanoError> {
    // create url which couchdb will be contacted
    let url = &*format!("{}/{}", db.url, db_name);
    // make the request to couchdb
    let response = db.client.put(url).send().await?;
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = response.json::<Value>().await?;

    match status {
        true => {
            let body: DBActionSuccess = serde_json::from_value(body)?;
            Ok(body)
        }
        false => {
            let body: CouchDBError = serde_json::from_value(body)?;
            Err(NanoError::Unauthorized(body, status_code))
        }
    }
}
