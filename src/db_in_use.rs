use crate::couchdb::db_actions::{
    db_changes::{self, DBChanges},
    db_doc::{self, DBDocList, DBDocSuccess, DBFindList},
    db_info::{self, DBInfo},
};
use crate::error::NanoError;

use reqwest::{self, Client};
use serde_json::Value;

pub struct DBInstanceInUse {
    pub url: String,
    pub db_name: String,
    pub client: Client,
}

impl DBInstanceInUse {
    pub async fn info(&self) -> Result<DBInfo, NanoError> {
        Ok(db_info::info(&self).await?)
    }
    pub async fn insert(
        &self,
        doc_body: &Value,
        id: Option<&str>,
        rev: Option<&str>,
    ) -> Result<DBDocSuccess, NanoError> {
        Ok(db_doc::insert(&self, doc_body, id, rev).await?)
    }
    pub async fn destroy(&self, id: &str, rev: &str) -> Result<DBDocSuccess, NanoError> {
        Ok(db_doc::destroy(&self, id, rev).await?)
    }
    pub async fn get(&self, id: &str, revs_info: bool) -> Result<Value, NanoError> {
        Ok(db_doc::get(&self, id, revs_info).await?)
    }
    pub async fn list(&self, revs_info: bool) -> Result<DBDocList, NanoError> {
        Ok(db_doc::list(&self, revs_info).await?)
    }
    pub async fn bulk(&self, docs: &Value) -> Result<Value, NanoError> {
        Ok(db_doc::bulk(&self, docs).await?)
    }
    pub async fn find(&self, mango_query_obj: &Value) -> Result<DBFindList, NanoError> {
        Ok(db_doc::find(&self, mango_query_obj).await?)
    }
    pub async fn changes(&self, doc_ids: Option<Vec<&str>>) -> Result<DBChanges, NanoError> {
        Ok(db_changes::changes(&self, doc_ids).await?)
    }
}
