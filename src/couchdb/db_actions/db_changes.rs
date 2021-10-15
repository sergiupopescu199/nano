use crate::db_in_use::DBInstanceInUse;
use crate::error::NanoError;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct DBChanges {
    pub results: Vec<ChangesDoc>,
    pub last_seq: String,
    pub pending: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangesDoc {
    pub seq: String,
    pub id: String,
    pub changes: Vec<Changes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {
    pub rev: String,
}

impl DBChanges {
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

pub async fn changes(
    db: &DBInstanceInUse,
    doc_ids: Option<Vec<&str>>,
) -> Result<DBChanges, NanoError> {
    let response = if doc_ids.is_some() {
        let j = json!({
            "doc_ids": doc_ids.unwrap()
        });
        let formated_url = format!("{}/{}/_changes?filter=_doc_ids", db.url, db.db_name);

        match db.client.post(&formated_url).json(&j).send().await {
            Ok(response) => response,
            Err(err) => return Err(NanoError::InvalidRequest(err)),
        }
    } else {
        let formated_url = format!("{}/{}/_changes", db.url, db.db_name);
        match db.client.get(&formated_url).send().await {
            Ok(response) => response,
            Err(err) => return Err(NanoError::InvalidRequest(err)),
        }
    };
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    // parse the response body
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    match status {
        true => {
            let body: DBChanges = serde_json::from_value(body)?;
            Ok(body)
        }
        false => Err(NanoError::GenericCouchdbError(body)),
    }
}
