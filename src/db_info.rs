use crate::error::NanoError;
use serde::{Deserialize, Serialize};

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
        let u = serde_json::to_value(self.clone())?;
        Ok(serde_json::to_string_pretty(&u)?)
    }
    pub fn to_string(&self) -> Result<String, NanoError> {
        let u = serde_json::to_value(self.clone())?;
        Ok(serde_json::to_string(&u)?)
    }
}
impl CouchDBInfo {
    pub fn to_string_pretty(&self) -> Result<String, NanoError> {
        let u = serde_json::to_value(self.clone())?;
        Ok(serde_json::to_string_pretty(&u)?)
    }
    pub fn to_string(&self) -> Result<String, NanoError> {
        let u = serde_json::to_value(self.clone())?;
        Ok(serde_json::to_string(&u)?)
    }
}
