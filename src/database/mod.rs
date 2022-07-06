pub mod types;
use std::borrow::Borrow;
use std::fmt::Debug;

use crate::database::types::ChangesDoc;
use crate::error::{CouchDBError, NanoError};
use crate::ParseQueryParams;
use types::{
    BulkData, BulkDocs, BulkDocsResponse, BulkGetResponse, ChangesQueryData, ChangesQueryParams,
    ChangesQueryParamsStream, ChangesResponse, DBInUse, DBInfo, DBOperationSuccess, DocResponse,
    FindResponse, GetDocRequestParams, GetDocsRequestParams, GetMultipleDocs, Index, IndexResponse,
};

use async_stream::try_stream;
use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use self::types::GetIndexResponse;

impl DBInUse {
    /// Get database information
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// let my_db_info = my_db.info().await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/common.html#get--db)
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
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    /// // create a doc
    /// let doc = serde_json::json!({"hello":"world"});
    /// // create a doc without specifying it's id
    /// let docs = my_db.create_or_update_doc(&doc, None, None).await.unwrap();
    /// // create a doc specifying a custom id
    /// let docs = my_db.create_or_update_doc(&doc, Some("my_id"), None).await.unwrap();
    /// // update a doc using the revision of previously created doc
    /// let docs = my_db.create_or_update_doc(&doc, Some("my_id"), docs.rev).await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/document/common.html#put--db-docid)
    pub async fn create_or_update_doc<T>(
        &self,
        doc_body: T,
        id: Option<&str>,
        rev: Option<&str>,
    ) -> Result<DocResponse, NanoError>
    where
        T: Serialize + Borrow<T>,
    {
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
            .json(doc_body.borrow())
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
    ///
    ///  ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// let docs = my_db.delete_doc("9042619901bb873974b76d206102c006", "6-34af5d6442ffedb5279b31b6d9b02d06").await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/document/common.html#delete--db-docid)
    pub async fn delete_doc<A, B>(&self, id: A, rev: B) -> Result<DocResponse, NanoError>
    where
        A: Into<String>,
        B: Into<String>,
    {
        let formated_url = format!(
            "{}/{}/{}?rev={}",
            self.url,
            self.db_name,
            id.into(),
            rev.into()
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
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// // customize the request with some params
    /// let params = GetDocRequestParams::default()
    ///                 //get all document revisions
    ///                 .revs(true)
    ///
    /// let docs = my_db.get_doc("9042619901bb873974b76d206102c006",Some(&params)).await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/document/common.html#get--db-docid)
    pub async fn get_doc<'a, S>(
        &self,
        id: S,
        params: Option<&'a GetDocRequestParams>,
    ) -> Result<Value, NanoError>
    where
        S: Into<String>,
    {
        let formated_url = format!(
            "{}/{}/{}?{}",
            self.url,
            self.db_name,
            id.into(),
            params
                .borrow()
                .unwrap_or(&GetDocRequestParams::default())
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
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// // customize the request with some params
    /// let params = GetDocsRequestParams::default()
    ///                 // include docs body in the response
    ///                 .include_docs(true)
    ///                 // per request get 100 docs at the time
    ///                 .limit(100)
    ///
    /// let docs = my_db.list_docs(Some(&params)).await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/bulk-api.html#)
    pub async fn list_docs<'a, T>(
        &self,
        params: Option<&'a GetDocsRequestParams>,
    ) -> Result<GetMultipleDocs, NanoError> {
        let formated_url = format!("{}/{}/_all_docs", self.url, self.db_name);
        let response = match self
            .client
            .post(&formated_url)
            .json(params.unwrap_or(&GetDocsRequestParams::default().include_docs(true)))
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
    ///
    /// ## Example different docs in a vector
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    /// // having different types of docs in an array just use serde_json::Value
    /// let docs = vec1[serde_json::json!({"hello": "world"}), serde_json::json!({"hello":"world", "name":"John"})];
    ///
    /// let bulk_res = my_db.bulk_docs(&docs).await.unwrap();
    /// // access the vector from the struct
    /// println!("{:#?}", bulk_res.0);
    /// ```
    ///
    /// ## Example same documents in a vector
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Counter {
    ///     num: i32
    /// }
    ///
    /// // if we know that the document type is always the same we could use a Struct
    /// let docs = vec1[Counter{ num: 1 }, Counter{ num: 2 }, Counter{ num: 3 }];
    ///
    /// let bulk_res = my_db.bulk_docs(&docs).await.unwrap();
    /// // access the vector from the struct
    /// println!("{:#?}", bulk_res.0);
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/bulk-api.html#db-bulk-docs)
    pub async fn bulk_docs<T, C>(&self, docs: C) -> Result<BulkDocsResponse, NanoError>
    where
        T: Serialize + Debug,
        C: Borrow<BulkDocs<T>>,
    {
        let formated_url = format!("{}/{}/_bulk_docs", self.url, self.db_name);
        let response = match self
            .client
            .post(&formated_url)
            .json(docs.borrow())
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
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
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
    ///
    /// let find_res = my_db.find(&mango_query_obj).await.unwrap()
    ///
    /// ```
    /// ### Or using [MangoQuery](crate::database::types::MangoQuery) type
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// let mango_query_obj = MangoQuery::default()
    ///                         .selector(serde_json::json!("year": {"$gt": 2010}))
    ///                         .fields(vec!["_id", "_rev", "year", "title"])
    ///                         .sort(vec![SortType::Json(serde_json::json!({"year": "asc"}))])
    ///                         .limit(2)
    ///                         .skip(0)
    ///                         .execution_stats(true);
    ///
    /// let find_res = my_db.find(&mango_query_obj).await.unwrap()
    ///
    /// ```
    ///
    pub async fn find<T>(&self, mango_query_obj: T) -> Result<FindResponse, NanoError>
    where
        T: Serialize + Borrow<T>,
    {
        let formated_url = format!("{}/{}/_find", self.url, self.db_name);

        let response = self
            .client
            .post(&formated_url)
            .json(mango_query_obj.borrow())
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

    /// Keeps a continuous connection receiving data from CouchDB, the default timeout is 60 sec, after which the connection will be
    /// automaticli closed, using `ChangesQueryParamsStream::default().heartbeat(<period in milliseconds>)` will keep the connection alive indefinetly
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// // get changes by doc ids
    /// let doc_ids = ChangesQueryData::DocIds(vec!["9042619901bb873974b76d20610427fb"])
    /// let changes_query_params = ChangesQueryParamsStream::default()
    ///                 // include documents body in the response
    ///                .include_docs(true)
    ///                 // filter the response by document id given by `doc_ids`
    ///                .filter(nano::database::types::Filter::DocIds),
    ///                 // give a max limit of documents given in a response
    ///                 .limit(100);
    /// let changes_by_doc_ids = my_db.changes_stream(Some(&doc_ids), Some(&changes_query_params)).await;
    /// // we must use this macto for iteration
    /// future_utils::pin_mut!(changes_by_doc_ids);
    ///
    /// while let Some(value) = info.next().await {
    ///     println!("got {:#?}", value.unwrap());
    /// }
    /// ```
    pub async fn changes_stream<'a>(
        &'a self,
        data: Option<&'a ChangesQueryData<'a>>,
        query_params: Option<&'a ChangesQueryParamsStream>,
    ) -> impl Stream<Item = Result<ChangesResponse, NanoError>> + 'a {
        try_stream! {
        let query_params = query_params.borrow()
            .unwrap_or(&ChangesQueryParamsStream::default())
            .parse_params();
        let formated_url = format!("{}/{}/_changes?{}", self.url, self.db_name, query_params);

        let mut response = match data.borrow() {
            Some(data) => match data {
                ChangesQueryData::DocIds(doc_ids) => {
                    self.client
                        .post(&formated_url)
                        .json(&serde_json::json!({ "doc_ids": doc_ids }))
                        .send()
                        .await?.bytes_stream()
                }
                ChangesQueryData::Selector(selector) => {
                    self.client
                        .post(&formated_url)
                        .json(&selector)
                        .send()
                        .await?.bytes_stream()
                }
            },
            None => {
                self.client
                    .post(&formated_url)
                    .json(&serde_json::json!({}))
                    .send()
                    .await?.bytes_stream()
            }
        };

        // needs some more work and polish
        while let Some(item) = response.next().await {
            let mut items: Vec<ChangesDoc> = vec![];
            let item = item?;
            if item.len() > 1 {
                let body = String::from_utf8(item.to_vec()).unwrap();
                // if last_seq is present this means the connection is closed
                if !body.contains("last_seq") {
                    for data in body.split_ascii_whitespace().into_iter() {
                        let change: ChangesDoc = serde_json::from_str(data)?;
                        items.push(change)
                    }
                    let result = ChangesResponse {
                        last_seq: None,
                        pending: None,
                        results: Some(items),
                    };
                    // return data to the stream
                    yield result;
                } else {
                    let result: ChangesResponse = serde_json::from_str(&body).unwrap();
                    // return data to the stream
                    yield result;
                }
            }
            }
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
    /// ### NOTE
    /// The results returned by `_changes` are partially ordered. In other words, the order is not guaranteed to be preserved for multiple calls.
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// // get all changes from db
    /// let all_changes = my_db.changes(None, None).await.unwrap()
    ///
    /// // get changes by doc ids
    /// let doc_ids = ChangesQueryData::DocIds(vec!["9042619901bb873974b76d20610427fb"])
    /// let changes_query_params = ChangesQueryParams::default()
    ///                 // include documents body in the response
    ///                .include_docs(true)
    ///                 // filter the response by document id given by `doc_ids`
    ///                .filter(nano::database::types::Filter::DocIds),
    ///                 // give a max limit of documents given in a response
    ///                 .limit(100);
    /// let changes_by_doc_ids = my_db.changes(Some(&doc_ids), Some(&changes_query_params)).await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/changes.html)
    pub async fn changes<'a>(
        &self,
        data: Option<&'a ChangesQueryData<'a>>,
        query_params: Option<&'a ChangesQueryParams>,
    ) -> Result<ChangesResponse, NanoError> {
        let query_params = query_params
            .unwrap_or(&ChangesQueryParams::default())
            .parse_params();
        let formated_url = format!("{}/{}/_changes?{}", self.url, self.db_name, query_params);
        println!("{}", formated_url);

        let response = match data {
            Some(data) => match data {
                ChangesQueryData::DocIds(doc_ids) => {
                    self.client
                        .post(&formated_url)
                        .json(&serde_json::json!({ "doc_ids": doc_ids }))
                        .send()
                        .await?
                }
                ChangesQueryData::Selector(selector) => {
                    self.client
                        .post(&formated_url)
                        .json(selector)
                        .send()
                        .await?
                }
            },
            None => {
                self.client
                    .post(&formated_url)
                    .json(&serde_json::json!({}))
                    .send()
                    .await?
            }
        };

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

    /// JSON object describing the index to create.
    ///
    /// ### Index as json obj
    /// ```
    /// {
    ///    "index": {
    ///        "fields": ["foo"]
    ///    },
    ///    "name" : "foo-index",
    ///    "type" : "json"
    /// }
    /// ```
    /// ### Index as Rust types
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// let index = Index::default()
    ///             .add_index(IndexData::default().fields(vec!["foo"]))
    ///             .name("foo-index")
    ///             .index_type(IndexType::Json);
    ///
    /// let index_res =  my_db.create_index(&index).await.unwrap()
    /// ```
    ///
    /// More info about [index](https://docs.couchdb.org/en/stable/api/database/find.html#db-index)
    pub async fn create_index<T>(&self, index: T) -> Result<IndexResponse, NanoError>
    where
        T: Borrow<Index>,
    {
        let formated_url = format!("{}/{}/_index", self.url, self.db_name);
        let response = match self
            .client
            .post(&formated_url)
            .json(index.borrow())
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
                let body: IndexResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// Get all indexes present in db
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// let index_res =  my_db.get_index().await.unwrap()
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/find.html#get--db-_index)
    pub async fn get_index(&self) -> Result<GetIndexResponse, NanoError> {
        let url = format!("{}/{}/_index", self.url, self.db_name);
        let response = self.client.get(url.as_str()).send().await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: GetIndexResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// Delete and index in the db
    ///
    /// ## Example
    /// We have an index like this, in order to delete it we must use `ddoc` and `name`
    /// ```
    /// {
    ///    "ddoc": "_design/a5f4711fc9448864a13c81dc71e660b524d7410c",
    ///    "name": "foo-index",
    ///    "type": "json",
    ///    "def": {
    ///        "fields": [
    ///            {
    ///                "foo": "asc"
    ///            }
    ///        ]
    ///    }
    /// }
    ///
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// let delete_index_res = my_db.delete_index("_design/a5f4711fc9448864a13c81dc71e660b524d7410c", "foo-index").await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/find.html#delete--db-_index-designdoc-json-name)
    pub async fn delete_index<'a, A, B>(
        &self,
        ddoc: A,
        index_name: B,
    ) -> Result<DBOperationSuccess, NanoError>
    where
        A: Into<String>,
        B: Into<String>,
    {
        let url = format!(
            "{}/{}/_index/{}/json/{}",
            self.url,
            self.db_name,
            ddoc.into(),
            index_name.into()
        );
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

    /// This method can be called to query several documents in bulk.
    /// It is well suited for fetching a specific revision of documents, as replicators do for example, or for getting revision history.
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// // using serde_json::json!() method
    /// let data = serde_json::json({
    ///     docs: [
    ///         {
    ///             "id": "9042619901bb873974b76d206102e907"
    ///         },
    ///         {
    ///             "id": "123"
    ///             "rev":"1-4a7e4ae49c4366eaed8edeaea8f784ad"
    ///         }
    ///     ]
    /// })
    ///
    /// // or using typed method
    /// let data = BulkData::default().docs(vec![
    ///     BulkDocQuery::new("123"),
    ///     BulkDocQuery::new_with_rev("1234", "1-4a7e4ae49c4366eaed8edeaea8f784ad"),
    /// ]);
    /// let bulk_res = my_db.bulk_get(&data).await.unwrap()
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/bulk-api.html#db-bulk-get)
    pub async fn bulk_get<T, C>(&self, docs: C) -> Result<BulkGetResponse, NanoError>
    where
        T: Serialize,
        C: Borrow<BulkData<T>>,
    {
        let url = format!("{}/{}/_bulk_get", self.url, self.db_name,);
        let response = self
            .client
            .post(url.as_str())
            .json(docs.borrow())
            .send()
            .await?;
        // check the status code if it's in range from 200-299
        let status = response.status().is_success();
        let status_code = response.status().as_u16();
        // parse the response body
        let body = response.json::<Value>().await?;

        match status {
            true => {
                let body: BulkGetResponse = serde_json::from_value(body)?;
                Ok(body)
            }
            false => {
                let body: CouchDBError = serde_json::from_value(body)?;
                Err(NanoError::Unauthorized(body, status_code))
            }
        }
    }

    /// Purge documents from database
    ///
    /// ## Example
    /// ```
    /// let nano = Nano::new("http://dev:dev@localhost:5984");
    /// let my_db nano.create_and_connect_to_db("my_db", false).await;
    ///
    /// // doc ids to be purged
    /// let doc_ids =vec![
    ///        "9042619901bb873974b76d206102e907",
    ///        "9042619901bb873974b76d20610319b6",
    ///  ];
    /// let purged_docs_res = my_db.purge_docs(doc_ids).await.unwrap();
    /// ```
    ///
    /// More [info](https://docs.couchdb.org/en/stable/api/database/misc.html#post--db-_purge)
    pub async fn purge_docs(&self, doc_ids: Vec<&str>) -> Result<Value, NanoError> {
        #[derive(Deserialize)]
        struct Rev {
            rev: String,
            #[allow(dead_code)]
            status: String,
        }

        let mut docs_info = vec![];
        // get doc info from db
        for id in doc_ids.into_iter() {
            docs_info.push((
                id.clone(),
                self.get_doc(
                    id,
                    Some(&GetDocRequestParams::default().meta(true).deleted(true)),
                )
                .await?,
            ));
        }

        let mut doc_revs = vec![];
        // get doc revision
        for (id, info) in docs_info.into_iter() {
            let rev: Vec<Rev> = serde_json::from_value(info["_revs_info"].clone())?;
            doc_revs.push((id, rev))
        }

        let mut json_obj = serde_json::json!({});
        // create the body for documents do be purged
        for (id, rev) in doc_revs {
            json_obj[id] = rev.into_iter().map(|a| a.rev).collect()
        }

        let url = format!("{}/{}/_purge", self.url, self.db_name,);
        // purge documents
        let response = self
            .client
            .post(url.as_str())
            .json(&json_obj)
            .send()
            .await?;
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
}
