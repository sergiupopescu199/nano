use crate::error::{CouchDBError, NanoError};
use crate::nano::Nano;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchDBListDBs {
    pub db_list: Vec<String>,
}

pub async fn list(db: &Nano) -> Result<CouchDBListDBs, NanoError> {
    // create url which couchdb will be contacted
    let url = format!("{}/_all_dbs", db.url);
    // make the request to couchdb
    let response = db.client.get(&url).send().await?;
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = response.json::<Value>().await?;

    match status {
        true => {
            let body = json!({ "db_list": body });
            let body: CouchDBListDBs = serde_json::from_value(body)?;
            Ok(body)
        }
        false => {
            let body: CouchDBError = serde_json::from_value(body)?;
            Err(NanoError::Unauthorized(body, status_code))
        }
    }
}
