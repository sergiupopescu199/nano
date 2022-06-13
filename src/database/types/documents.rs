use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Database response after document creation/deletion or update
#[derive(Debug, Serialize, Deserialize)]
pub struct DocResponse {
    /// Operation status
    pub ok: bool,
    /// Document ID
    pub id: String,
    /// Revision MVCC token
    pub rev: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMultipleDocs {
    /// Number of documents in the database
    pub total_rows: i64,
    /// Offset where the design document list started
    pub offset: i64,
    /// Vector of documents stored
    pub rows: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FindResponse {
    /// Vector of documents matching the search. In each matching document, the fields specified in the fields part of the request body are listed, along with their values.
    pub docs: Vec<Value>,
    /// A string that enables you to specify which page of results you require. Used for paging through result sets.
    ///  Every query returns an opaque string under the bookmark key that can then be passed back in a query to get the next page of results.
    /// If any part of the selector query changes between requests, the results are undefined. Optional, default: null
    pub bookmark: String,
    /// Execution warnings
    pub warning: String,
    /// Execution stats
    pub execution_stats: Option<ExecutionStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub total_keys_examined: i64,
    pub total_docs_examined: i64,
    pub total_quorum_docs_examined: i64,
    pub results_returned: i64,
    pub execution_time_ms: f64,
}

/// Get document request params
#[derive(Reflect, Default, Debug)]
pub struct GetDocRequestParams {
    /// Includes attachments bodies in response
    attachments: bool,
    /// Includes encoding information in attachment stubs if the particular attachment is compressed.
    att_encoding_info: bool,
    /// Includes information about conflicts in document
    conflicts: bool,
    /// Includes information about deleted conflicted revisions
    deleted_conflicts: bool,
    /// Forces retrieving latest `leaf` revision, no matter what rev was requested
    latest: bool,
    /// Includes last update sequence for the document
    local_seq: bool,
    /// Acts same as specifying all `conflicts`, `deleted_conflicts` and `revs_info` query parameters
    meta: bool,
    ///  Retrieves document of specified revision
    rev: String,
    /// Includes list of all known document revisions.
    revs: bool,
    /// Includes detailed information for all known document revisions
    revs_info: bool,
    /// Deleted documents
    deleted: bool,
}

impl GetDocRequestParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Includes attachments bodies in response
    pub fn attachments(mut self, enable: bool) -> Self {
        self.attachments = enable;
        self
    }

    /// Includes encoding information in attachment stubs if the particular attachment is compressed.
    pub fn att_encoding_info(mut self, enable: bool) -> Self {
        self.att_encoding_info = enable;
        self
    }

    /// Includes information about conflicts in document
    pub fn conflicts(mut self, enable: bool) -> Self {
        self.conflicts = enable;
        self
    }

    /// Includes information about deleted conflicted revisions
    pub fn deleted_conflicts(mut self, enable: bool) -> Self {
        self.deleted_conflicts = enable;
        self
    }

    /// Forces retrieving latest `leaf` revision, no matter what rev was requested
    pub fn latest(mut self, enable: bool) -> Self {
        self.latest = enable;
        self
    }

    /// Includes last update sequence for the document
    pub fn local_seq(mut self, enable: bool) -> Self {
        self.local_seq = enable;
        self
    }

    /// Acts same as specifying all `conflicts`, `deleted_conflicts` and `revs_info` query parameters
    pub fn meta(mut self, enable: bool) -> Self {
        self.meta = enable;
        self
    }

    ///  Retrieves document of specified revision
    pub fn rev<A>(mut self, rev: A) -> Self
    where
        A: Into<String>,
    {
        self.rev = rev.into();
        self
    }

    /// Includes list of all known document revisions.
    pub fn revs(mut self, enable: bool) -> Self {
        self.revs = enable;
        self
    }

    /// Includes detailed information for all known document revisions
    pub fn revs_info(mut self, enable: bool) -> Self {
        self.revs_info = enable;
        self
    }

    /// Get doc even if it has been deleted
    pub fn deleted(mut self, enable: bool) -> Self {
        self.deleted = enable;
        self
    }
}

/// Get documents request params
#[derive(Serialize, Deserialize, Debug)]
pub struct GetDocsRequestParams {
    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<bool>,
    /// Includes conflicts information in response. Ignored if isn’t `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    conflicts: Option<bool>,
    ///  Return the design documents in descending by key order
    #[serde(skip_serializing_if = "Option::is_none")]
    descending: Option<bool>,
    /// Stop returning records when the specified key is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    endkey: Option<String>,
    /// Alias for `endkey` param
    #[serde(skip_serializing_if = "Option::is_none")]
    end_key: Option<String>,
    /// Stop returning records when the specified design document ID is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    endkey_docid: Option<String>,
    /// Alias for `endkey_docid` param
    #[serde(skip_serializing_if = "Option::is_none")]
    end_key_doc_id: Option<String>,
    ///  Group the results using the reduce function to a group or single row
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<bool>,
    /// Specify the group level to be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    group_level: Option<i64>,
    /// Include the full content of the design documents in the return
    #[serde(skip_serializing_if = "Option::is_none")]
    include_docs: Option<bool>,
    /// Specifies whether the specified end key should be included in the result
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusive_end: Option<bool>,
    /// Return only design documents that match the specified key.
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<KeyValue>,
    /// Return only design documents that match the specified keys
    #[serde(skip_serializing_if = "Option::is_none")]
    keys: Option<Vec<KeyValue>>,
    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed.
    ///
    /// Ignored if `include_docs` isn’t `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    att_encoding_info: Option<bool>,
    /// Limit the number of the returned documents to the specified number.
    ///
    /// Default are `25` docs
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    /// Use the reduction function. Default is true when a reduce function is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce: Option<bool>,
    /// Skip this number of records before starting to return the results
    ///
    /// Default is `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<i64>,
    ///  Sort returned rows. Setting this to false offers a performance boost.
    ///
    /// The total_rows and offset fields are not available when this is set to false. Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    sorted: Option<bool>,
    /// Whether or not the view results should be returned from a stable set of shards.
    #[serde(skip_serializing_if = "Option::is_none")]
    stable: Option<bool>,
    ///  Whether to include in the response an `update_seq` value indicating the sequence id of the database the view reflects
    #[serde(skip_serializing_if = "Option::is_none")]
    update_seq: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyValue {
    key: String,
}

impl Default for GetDocsRequestParams {
    fn default() -> Self {
        Self {
            attachments: Option::default(),
            att_encoding_info: Option::default(),
            group_level: Option::default(),
            key: Option::default(),
            keys: Option::default(),
            conflicts: Option::default(),
            descending: Option::default(),
            endkey: Option::default(),
            end_key: Option::default(),
            endkey_docid: Option::default(),
            end_key_doc_id: Option::default(),
            include_docs: Option::default(),
            inclusive_end: Some(true),
            group: Option::default(),
            limit: Some(25),
            reduce: Option::default(),
            skip: Some(0),
            sorted: Some(true),
            stable: Option::default(),
            update_seq: Option::default(),
        }
    }
}

impl GetDocsRequestParams {
    pub fn new() -> Self {
        Self::default()
    }
    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`
    pub fn attachments(mut self, enable: bool) -> Self {
        self.attachments = Some(enable);
        self
    }
    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed.
    ///
    /// Ignored if `include_docs` isn’t `true`.
    pub fn att_encoding_info(mut self, enable: bool) -> Self {
        self.att_encoding_info = Some(enable);
        self
    }
    /// Specify the group level to be used.
    pub fn group_level(mut self, group_level: i64) -> Self {
        self.group_level = Some(group_level);
        self
    }
    ///  Group the results using the reduce function to a group or single row
    pub fn group(mut self, enable: bool) -> Self {
        self.group = Some(enable);
        self
    }
    /// Return only design documents that match the specified key.
    pub fn key(mut self, key: KeyValue) -> Self {
        self.key = Some(key);
        self
    }
    /// Return only design documents that match the specified keys.
    pub fn keys(mut self, keys: Vec<KeyValue>) -> Self {
        self.keys = Some(keys);
        self
    }
    /// Includes conflicts information in response. Ignored if isn’t `true`
    pub fn conflicts(mut self, enable: bool) -> Self {
        self.conflicts = Some(enable);
        self
    }
    ///  Return the design documents in descending by key order
    pub fn descending(mut self, enable: bool) -> Self {
        self.descending = Some(enable);
        self
    }
    /// Stop returning records when the specified key is reached
    pub fn end_key<A>(mut self, key: A) -> Self
    where
        A: Into<String>,
    {
        self.end_key = Some(key.into());
        self
    }
    /// Stop returning records when the specified design document ID is reached.
    pub fn end_key_doc_id<A>(mut self, doc_id: A) -> Self
    where
        A: Into<String>,
    {
        self.end_key_doc_id = Some(doc_id.into());
        self
    }
    /// Include the full content of the design documents in the return
    pub fn include_docs(mut self, enable: bool) -> Self {
        self.include_docs = Some(enable);
        self
    }
    /// Specifies whether the specified end key should be included in the result
    pub fn inclusive_end(mut self, enable: bool) -> Self {
        self.inclusive_end = Some(enable);
        self
    }
    /// Limit the number of the returned documents to the specified number.
    ///
    /// Default are `25` docs
    pub fn limit(mut self, max_docs: i64) -> Self {
        self.limit = Some(max_docs);
        self
    }
    /// Skip this number of records before starting to return the results
    ///
    /// Default is `0`
    pub fn skip(mut self, max_docs_skip: i64) -> Self {
        self.skip = Some(max_docs_skip);
        self
    }
    /// Use the reduction function. Default is true when a reduce function is defined.
    pub fn reduce(mut self, enable: bool) -> Self {
        self.reduce = Some(enable);
        self
    }
    /// Whether or not the view results should be returned from a stable set of shards.
    pub fn stable(mut self, enable: bool) -> Self {
        self.stable = Some(enable);
        self
    }
    ///  Whether to include in the response an `update_seq` value indicating the sequence id of the database the view reflects
    pub fn update_seq(mut self, enable: bool) -> Self {
        self.update_seq = Some(enable);
        self
    }
}

/// Save Documents in bulk
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDocs<T>
where
    T: Serialize,
{
    /// List of documents objects
    docs: Vec<T>,
    /// If `false`, prevents the database from assigning them new revision IDs. Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    new_edits: Option<bool>,
}

impl<T> BulkDocs<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }
    /// Vec of documents to be sent to db
    pub fn docs(mut self, docs: Vec<T>) -> Self {
        self.docs = docs;
        self
    }
    /// If `false`, prevents the database from assigning them new revision IDs. Default is `true`.
    pub fn new_edits(mut self, enable: bool) -> Self {
        self.new_edits = Some(enable);
        self
    }
}

impl<T> Default for BulkDocs<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            docs: vec![],
            new_edits: Option::default(),
        }
    }
}
/// Bulk saved documents
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDocsRes {
    /// Operation status
    pub ok: Option<bool>,
    /// Document ID
    pub id: String,
    /// New document revision token. Available if document has saved without errors
    pub rev: Option<String>,
    /// Error type
    pub error: Option<String>,
    ///  Error reason.
    pub reason: Option<String>,
}

/// Response of bulk saved documents
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDocsResponse(pub Vec<BulkDocsRes>);

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkGetResponse {
    pub results: Vec<BulkGetObj>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkGetObj {
    pub id: String,
    pub docs: Vec<BulkResult>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkResult {
    pub ok: Option<Value>,
    pub error: Option<ErrorBulkResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorBulkResponse {
    pub id: String,
    pub rev: String,
    pub error: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkData<T>
where
    T: Serialize,
{
    docs: Vec<T>,
}

impl<T> Default for BulkData<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self { docs: vec![] }
    }
}

impl<T> BulkData<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }
    /// docs to be queried
    pub fn docs(mut self, docs: Vec<T>) -> Self {
        self.docs = docs;
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDocQuery {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
}

impl BulkDocQuery {
    pub fn new<A>(id: A) -> Self
    where
        A: Into<String>,
    {
        Self {
            id: id.into(),
            rev: None,
        }
    }

    pub fn new_with_rev<A, B>(id: A, rev: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            rev: Some(rev.into()),
        }
    }

    /// add revision to the specified document
    pub fn rev<A>(mut self, rev: A) -> Self
    where
        A: Into<String>,
    {
        self.rev = Some(rev.into());
        self
    }
}
