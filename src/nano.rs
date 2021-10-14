use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db_info::{CouchDBInfo, DBInfo};
use crate::db_instance_in_use::DBInstanceInUse;
use crate::error::{CouchDBError, NanoError};

#[derive(Debug, Serialize, Deserialize)]
pub struct DBActionSuccess {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CouchDBListDBs {
    pub db_list: Vec<String>,
}

#[derive(Debug)]
pub struct Nano {
    url: String,
    client: Client,
}

impl Nano {
    pub fn new(url: &str) -> Nano {
        let new_client = Client::new();
        Nano {
            url: url.to_string(),
            client: new_client,
        }
    }

    pub async fn info(&self) -> Result<CouchDBInfo, NanoError> {
        let response = match self.client.get(&self.url).send().await {
            Ok(response) => response,
            Err(err) => return Err(NanoError::InvalidUrlOrPort(err)),
        };

        let body = match response.json::<CouchDBInfo>().await {
            Ok(body) => body,
            Err(err) => return Err(NanoError::InvalidUrlOrPort(err)),
        };
        Ok(body)
    }

    pub async fn get(&self, db_name: &str) -> Result<DBInfo, NanoError> {
        let url = &*format!("{}/{}", self.url, db_name);
        let response = match self.client.get(url).send().await {
            Ok(response) => response,
            Err(err) => return Err(NanoError::InvalidUrlOrPort(err)),
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

    // list all databases
    pub async fn list(&self) -> Result<CouchDBListDBs, NanoError> {
        // create url which couchdb will be contacted
        let url = &*format!("{}/_all_dbs", self.url);
        // make the request to couchdb
        let response = match self.client.get(url).send().await {
            Ok(response) => response,
            Err(err) => return Err(NanoError::InvalidUrlOrPort(err)),
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

    pub async fn create(&self, db_name: &str) -> Result<DBActionSuccess, NanoError> {
        // create url which couchdb will be contacted
        let url = &*format!("{}/{}", self.url, db_name);
        // make the request to couchdb
        let response = match self.client.put(url).send().await {
            Ok(response) => response,
            Err(err) => return Err(NanoError::InvalidUrlOrPort(err)),
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
                let body: DBActionSuccess = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }
    pub async fn destroy(&self, db_name: &str) -> Result<DBActionSuccess, NanoError> {
        // create url which couchdb will be contacted
        let url = &*format!("{}/{}", self.url, db_name);
        // make the request to couchdb
        let response = match self.client.delete(url).send().await {
            Ok(response) => response,
            Err(err) => return Err(NanoError::InvalidUrlOrPort(err)),
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
                let body: DBActionSuccess = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    pub fn use_db(&self, db_name: &str) -> DBInstanceInUse {
        DBInstanceInUse {
            url: self.url.clone(),
            db_name: db_name.to_string(),
            client: self.client.clone(),
        }
    }
}
