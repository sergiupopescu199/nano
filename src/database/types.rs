use crate::{Convert, ParseQueryParams};
use bevy_reflect::Reflect;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl Convert for DBInfo {}
impl Convert for ChangesResponse {}
impl Convert for GetMultipleDocs {}
impl Convert for DocResponse {}
impl Convert for FindResponse {}
impl Convert for DBOperationSuccess {}

/// DB information
#[derive(Debug, Serialize, Deserialize)]
pub struct DBInfo {
    /// Database name
    pub db_name: String,
    ///  An opaque string that describes the purge state of the database.
    /// Do not rely on this string for counting the number of purge operations.
    pub purge_seq: String,
    /// An opaque string that describes the state of the database.
    /// Do not rely on this string for counting the number of updates.
    pub update_seq: String,
    /// Database Size
    pub sizes: Sizes,
    /// Database properties
    pub props: Props,
    /// Number of deleted documents
    pub doc_del_count: i64,
    /// A count of the documents in the specified database.
    pub doc_count: i64,
    /// The version of the physical format used for the data when it is stored on disk.
    pub disk_format_version: i64,
    /// Set to `true` if the database compaction routine is operating on this database.
    pub compact_running: bool,
    /// Cluster information
    pub cluster: Cluster,
    /// Always "0". (Returned for legacy reasons.)
    pub instance_start_time: String,
}

/// Cluster information
#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    /// Shards. The number of range partitions.
    pub q: i64,
    /// Replicas. The number of copies of every document.
    pub n: i64,
    /// Write quorum. The number of copies of a document that need to be written before a successful reply.
    pub w: i64,
    /// Read quorum. The number of consistent copies of a document that need to be read before a successful reply.
    pub r: i64,
}

/// Database Size
#[derive(Debug, Serialize, Deserialize)]
pub struct Sizes {
    /// The size of the database file on disk in bytes.
    /// Views indexes are not included in the calculation.
    pub file: i64,
    /// The uncompressed size of database contents in bytes.
    pub external: i64,
    /// he size of live data inside the database, in bytes.
    pub active: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Props {
    /// If present and true, this indicates that the database is partitioned.
    pub partitioned: Option<bool>,
}

/// Connected Database
///
/// After creating a database, when connecting to a database from now on this struct will be used to interact with it
pub struct DBInUse {
    /// CouchDB node url
    pub url: String,
    /// Database name
    pub db_name: String,
    /// reqwest client which will be used to perform HTTP requests to CouchDB server
    pub client: Client,
}

/// Returns a sorted list of changes made to documents in the database, in time order of application, can be obtained from the database’s `_changes` resource.
///
/// Only the most recent change for a given document is guaranteed to be provided, for example if a document has had fields added, and then deleted,
/// an API client checking for changes will not necessarily receive the intermediate state of added documents.
///
///This can be used to listen for update and modifications to the database for post processing or synchronization, and for practical purposes,
/// a continuously connected `_changes` feed is a reasonable approach for generating a real-time log for most applications.
///
/// The results returned by `_changes` are partially ordered. In other words, the order is not guaranteed to be preserved for multiple calls.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangesResponse {
    /// A vector of changes made to a database
    pub results: Vec<ChangesDoc>,
    /// Last change update sequence
    pub last_seq: String,
    // Count of remaining items in the feed
    pub pending: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangesDoc {
    /// Update sequence
    pub seq: String,
    ///  Document ID
    pub id: String,
    /// Vector of document’s leaves with single field `rev`
    pub changes: Vec<Changes>,
    /// `true` if the document is deleted.
    pub deleted: Option<bool>,
}
/// Document leaves with single field `rev`
#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {
    /// Revision of the document
    pub rev: String,
}

/// Database response after document creation/deletion or update
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

/// Success creating/deleting a database response from CouchDB
#[derive(Debug, Serialize, Deserialize)]
pub struct DBOperationSuccess {
    /// Operation status
    pub ok: bool,
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
}

impl ParseQueryParams for GetDocRequestParams {}

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
    pub fn rev(mut self, rev: String) -> Self {
        self.rev = rev;
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
    pub fn end_key(mut self, key: String) -> Self {
        self.end_key = Some(key);
        self
    }
    /// Stop returning records when the specified design document ID is reached.
    pub fn end_key_doc_id(mut self, doc_id: String) -> Self {
        self.end_key_doc_id = Some(doc_id);
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
    pub docs: Vec<T>,
    /// If `false`, prevents the database from assigning them new revision IDs. Default is `true`.
    pub new_edits: bool,
}

/// Response of bulk saved documents
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDocsResponse {
    /// Operation status
    pub ok: bool,
    /// Document ID
    pub id: String,
    /// New document revision token. Available if document has saved without errors
    pub rev: String,
    /// Error type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    ///  Error reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
/// Create a Query to CouchDB
///
/// ## Example
/// Lets assume we need to make this query
/// ```
/// {
///     "selector": {
///       "year": {
///         "$eq": 2001
///       }
///     },
///     "sort": [
///       "year"
///     ],
///     "fields": [
///       "year"
///     ]
/// }
/// ```
/// This are the steps to transform in a MangoQuery type:
/// ```
/// // we must use the `json!()` macro because in a struct we cant have keys which starts with `$`
/// let selector = serde_json::json!({
///     "year": {
///         $eq: 2021
///     }
/// })
/// let sort = vec![SortType::String("year".to_string())]
/// let fields = vec!["year".to_string()]
///
/// let mut mango_query = MangoQuery::default();
/// mango_query.selector(selector)
///             .sort(sort)
///             .fields(fields);
///
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct MangoQuery {
    /// Selectors are expressed as a JSON object describing documents of interest. Within this structure, you can apply conditional logic using specially named fields.
    selector: Value,
    /// The `sort` field contains a list of field name and direction pairs, expressed as a basic array.
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<SortType>>,
    /// JSON array specifying which fields of each object should be returned. If it is omitted, the entire object is returned
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<String>>,
    /// Maximum number of results returned. Default is `25`
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    /// Skip the first `n` results, where `n` is the value specified
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<i64>,
    /// Instruct a query to use a specific index.
    #[serde(skip_serializing_if = "Option::is_none")]
    use_index: Option<Vec<String>>,
    /// Include conflicted documents if `true`. Intended use is to easily find conflicted documents, without an index or view. Default is `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    conflicts: Option<bool>,
    /// Read quorum needed for the result. This defaults to 1, in which case the document found in the index is returned.
    ///
    /// If set to a higher value, each document is read from at least that many replicas before it is returned in the results.
    /// This is likely to take more time than using only the document stored locally with the index.
    /// Default `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    r: Option<i64>,
    /// A string that enables you to specify which page of results you require. Used for paging through result sets.
    ///
    /// Every query returns an opaque string under the bookmark key that can then be passed back in a query to get the next page of results.
    ///  If any part of the selector query changes between requests, the results are undefined, Default `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    bookmark: Option<String>,
    /// Whether to update the index prior to returning the result. Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<bool>,
    /// Whether or not the view results should be returned from a `stable` set of shards
    #[serde(skip_serializing_if = "Option::is_none")]
    stable: Option<bool>,
    /// Include execution statistics in the query response, Default `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_stats: Option<bool>,
}

impl Default for MangoQuery {
    fn default() -> Self {
        Self {
            selector: serde_json::Value::default(),
            sort: Option::default(),
            fields: Option::default(),
            limit: Option::default(),
            skip: Option::default(),
            use_index: Option::default(),
            conflicts: Option::default(),
            r: Option::default(),
            bookmark: Option::default(),
            update: Option::default(),
            stable: Option::default(),
            execution_stats: Option::default(),
        }
    }
}

impl MangoQuery {
    pub fn new() -> Self {
        Self::default()
    }
    /// Selectors are expressed as a JSON object describing documents of interest. Within this structure, you can apply conditional logic using specially named fields.
    ///
    /// Whilst selectors have some similarities with MongoDB query documents, these arise from a similarity of purpose and do not necessarily extend to commonality of function or result.
    /// Elementary selector syntax requires you to specify one or more fields, and the corresponding values required for those fields.
    ///
    /// ## Example simle selector
    /// This selector matches all documents whose “director” field has the value “Lars von Trier”.
    /// ```
    /// {
    ///     "selector": {
    ///         "title": "Live And Let Die"
    ///     },
    ///      "fields": [
    ///      "title",
    ///      "cast"
    ///      ]
    /// }
    /// ```
    /// You can create more complex selector expressions by combining operators.
    /// For best performance, it is best to combine `combination` or `array logical` operators, such as `$regex`,
    /// with an equality operators such as `$eq`, `$gt`, `$gte`, `$lt`, and `$lte` (but not `$ne`).
    ///
    /// A more complex selector enables you to specify the values for field of nested objects, or subfields.
    /// For example, you might use a standard JSON structure for specifying a field and subfield.
    /// ## Example of a field and subfield selector, using a standard JSON structure:
    /// ```
    /// {
    ///     "selector" {
    ///         "imdb": {
    ///             "rating": 8
    ///         }
    ///     }
    /// }
    /// ```
    /// An abbreviated equivalent uses a dot notation to combine the field and subfield names into a single name.
    /// ```
    /// {
    ///     "selector" {
    ///         "imdb.rating": 8
    ///     }
    /// }
    /// ```
    /// An example of the `$eq` operator used with database indexed on the field "year"
    /// ```
    /// {
    ///     "selector": {
    ///       "year": {
    ///         "$eq": 2001
    ///       }
    ///     },
    ///     "sort": [
    ///       "year"
    ///     ],
    ///     "fields": [
    ///       "year"
    ///     ]
    /// }
    /// ```
    /// Example of using explicit `$and` and `$eq` operators
    /// ```
    /// {
    ///     "selector": {
    ///         "$and": [
    ///            {
    ///                "director": {
    ///                    "$eq": "Lars von Trier"
    ///                }
    ///            },
    ///            {
    ///                "year": {
    ///                    "$eq": 2003
    ///                }
    ///            }
    ///         ]   
    ///     }
    /// }
    /// ```
    /// for more info about `_find` and its `selector` queries: https://docs.couchdb.org/en/stable/api/database/find.html#db-find
    pub fn selector(mut self, selector: Value) -> Self {
        self.selector = selector;
        self
    }

    /// The `sort` field contains a list of field name and direction pairs, expressed as a basic array.
    ///
    /// The first field name and direction pair is the topmost level of sort.
    ///
    /// The second pair, if provided, is the next level of sort.
    /// The field can be any field, using dotted notation if desired for sub-document fields.
    ///
    /// The direction value is `asc` for ascending, and `desc` for descending. If you omit the direction value, the default `asc` is used.
    /// ## Example sorting by 2 fields
    /// ```
    /// [{"fieldName1": "desc"}, {"fieldName2": "desc" }]
    /// ```
    /// ## Example sorting by 2 fields,assuming `default` direction for both
    /// ```
    /// ["fieldName1", "fieldName2"]
    /// ```
    /// A typical requirement is to search for some content using a selector, then to sort the results according to the specified field, in the required direction.
    ///
    /// To use sorting, ensure that:
    /// - At least one of the sort fields is included in the selector.
    /// - There is an index already defined, with all the sort fields in the same order
    /// - Each object in the sort array has a single key.
    ///
    /// If an object in the sort array does not have a single key, the resulting sort order is implementation specific and might change.
    ///
    /// Find does not support multiple fields with different sort orders, so the directions must be either all ascending or all descending.
    /// ## Example of a simple query using sorting:
    /// ```
    /// {
    ///     "selector": {"Actor_name": "Robert De Niro"},
    ///     "sort": [{"Actor_name": "asc"}, {"Movie_runtime": "asc"}]
    /// }
    /// ```
    pub fn sort(mut self, values: Vec<SortType>) -> Self {
        self.sort = Some(values);
        self
    }
    /// JSON array specifying which fields of each object should be returned. If it is omitted, the entire object is returned
    ///
    /// It is possible to specify exactly which fields are returned for a document when selecting from a database. The two advantages are:
    /// - Your results are limited to only those parts of the document that are required for your application.
    /// - A reduction in the size of the response.
    ///
    /// The fields returned are specified as an array.
    ///
    /// Only the specified filter fields are included, in the response. There is no automatic inclusion of the `_id` or other metadata fields when a field list is included.
    /// ## Example of selective retrieval of fields from matching documents:
    /// ```
    /// {
    ///     "selector": { "Actor_name": "Robert De Niro" },
    ///     "fields": ["Actor_name", "Movie_year", "_id", "_rev"]
    /// }
    /// ```
    pub fn fields(mut self, values: Vec<String>) -> Self {
        self.fields = Some(values);
        self
    }
    /// Maximum number of results returned. Default is `25`
    pub fn limit(mut self, max_docs: i64) -> Self {
        self.limit = Some(max_docs);
        self
    }
    /// Skip the first `n` results, where `n` is the value specified
    pub fn skip(mut self, docs_to_skip: i64) -> Self {
        self.skip = Some(docs_to_skip);
        self
    }
    /// Instruct a query to use a specific index.
    pub fn use_index(mut self, index_to_use: Vec<String>) -> Self {
        self.use_index = Some(index_to_use);
        self
    }
    /// Include conflicted documents if `true`. Intended use is to easily find conflicted documents, without an index or view. Default is `false`
    pub fn conflicts(mut self, enable: bool) -> Self {
        self.conflicts = Some(enable);
        self
    }
    /// Read quorum needed for the result. This defaults to 1, in which case the document found in the index is returned.
    ///
    /// If set to a higher value, each document is read from at least that many replicas before it is returned in the results.
    /// This is likely to take more time than using only the document stored locally with the index.
    /// Default `1`.
    pub fn r(mut self, quorum_num: i64) -> Self {
        self.r = Some(quorum_num);
        self
    }
    /// A string that enables you to specify which page of results you require. Used for paging through result sets.
    ///
    /// Every query returns an opaque string under the bookmark key that can then be passed back in a query to get the next page of results.
    ///  If any part of the selector query changes between requests, the results are undefined, Default `null`
    pub fn bookmark(mut self, value: String) -> Self {
        self.bookmark = Some(value);
        self
    }
    /// Whether to update the index prior to returning the result. Default is `true`.
    pub fn update(mut self, enable: bool) -> Self {
        self.update = Some(enable);
        self
    }
    /// Whether or not the view results should be returned from a `stable` set of shards
    pub fn stable(mut self, enable: bool) -> Self {
        self.stable = Some(enable);
        self
    }
    /// Include execution statistics in the query response, Default `false`
    pub fn execution_stats(mut self, enable: bool) -> Self {
        self.execution_stats = Some(enable);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Sorting can accept an array of strings or json
pub enum SortType {
    String(String),
    Json(Value),
}

impl Default for SortType {
    fn default() -> Self {
        Self::String(String::default())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangesQueryParams {
    /// Vector of document IDs to filter the changes feed
    doc_ids: Vec<String>,
    /// Includes conflicts information in response. Ignored if isn’t `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    conflicts: Option<bool>,
    ///  Return the change results in descending sequence order (most recent change first). Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    descending: Option<bool>,
    /// `normal` Specifies Normal Polling Mode. All past changes are returned immediately. Default.
    #[serde(skip_serializing_if = "Option::is_none")]
    feed: Option<Feed>,
    /// Reference to a filter function from a design document that will filter whole stream emitting only filtered events.
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    /// Period in milliseconds after which an empty line is sent in the results.
    ///
    /// Only applicable for `longpoll`, `continuous`, and `eventsource` feeds. Overrides any timeout to keep the feed alive indefinitely.
    ///
    /// Default is `60000`
    #[serde(skip_serializing_if = "Option::is_none")]
    heartbeat: Option<i64>,
    /// Include the associated document with each result. If there are conflicts, only the winning revision is returned. Default is `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    include_docs: Option<bool>,
    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<bool>,
    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed. \
    ///
    /// Ignored if `include_docs` isn’t `true`. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    att_encoding_info: Option<bool>,
    ///  Alias of `Last-Event-ID` header.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last-event-id")]
    last_event_id: Option<i64>,
    /// Limit number of result rows to the specified value (note that using 0 here has the same effect as 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    /// Specifies how many revisions are returned in the changes array. The default, `main_only`, will only return the current “winning” revision;
    ///
    /// `all_docs` will return all leaf revisions (including conflicts and deleted former conflicts).
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
    ///  Maximum period in milliseconds to wait for a change before the response is sent, even if there are no results.
    ///
    /// Only applicable for `longpoll` or `continuous` feeds. Default value is specified by `chttpd/changes_timeout` configuration option.
    ///
    ///  Note that `60000` value is also the default maximum timeout to prevent undetected dead connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<i64>,
    /// Allows to use view functions as filters. Documents counted as “passed” for view filter in case if map function emits at least one record for them.
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<String>,
    /// When fetching changes in a batch, setting the seq_interval parameter tells CouchDB to only calculate the update seq with every Nth result returned.
    ///
    /// By setting `seq_interval=<batch size>` , where `<batch size>` is the number of results requested per batch, load can be reduced on the source CouchDB database;
    /// computing the seq value across many shards (esp. in highly-sharded databases) is expensive in a heavily loaded CouchDB cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    seq_interval: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Feed {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "longpoll")]
    LongPoll,
    #[serde(rename = "continuous")]
    Continuous,
    #[serde(rename = "eventsource")]
    EventSource,
}

impl Default for Feed {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "main_only")]
    MainOnly,
    #[serde(rename = "all_docs")]
    AllDocs,
}
impl Default for Style {
    fn default() -> Self {
        Self::MainOnly
    }
}

impl Default for ChangesQueryParams {
    fn default() -> Self {
        Self {
            doc_ids: vec![],
            att_encoding_info: Option::default(),
            attachments: Option::default(),
            conflicts: Option::default(),
            descending: Option::default(),
            feed: Option::default(),
            filter: Option::default(),
            heartbeat: Option::default(),
            include_docs: Option::default(),
            last_event_id: Option::default(),
            limit: Option::default(),
            seq_interval: Option::default(),
            style: Option::default(),
            timeout: Option::default(),
            view: Option::default(),
        }
    }
}

impl ChangesQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Vector of document IDs to filter the changes feed
    pub fn doc_ids(mut self, doc_ids: Vec<String>) -> Self {
        self.doc_ids = doc_ids;
        self
    }

    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed. \
    ///
    /// Ignored if `include_docs` isn’t `true`. Default is `false`.
    pub fn att_encoding_info(mut self, enable: bool) -> Self {
        self.att_encoding_info = Some(enable);
        self
    }

    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`. Default is `false`.
    pub fn attachments(mut self, enable: bool) -> Self {
        self.attachments = Some(enable);
        self
    }

    /// Includes conflicts information in response. Ignored if isn’t `true`
    pub fn conflicts(mut self, enable: bool) -> Self {
        self.conflicts = Some(enable);
        self
    }

    /// `normal` Specifies Normal Polling Mode. All past changes are returned immediately. Default.
    pub fn feed(mut self, feed: Feed) -> Self {
        self.feed = Some(feed);
        self
    }

    /// Reference to a filter function from a design document that will filter whole stream emitting only filtered events.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    /// Period in milliseconds after which an empty line is sent in the results.
    ///
    /// Only applicable for `longpoll`, `continuous`, and `eventsource` feeds. Overrides any timeout to keep the feed alive indefinitely.
    ///
    /// Default is `60000`
    pub fn heartbeat(mut self, value: i64) -> Self {
        self.heartbeat = Some(value);
        self
    }

    /// Include the associated document with each result. If there are conflicts, only the winning revision is returned. Default is `false`
    pub fn include_docs(mut self, enable: bool) -> Self {
        self.include_docs = Some(enable);
        self
    }

    ///  Alias of `Last-Event-ID` header.
    pub fn last_event_id(mut self, value: i64) -> Self {
        self.last_event_id = Some(value);
        self
    }

    /// Limit number of result rows to the specified value (note that using 0 here has the same effect as 1).
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// When fetching changes in a batch, setting the seq_interval parameter tells CouchDB to only calculate the update seq with every Nth result returned.
    ///
    /// By setting `seq_interval=<batch size>` , where `<batch size>` is the number of results requested per batch, load can be reduced on the source CouchDB database;
    /// computing the seq value across many shards (esp. in highly-sharded databases) is expensive in a heavily loaded CouchDB cluster.
    pub fn seq_interval(mut self, value: i64) -> Self {
        self.seq_interval = Some(value);
        self
    }

    /// Specifies how many revisions are returned in the changes array. The default, `main_only`, will only return the current “winning” revision;
    ///
    /// `all_docs` will return all leaf revisions (including conflicts and deleted former conflicts).
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    ///  Maximum period in milliseconds to wait for a change before the response is sent, even if there are no results.
    ///
    /// Only applicable for `longpoll` or `continuous` feeds. Default value is specified by `chttpd/changes_timeout` configuration option.
    ///
    ///  Note that `60000` value is also the default maximum timeout to prevent undetected dead connections.
    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
        self
    }

    /// Allows to use view functions as filters. Documents counted as “passed” for view filter in case if map function emits at least one record for them.
    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
        self
    }

    ///  Return the change results in descending sequence order (most recent change first). Default is `false`.
    pub fn descending(mut self, enable: bool) -> Self {
        self.descending = Some(enable);
        self
    }
}
