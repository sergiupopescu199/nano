use crate::couchdb::db_actions::db_info::DBInfo;
use crate::error::{CouchDBError, NanoError};
use crate::nano::Nano;

use serde_json::Value;

pub async fn get(db: &Nano, db_name: &str) -> Result<DBInfo, NanoError> {
    let url = format!("{}/{}", db.url, db_name);
    let response = match db.client.get(&url).send().await {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(err.into()),
    };

    match status {
        true => {
            let body: DBInfo = serde_json::from_value(body)?;
            Ok(body)
        }
        false => {
            let body: CouchDBError = serde_json::from_value(body)?;
            Err(NanoError::Unauthorized(body, status_code))
        }
    }
}
