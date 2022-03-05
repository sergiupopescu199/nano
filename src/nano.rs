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
    pub fn new<S>(url: S) -> Nano
    where
        S: Into<String>,
    {
        let new_client = Client::new();
        Nano {
            url: url.into(),
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
    pub async fn create<S>(
        &self,
        db_name: S,
        partitioned: bool,
    ) -> Result<DBActionSuccess, NanoError>
    where
        S: AsRef<str>,
    {
        Ok(create::create(&self, db_name.as_ref(), partitioned).await?)
    }
    /// delete database
    pub async fn destroy<S>(&self, db_name: S) -> Result<DBActionSuccess, NanoError>
    where
        S: AsRef<str>,
    {
        Ok(destroy::destroy(&self, db_name.as_ref()).await?)
    }

    /// use a database
    pub fn use_db<S>(&self, db_name: S) -> DBInstanceInUse
    where
        S: Into<String>,
    {
        DBInstanceInUse {
            url: self.url.clone(),
            db_name: db_name.into(),
            client: self.client.clone(),
        }
    }
}
