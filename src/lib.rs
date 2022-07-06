use bevy_reflect::Reflect;
#[cfg(feature = "color")]
pub use colored_json;
pub mod database;
pub use error::NanoError;
mod error;
use crate::database::types::{DBInUse, DBOperationSuccess};
use error::CouchDBError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub trait Convert {
    /// Convert to string and indent
    fn to_string_pretty(&self) -> Result<String, NanoError>
    where
        Self: Serialize,
    {
        let u = serde_json::to_value(&self)?;
        Ok(serde_json::to_string_pretty(&u)?)
    }
    /// Convert to string
    fn to_string(&self) -> Result<String, NanoError>
    where
        Self: Serialize,
    {
        let u = serde_json::to_value(&self)?;
        Ok(serde_json::to_string(&u)?)
    }
    /// Convert to json value
    fn to_json(&self) -> Result<Value, NanoError>
    where
        Self: Serialize,
    {
        Ok(serde_json::to_value(&self)?)
    }
    /// Convert to string, indent and color it
    #[cfg(feature = "color")]
    fn to_colored_string(&self) -> Result<String, NanoError>
    where
        Self: Serialize,
    {
        let u = serde_json::to_value(&self)?;
        Ok(colored_json::to_colored_json_auto(&u)?)
    }
}

impl Convert for CouchDBInfo {}

pub trait ParseQueryParams: bevy_reflect::Struct {
    /// Parse Struct keys and values into a HTTP query string
    fn parse_params(&self) -> String {
        let mut params = "".to_string();
        // iterate for every key of teh struct
        for (index, value) in self.iter_fields().enumerate() {
            // get field name
            let field_name = self.name_at(index).unwrap();
            // based on value get it's value
            let value_formatted = self.get_value(value);
            // check value data and exluce if bool type is false and if string is empty
            if !value_formatted.eq("false")
                && !value_formatted.is_empty()
                && !value_formatted.eq("0")
            {
                params.push_str(&format!("{}={}&", field_name, value_formatted));
            }
        }
        params
    }
    /// Based on value type get the actual value as a String
    fn get_value(&self, value: &dyn Reflect) -> String {
        match value.type_name() {
            "bool" => value.downcast_ref::<bool>().unwrap().to_string(),
            "i64" => value.downcast_ref::<i64>().unwrap().to_string(),
            "alloc::string::String" => value.downcast_ref::<String>().unwrap().to_owned(),
            _ => "".to_string(),
        }
    }
}

/// List all databases present on CouchDB node
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouchDBListDBs {
    /// List of all databases present on the CouchDB node
    pub db_list: Vec<String>,
}

/// CouchDB node information
/// ```
///    {
///         "couchdb": "Welcome",
///         "features": [
///             "access-ready",
///             "partitioned",
///             "pluggable-storage-engines",
///             "reshard",
///             "scheduler"
///         ],
///         "git_sha": "572b68e72",
///         "uuid": "7ecbe8fcc2cde610fe02ee82df51cbf7",
///         "vendor": {
///             "name": "The Apache Software Foundation"
///         },
///         "version": "3.1.2"
///    }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouchDBInfo {
    pub couchdb: String,
    /// CouchDB node version
    pub version: String,
    /// Git hash
    pub git_sha: String,
    /// Unique uuid of CouchDB node
    pub uuid: String,
    /// Enabled features
    pub features: Vec<String>,
    /// Custom vendor description
    pub vendor: Vendor,
}

/// Custom vendor description
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vendor {
    /// Vendor name and description
    pub name: String,
}

/// CouchDB node
#[derive(Debug, Clone)]
pub struct Nano {
    /// # Example
    /// ```
    /// http://<user>:<password>@<url>:<port>
    /// ```
    pub url: String,
    pub client: Client,
}

impl Nano {
    /// Connect to a new CouchDB node
    /// # Example
    /// ```
    /// let db = Nano::new("http://dev:dev@localhost:5984");
    /// ```
    pub fn new<S>(url: S) -> Nano
    where
        S: Into<String>,
    {
        Nano {
            url: url.into(),
            client: Client::new(),
        }
    }

    /// Get CouchDB node information
    /// # Example
    /// ```
    /// // connect to a CouchDB node
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// // retrive node info
    /// let node_info = nano.get_node_info().await?;
    ///
    /// ```
    pub async fn get_node_info(&self) -> Result<CouchDBInfo, NanoError> {
        let response = self.client.get(&self.url).send().await?;
        Ok(response.json::<CouchDBInfo>().await?)
    }

    /// list all databases
    /// # Example
    /// ```
    /// // connect to a CouchDB node
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// // retrive all dbs from CouchDB server
    /// let present_dbs = nano.all_dbs().await?;
    ///
    /// ```
    pub async fn all_dbs(&self) -> Result<CouchDBListDBs, NanoError> {
        // create url which couchdb will be contacted
        let url = format!("{}/_all_dbs", self.url);
        // make the request to couchdb
        let response = self.client.get(&url).send().await?;
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

    /// Create a new database
    ///
    /// The database name **must** be composed by following next rules:
    /// - Name **must** begin with a lowercase letter `(a-z)`
    /// - Lowercase characters `(a-z)`
    /// - Digits `(0-9)`
    /// - Any of the characters `_, $, (, ), +, -,` and `/`.
    ///
    /// If you’re familiar with Regular Expressions, the rules above could be written as `^[a-z][a-z0-9_$()+/-]*$`.
    ///
    /// # Example
    /// ```
    /// // connect to a CouchDB node
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// // create a not partitioned db
    /// let my_new_db_res = nano.create_db("my_new_db", false).await?;
    /// // create a partitioned db
    /// let my_new_db_partittioned_res = nano.create_db("my_new_db_partitioned", true).await?;
    ///
    /// ```
    pub async fn create_db<S>(
        &self,
        db_name: S,
        partitioned: bool,
    ) -> Result<DBOperationSuccess, NanoError>
    where
        S: Into<String>,
    {
        // create url which couchdb will be contacted
        let formated_url = if partitioned {
            format!(
                "{}/{}?partitioned={}",
                self.url,
                db_name.into(),
                partitioned
            )
        } else {
            format!("{}/{}", self.url, db_name.into())
        };
        // make the request to couchdb
        let response = self.client.put(&formated_url).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: DBOperationSuccess = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// Deletes the specified database, and all the documents and attachments contained within it.
    /// # Example
    /// ```
    /// // connect to a CouchDB node
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// // create a not partitioned db
    /// let my_new_db_res = nano.create_db("my_new_db", false).await?;
    /// // delete the newly created db
    /// let delete_db_res = nano.delete_db("my_new_db").await?;
    ///
    /// ```
    pub async fn delete_db<S>(&self, db_name: S) -> Result<DBOperationSuccess, NanoError>
    where
        S: Into<String>,
    {
        // create url which couchdb will be contacted
        let url = format!("{}/{}", self.url, db_name.into());
        // make the request to couchdb
        let response = self.client.delete(url.as_str()).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: DBOperationSuccess = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// Connect to a database
    /// # Example
    /// ```
    /// // connect to a CouchDB node
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// // create a not partitioned db
    /// let my_new_db_res = nano.create_db("my_new_db", false).await?;
    /// // connect to the newly created db in order to create/delete & modify documents
    /// let my_new_db = nano.connect_to_db("my_new_db")
    /// ```
    pub fn connect_to_db<S>(&self, db_name: S) -> DBInUse
    where
        S: Into<String>,
    {
        DBInUse {
            url: self.url.clone(),
            db_name: db_name.into(),
            client: self.client.clone(),
        }
    }
    /// Create a database if it does not exists and connecto to it
    /// # Example
    /// ```
    /// // connect to a CouchDB node
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// // connect to the newly created db
    /// let my_new_db = nano.create_and_connect_to_db("my_new_db").await;
    /// ```
    pub async fn create_and_connect_to_db<S>(&self, db_name: S, partitioned: bool) -> DBInUse
    where
        S: Into<String>,
    {
        let db_name = db_name.into();
        match self.create_db(&db_name, partitioned).await {
            Ok(_) => DBInUse {
                url: self.url.clone(),
                db_name: db_name,
                client: self.client.clone(),
            },
            Err(_) => DBInUse {
                url: self.url.clone(),
                db_name: db_name,
                client: self.client.clone(),
            },
        }
    }
}
