use crate::db_info::DBInfo;
use crate::error::{CouchDBError, NanoError};
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
pub struct DBInstanceInUse {
    pub url: String,
    pub db_name: String,
    pub client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBDocSuccess {
    pub ok: bool,
    pub id: String,
    pub rev: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBDocList {
    pub total_rows: i64,
    pub offset: i64,
    pub rows: Vec<Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DBFindList {
    pub docs: Vec<Value>,
    pub bookmark: String,
    pub warning: String,
}

impl DBInstanceInUse {
    pub async fn info(&self) -> Result<DBInfo, NanoError> {
        let url = &*format!("{}/{}", self.url, self.db_name);
        let response = self.client.get(url).send().await?;
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
    pub async fn insert(
        &self,
        doc_body: &Value,
        id: Option<&str>,
        rev: Option<&str>,
    ) -> Result<DBDocSuccess, NanoError> {
        let formated_url = if id.is_some() && rev.is_some() {
            format!(
                "{}/{}/{}?rev={}",
                self.url,
                self.db_name,
                id.unwrap(),
                rev.unwrap()
            )
        } else if id.is_some() && rev.is_none() {
            format!("{}/{}/{}", self.url, self.db_name, id.unwrap())
        } else {
            format!(
                "{}/{}/{}",
                self.url,
                self.db_name,
                Uuid::new_v4().to_string()
            )
        };

        let response = self.client.put(&formated_url).json(&doc_body).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body =response.json::<Value>().await?;

        match status {
            true => {
                let body: DBDocSuccess = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }
    pub async fn destroy(&self, id: &str, rev: &str) -> Result<DBDocSuccess, NanoError> {
        let formated_url = format!("{}/{}/{}?rev={}", self.url, self.db_name, id, rev);

        let response = self.client.delete(&formated_url).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: DBDocSuccess = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }
    pub async fn get(&self, id: &str, revs_info: bool) -> Result<Value, NanoError> {
        let formated_url = if revs_info {
            format!(
                "{}/{}/{}?revs_info={}",
                self.url, self.db_name, id, revs_info
            )
        } else {
            format!("{}/{}/{}", self.url, self.db_name, id)
        };

        let response = self.client.get(&formated_url).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: Value = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }
    pub async fn list(&self, revs_info: bool) -> Result<DBDocList, NanoError> {
        let formated_url = if revs_info {
            format!(
                "{}/{}/_all_docs?include_docs={}",
                self.url, self.db_name, revs_info
            )
        } else {
            format!("{}/{}/_all_docs", self.url, self.db_name)
        };

        let response = self.client.get(&formated_url).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: DBDocList = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }
    pub async fn bulk(&self, docs: &Value) -> Result<Value, NanoError> {
        let formated_url = format!("{}/{}/_bulk_docs", self.url, self.db_name);

        let response = self.client.post(&formated_url).json(docs).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        // parse the response body
        let body = response.json::<Value>().await?;
        match status {
            true => Ok(body),
            false => Err(NanoError::GenericCouchdbError(body)),
        }
    }
    pub async fn find(&self, mango_query_obj: &Value) -> Result<DBFindList, NanoError> {
        let formated_url = format!("{}/{}/_find", self.url, self.db_name);

        let response = self
		.client
		.post(&formated_url)
		.json(mango_query_obj)
		.send()
		.await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        // parse the response body
        let body = response.json::<Value>().await?;
        match status {
            true => {
                let body: DBFindList = serde_json::from_value(body)?;
                Ok(body)
            }
            false => Err(NanoError::GenericCouchdbError(body)),
        }
    }
}
