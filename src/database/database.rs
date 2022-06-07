use super::types::{
    BulkDocs, BulkDocsResponse, ChangesQueryParams, ChangesResponse, DBInUse, DBInfo, DocResponse,
    FindResponse, GetDocRequestParams, GetDocsRequestParams, GetMultipleDocs,
};
use crate::error::{CouchDBError, NanoError};
use crate::ParseQueryParams;

use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

impl DBInUse {
    /// Get database information
    pub async fn info(&self) -> Result<DBInfo, NanoError> {
        let url = format!("{}/{}", self.url, self.db_name);
        let response = self.client.get(url.as_str()).send().await?;
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
    /// Creates/Updates a new named document or creates a new revision of the existing document in the specified database, using the supplied JSON document structure.
    ///
    /// ## Creating a new Document
    ///
    /// - If the JSON structure includes the `id` and NOT the `rev` param, then the document will be created with the specified document ID.
    /// - If the `id` and `rev`field are NOT specified, a new unique ID will be generated ussing uuid V4 creating a new document
    ///
    /// ## Updating a Document
    ///
    /// When updating an existing document, the current document revision must be included in the document
    /// - The `id` and `rev` params MUST be included
    pub async fn create_or_update_doc(
        &self,
        doc_body: &Value,
        id: Option<&str>,
        rev: Option<&str>,
    ) -> Result<DocResponse, NanoError> {
        let (id, rev) = (id, rev);
        let formated_url = match (id, rev) {
            (Some(id), Some(rev)) => format!("{}/{}/{}?rev={}", self.url, self.db_name, id, rev),
            (Some(id), None) => format!("{}/{}/{}", self.url, self.db_name, id),
            (None, None) | (None, Some(_)) => format!(
                "{}/{}/{}",
                self.url,
                self.db_name,
                Uuid::new_v4().to_string()
            ),
        };

        let response = self
            .client
            .put(&formated_url)
            .json(&doc_body)
            .send()
            .await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: DocResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// Marks the specified document as deleted by adding a field `_deleted` with the value true.
    ///  
    /// Documents with this field will not be returned within requests anymore, but stay in the database.
    /// You must supply the `id` and  the current (latest) revision, by using the `rev` parameter
    pub async fn delete_doc<A, B>(&self, id: A, rev: B) -> Result<DocResponse, NanoError>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let formated_url = format!(
            "{}/{}/{}?rev={}",
            self.url,
            self.db_name,
            id.as_ref(),
            rev.as_ref()
        );

        let response = self.client.delete(&formated_url).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: DocResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// Returns one document by the specified docid from the specified db.
    ///
    /// Unless you request a specific revision, the latest revision of the document will always be returned.
    pub async fn get_doc<S>(
        &self,
        id: S,
        params: Option<GetDocRequestParams>,
    ) -> Result<Value, NanoError>
    where
        S: AsRef<str>,
    {
        let formated_url = format!(
            "{}/{}/{}?{}",
            self.url,
            self.db_name,
            id.as_ref(),
            params
                .unwrap_or(GetDocRequestParams::default())
                .parse_params()
        );

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

    /// List documents stored on database using `_all_docs` view.
    pub async fn list_docs(
        &self,
        params: Option<GetDocsRequestParams>,
    ) -> Result<GetMultipleDocs, NanoError> {
        let formated_url = format!("{}/{}/_all_docs", self.url, self.db_name);
        let response = match self
            .client
            .post(&formated_url)
            .json(&params.unwrap_or(GetDocsRequestParams::default().include_docs(true)))
            .send()
            .await
        {
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
                let body: GetMultipleDocs = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// The bulk document API allows you to create and update multiple documents at the same time within a single request.
    /// The basic operation is similar to creating or updating a single document, except that you batch the document structure and information.
    ///
    /// When creating new documents the document ID (`_id`) is optional.
    /// For updating existing documents, you must provide the document ID, revision information (`_rev`), and new document values.
    ///
    /// In case of batch deleting documents all fields as document ID, revision information and deletion status (`_deleted`) are required.
    pub async fn bulk_docs<T>(&self, docs: BulkDocs<T>) -> Result<BulkDocsResponse, NanoError>
    where
        T: Serialize,
    {
        let formated_url = format!("{}/{}/_bulk_docs", self.url, self.db_name);

        let response = match self.client.post(&formated_url).json(&docs).send().await {
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
                let body: BulkDocsResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => Err(NanoError::GenericCouchdbError(body)),
        }
    }

    /// Find documents using a declarative JSON querying syntax.
    /// ## Example of a query obj
    /// ```
    /// {
    ///    "selector": {
    ///        "year": {"$gt": 2010}
    ///    },
    ///    "fields": ["_id", "_rev", "year", "title"],
    ///    "sort": [{"year": "asc"}],
    ///    "limit": 2,
    ///    "skip": 0,
    ///    "execution_stats": true
    /// }
    /// ```
    /// For a fast declaration `serde_json::json!()` macro could be used to create the query obj:
    /// ```
    /// let mango_query_obj = serde_json::json!({
    ///    "selector": {
    ///        "year": {"$gt": 2010}
    ///    },
    ///    "fields": ["_id", "_rev", "year", "title"],
    ///    "sort": [{"year": "asc"}],
    ///    "limit": 2,
    ///    "skip": 0,
    ///    "execution_stats": true
    /// })
    /// ```
    /// Or using [MangoQuery](crate::database::types::MangoQuery) type
    ///
    pub async fn find<T>(&self, mango_query_obj: T) -> Result<FindResponse, NanoError>
    where
        T: Serialize,
    {
        let formated_url = format!("{}/{}/_find", self.url, self.db_name);

        let response = self
            .client
            .post(&formated_url)
            .json(&mango_query_obj)
            .send()
            .await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        // parse the response body
        let body = response.json::<Value>().await?;
        match status {
            true => {
                let body: FindResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => Err(NanoError::GenericCouchdbError(body)),
        }
    }

    /// Returns a sorted list of changes made to documents in the database, in time order of application, can be obtained from the database’s `_changes` resource.
    ///
    /// Only the most recent change for a given document is guaranteed to be provided, for example if a document has had fields added, and then deleted,
    /// an API client checking for changes will not necessarily receive the intermediate state of added documents.
    ///
    ///This can be used to listen for update and modifications to the database for post processing or synchronization, and for practical purposes, a continuously
    /// connected `_changes` feed is a reasonable approach for generating a real-time log for most applications.
    ///
    /// **NOTE** The results returned by _changes are partially ordered. In other words, the order is not guaranteed to be preserved for multiple calls.
    pub async fn changes(
        &self,
        query_params: Option<ChangesQueryParams>,
    ) -> Result<ChangesResponse, NanoError> {
        let formated_url = format!("{}/{}/_changes?filter=_doc_ids", self.url, self.db_name);
        let response = self
            .client
            .post(&formated_url)
            .json(&query_params.unwrap_or(ChangesQueryParams::default()))
            .send()
            .await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        // parse the response body
        let body = response.json::<Value>().await?;
        match status {
            true => {
                let body: ChangesResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => Err(NanoError::GenericCouchdbError(body)),
        }
    }
}

#[cfg(test)]

mod tests {
    use crate::{
        database::types::{ChangesQueryParams, GetDocRequestParams, GetDocsRequestParams},
        DBInUse, Nano,
    };

    async fn connect_to_node(url: &str) -> DBInUse {
        let nano = Nano::new(url);
        nano.create_and_connect_to_db("my_db", false).await
    }
    #[tokio::test]
    async fn info_test() {
        let my_db = connect_to_node("http://bella:bella@192.168.1.145:5984").await;
        let info = my_db.info().await.unwrap();
        println!("{:#?}", info)
    }
    #[tokio::test]
    async fn create_doc_test() {
        let my_db = connect_to_node("http://bella:bella@192.168.1.145:5984").await;
        let doc = serde_json::json!({ "hello": "it's me" });
        let info = my_db.create_or_update_doc(&doc, None, None).await.unwrap();
        let update_doc = my_db
            .create_or_update_doc(&doc, Some(info.id.as_str()), Some(info.rev.as_str()))
            .await
            .unwrap();
        println!("{:#?}", update_doc)
    }

    #[tokio::test]
    async fn get_doc_test() {
        let my_db = connect_to_node("http://bella:bella@192.168.1.145:5984").await;

        let doc = serde_json::json!({ "hello": "it's me" });
        let response = my_db.create_or_update_doc(&doc, None, None).await.unwrap();

        let info = my_db.get_doc(&response.id, None).await.unwrap();

        println!("{:#?}", &info);

        let info = my_db
            .get_doc(
                &response.id,
                Some(GetDocRequestParams::default().revs(true)),
            )
            .await
            .unwrap();

        println!("{:#?}", &info)
    }
    #[tokio::test]
    async fn get_docs_test() {
        let my_db = connect_to_node("http://bella:bella@192.168.1.145:5984").await;

        let info = my_db
            .list_docs(Some(
                GetDocsRequestParams::default()
                    .include_docs(false)
                    .limit(1)
                    .update_seq(true),
            ))
            .await
            .unwrap();

        println!("{:#?}", &info)
    }
    #[tokio::test]
    async fn changes_test() {
        let my_db = connect_to_node("http://bella:bella@192.168.1.145:5984").await;

        let info = my_db
            .changes(Some(ChangesQueryParams::default().doc_ids(vec![
                "4d53e84e-0dca-4f10-992d-c2caf3eb0f1e".to_string(),
            ])))
            .await
            .unwrap();
        println!("{:#?}", info)
    }
}
