use crate::couchdb::{
    create::{self, DBActionSuccess},
    destroy, get, info,
    list::{self, CouchDBListDBs},
};
use crate::couchdb::{db_actions::db_info::DBInfo, info::CouchDBInfo};
use crate::db_in_use::DBInstanceInUse;
use crate::error::NanoError;

use reqwest::Client;

#[derive(Debug)]
pub struct Nano {
    pub url: String,
    pub client: Client,
}

impl Nano {
    pub fn new(url: &str) -> Nano {
        let new_client = Client::new();
        Nano {
            url: url.to_string(),
            client: new_client,
        }
    }

    /// get couchdb info
    pub async fn info(&self) -> Result<CouchDBInfo, NanoError> {
        Ok(info::info(&self).await?)
    }
    /// get database information
    pub async fn get(&self, db_name: &str) -> Result<DBInfo, NanoError> {
        Ok(get::get(&self, db_name).await?)
    }

    /// list all databases
    pub async fn list(&self) -> Result<CouchDBListDBs, NanoError> {
        Ok(list::list(&self).await?)
    }

    /// create a database
    pub async fn create(&self, db_name: &str) -> Result<DBActionSuccess, NanoError> {
        Ok(create::create(&self, db_name).await?)
    }
    /// delete database
    pub async fn destroy(&self, db_name: &str) -> Result<DBActionSuccess, NanoError> {
        Ok(destroy::destroy(&self, db_name).await?)
    }

    /// use a database
    pub fn use_db(&self, db_name: &str) -> DBInstanceInUse {
        DBInstanceInUse {
            url: self.url.clone(),
            db_name: db_name.to_string(),
            client: self.client.clone(),
        }
    }
}
