use crate::error::CouchDBError;
use crate::{db_in_use::DBInstanceInUse, error::NanoError};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

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

impl DBDocSuccess {
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
impl DBDocList {
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
impl DBFindList {
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

pub async fn insert(
    db: &DBInstanceInUse,
    doc_body: &Value,
    id: Option<&str>,
    rev: Option<&str>,
) -> Result<DBDocSuccess, NanoError> {
    let formated_url = if id.is_some() && rev.is_some() {
        format!(
            "{}/{}/{}?rev={}",
            db.url,
            db.db_name,
            id.unwrap(),
            rev.unwrap()
        )
    } else if id.is_some() && rev.is_none() {
        format!("{}/{}/{}", db.url, db.db_name, id.unwrap())
    } else {
        format!("{}/{}/{}", db.url, db.db_name, Uuid::new_v4().to_string())
    };

    let response = match db.client.put(&formated_url).json(&doc_body).send().await {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };

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

pub async fn destroy(db: &DBInstanceInUse, id: &str, rev: &str) -> Result<DBDocSuccess, NanoError> {
    let formated_url = format!("{}/{}/{}?rev={}", db.url, db.db_name, id, rev);

    let response = match db.client.delete(&formated_url).send().await {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };

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
pub async fn get(db: &DBInstanceInUse, id: &str, revs_info: bool) -> Result<Value, NanoError> {
    let formated_url = if revs_info {
        format!("{}/{}/{}?revs_info={}", db.url, db.db_name, id, revs_info)
    } else {
        format!("{}/{}/{}", db.url, db.db_name, id)
    };

    let response = match db.client.get(&formated_url).send().await {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };

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
pub async fn list(db: &DBInstanceInUse, revs_info: bool) -> Result<DBDocList, NanoError> {
    let formated_url = if revs_info {
        format!(
            "{}/{}/_all_docs?include_docs={}",
            db.url, db.db_name, revs_info
        )
    } else {
        format!("{}/{}/_all_docs", db.url, db.db_name)
    };

    let response = match db.client.get(&formated_url).send().await {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    let status_code = response.status().as_u16();
    // parse the response body
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };

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
pub async fn bulk(db: &DBInstanceInUse, docs: &Value) -> Result<Value, NanoError> {
    let formated_url = format!("{}/{}/_bulk_docs", db.url, db.db_name);

    let response = match db.client.post(&formated_url).json(docs).send().await {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    // check the status code if it's in range from 200-299
    let status = response.status().is_success();
    // parse the response body
    let body = match response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
    };
    match status {
        true => Ok(body),
        false => Err(NanoError::GenericCouchdbError(body)),
    }
}
pub async fn find(db: &DBInstanceInUse, mango_query_obj: &Value) -> Result<DBFindList, NanoError> {
    let formated_url = format!("{}/{}/_find", db.url, db.db_name);

    let response = match db
        .client
        .post(&formated_url)
        .json(mango_query_obj)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(NanoError::InvalidRequest(err)),
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
            let body: DBFindList = serde_json::from_value(body)?;
            Ok(body)
        }
        false => Err(NanoError::GenericCouchdbError(body)),
    }
}
