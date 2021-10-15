use crate::couchdb::create::DBActionSuccess;
use crate::error::{CouchDBError, NanoError};
use crate::nano::Nano;

use serde_json::Value;

pub async fn destroy(db: &Nano, db_name: &str) -> Result<DBActionSuccess, NanoError> {
    // create url which couchdb will be contacted
    let url = &*format!("{}/{}", db.url, db_name);
    // make the request to couchdb
    let response = db.client.delete(url).send().await?;
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
