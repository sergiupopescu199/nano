use crate::{
    db_in_use::DBInstanceInUse,
    error::{CouchDBError, NanoError},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBInfo {
    pub db_name: String,
    pub purge_seq: String,
    pub update_seq: String,
    pub sizes: Sizes,
    pub props: Props,
    pub doc_del_count: i64,
    pub doc_count: i64,
    pub disk_format_version: i64,
    pub compact_running: bool,
    pub cluster: Cluster,
    pub instance_start_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub q: i64,
    pub n: i64,
    pub w: i64,
    pub r: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sizes {
    pub file: i64,
    pub external: i64,
    pub active: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Props {
    pub partitioned: Option<bool>,
}

impl DBInfo {
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

pub async fn info(db: &DBInstanceInUse) -> Result<DBInfo, NanoError> {
    let url = format!("{}/{}", db.url, db.db_name);
    let response = db.client.get(&url).send().await?;
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = response.json::<Value>().await?;

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
