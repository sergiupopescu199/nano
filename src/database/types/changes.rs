use std::borrow::Borrow;

use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::MangoQuery;

/// Returns a sorted list of changes made to documents in the database, in time order of application, can be obtained from the database’s `_changes` resource.
///
/// Only the most recent change for a given document is guaranteed to be provided, for example if a document has had fields added, and then deleted,
/// an API client checking for changes will not necessarily receive the intermediate state of added documents.
///
///This can be used to listen for update and modifications to the database for post processing or synchronization, and for practical purposes,
/// a continuously connected `_changes` feed is a reasonable approach for generating a real-time log for most applications.
///
/// The results returned by `_changes` are partially ordered. In other words, the order is not guaranteed to be preserved for multiple calls.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangesResponse {
    /// A vector of changes made to a database
    pub results: Option<Vec<ChangesDoc>>,
    /// Last change update sequence
    pub last_seq: Option<String>,
    // Count of remaining items in the feed
    pub pending: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangesDoc {
    /// Update sequence
    pub seq: String,
    ///  Document ID
    pub id: String,
    /// Vector of document’s leaves with single field `rev`
    pub changes: Vec<Changes>,
    /// `true` if the document is deleted.
    pub deleted: Option<bool>,
    /// include doc body if `include_doc=true` is provided
    pub doc: Option<Value>,
}
/// Document leaves with single field `rev`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Changes {
    /// Revision of the document
    pub rev: String,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone)]
pub struct ChangesQueryParamsStream {
    /// Includes conflicts information in response. Ignored if isn’t `true`
    conflicts: bool,
    ///  Return the change results in descending sequence order (most recent change first). Default is `false`.
    descending: bool,
    /// `normal` Specifies Normal Polling Mode. All past changes are returned immediately. Default.
    feed: String,
    /// Reference to a filter function from a design document that will filter whole stream emitting only filtered events.
    filter: String,
    /// Period in milliseconds after which an empty line is sent in the results.
    ///
    /// Only applicable for `longpoll`, `continuous`, and `eventsource` feeds. Overrides any timeout to keep the feed alive indefinitely.
    ///
    /// Default is `60000`
    heartbeat: i64,
    /// Include the associated document with each result. If there are conflicts, only the winning revision is returned. Default is `false`
    include_docs: bool,
    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`. Default is `false`.
    attachments: bool,
    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed. \
    ///
    /// Ignored if `include_docs` isn’t `true`. Default is `false`.
    att_encoding_info: bool,
    /// Limit number of result rows to the specified value (note that using 0 here has the same effect as 1).
    limit: i64,
    /// Specifies how many revisions are returned in the changes array. The default, `main_only`, will only return the current “winning” revision;
    ///
    /// `all_docs` will return all leaf revisions (including conflicts and deleted former conflicts).
    style: String,
    ///  Maximum period in milliseconds to wait for a change before the response is sent, even if there are no results.
    ///
    /// Only applicable for `longpoll` or `continuous` feeds. Default value is specified by `chttpd/changes_timeout` configuration option.
    ///
    ///  Note that `60000` value is also the default maximum timeout to prevent undetected dead connections.
    timeout: i64,
    /// Allows to use view functions as filters. Documents counted as “passed” for view filter in case if map function emits at least one record for them.
    view: String,
    /// When fetching changes in a batch, setting the seq_interval parameter tells CouchDB to only calculate the update seq with every Nth result returned.
    ///
    /// By setting `seq_interval=<batch size>` , where `<batch size>` is the number of results requested per batch, load can be reduced on the source CouchDB database;
    /// computing the seq value across many shards (esp. in highly-sharded databases) is expensive in a heavily loaded CouchDB cluster.
    seq_interval: i64,
}
#[derive(Debug, Serialize, Deserialize, Reflect, Clone, Default)]
pub struct ChangesQueryParams {
    /// Includes conflicts information in response. Ignored if isn’t `true`
    conflicts: bool,
    ///  Return the change results in descending sequence order (most recent change first). Default is `false`.
    descending: bool,
    /// Reference to a filter function from a design document that will filter whole stream emitting only filtered events.
    filter: String,
    /// Include the associated document with each result. If there are conflicts, only the winning revision is returned. Default is `false`
    include_docs: bool,
    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`. Default is `false`.
    attachments: bool,
    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed. \
    ///
    /// Ignored if `include_docs` isn’t `true`. Default is `false`.
    att_encoding_info: bool,
    /// Limit number of result rows to the specified value (note that using 0 here has the same effect as 1).
    limit: i64,
    /// Specifies how many revisions are returned in the changes array. The default, `main_only`, will only return the current “winning” revision;
    ///
    /// `all_docs` will return all leaf revisions (including conflicts and deleted former conflicts).
    style: String,
    /// Allows to use view functions as filters. Documents counted as “passed” for view filter in case if map function emits at least one record for them.
    view: String,
    /// When fetching changes in a batch, setting the seq_interval parameter tells CouchDB to only calculate the update seq with every Nth result returned.
    ///
    /// By setting `seq_interval=<batch size>` , where `<batch size>` is the number of results requested per batch, load can be reduced on the source CouchDB database;
    /// computing the seq value across many shards (esp. in highly-sharded databases) is expensive in a heavily loaded CouchDB cluster.
    seq_interval: i64,
}

/// Feed options
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Feed {
    /// Equal to a regualr Request/Response
    Normal,
    LongPoll,
    /// A continuous connection in a Stream fashion between CouchDB and the client
    Continuous,
    EventSource,
}

impl std::fmt::Display for Feed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Feed::Normal => write!(f, "normal"),
            Feed::LongPoll => write!(f, "longpoll"),
            Feed::Continuous => write!(f, "continuous"),
            Feed::EventSource => write!(f, "eventsource"),
        }
    }
}

impl Default for Feed {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Style {
    MainOnly,
    AllDocs,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Style::MainOnly => write!(f, "main_only"),
            Style::AllDocs => write!(f, "all_docs"),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::MainOnly
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Filter {
    /// `filter=_selector`
    ///
    /// This filter accepts only changes for documents which match a specified selector, defined using the same selector syntax used for `_find`.
    Selector,
    /// This filter accepts only changes for documents which ID in specified in doc_ids query parameter or payload’s object array
    DocIds,
    /// The `_design` filter accepts only changes for any design document within the requested database.
    Design,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Filter::Selector => write!(f, "_selector"),
            Filter::DocIds => write!(f, "_doc_ids"),
            Filter::Design => write!(f, "_design"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChangesQueryData<'a> {
    /// Selector json used to make a query, it can accept either `serde_json::json!()` or `MangoQuery` type
    ///
    /// ### Example of a selector example
    /// ```
    /// {
    ///    "selector": { "_id": { "$regex": "^_design/" } }
    /// }
    /// ```
    Selector(MangoQuery),
    /// Vec of doc IDs
    DocIds(Vec<&'a str>),
}

impl Default for ChangesQueryParamsStream {
    fn default() -> Self {
        Self {
            att_encoding_info: bool::default(),
            attachments: bool::default(),
            conflicts: bool::default(),
            descending: bool::default(),
            feed: Feed::Continuous.to_string(),
            filter: String::default(),
            heartbeat: i64::default(),
            include_docs: bool::default(),
            limit: i64::default(),
            seq_interval: i64::default(),
            style: String::default(),
            timeout: i64::default(),
            view: String::default(),
        }
    }
}

impl ChangesQueryParamsStream {
    pub fn new() -> Self {
        Self::default()
    }

    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed. \
    ///
    /// Ignored if `include_docs` isn’t `true`. Default is `false`.
    pub fn att_encoding_info(mut self, enable: bool) -> Self {
        self.att_encoding_info = enable;
        self
    }

    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`. Default is `false`.
    pub fn attachments(mut self, enable: bool) -> Self {
        self.attachments = enable;
        self
    }

    /// Includes conflicts information in response. Ignored if isn’t `true`
    pub fn conflicts(mut self, enable: bool) -> Self {
        self.conflicts = enable;
        self
    }

    /// `normal` Specifies Normal Polling Mode. All past changes are returned immediately. Default.
    pub fn feed<T>(mut self, feed: T) -> Self
    where
        T: Borrow<Feed>,
    {
        self.feed = feed.borrow().to_string();
        self
    }

    /// Reference to a filter function from a design document that will filter whole stream emitting only filtered events.
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Borrow<Filter>,
    {
        self.filter = filter.borrow().to_string();
        self
    }

    /// Period in milliseconds after which an empty line is sent in the results.
    ///
    /// Only applicable for `longpoll`, `continuous`, and `eventsource` feeds. Overrides any timeout to keep the feed alive indefinitely.
    ///
    /// Default is `60000`
    pub fn heartbeat(mut self, value: i64) -> Self {
        self.heartbeat = value;
        self
    }

    /// Include the associated document with each result. If there are conflicts, only the winning revision is returned. Default is `false`
    pub fn include_docs(mut self, enable: bool) -> Self {
        self.include_docs = enable;
        self
    }

    /// Limit number of result rows to the specified value (note that using 0 here has the same effect as 1).
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = value;
        self
    }

    /// When fetching changes in a batch, setting the seq_interval parameter tells CouchDB to only calculate the update seq with every Nth result returned.
    ///
    /// By setting `seq_interval=<batch size>` , where `<batch size>` is the number of results requested per batch, load can be reduced on the source CouchDB database;
    /// computing the seq value across many shards (esp. in highly-sharded databases) is expensive in a heavily loaded CouchDB cluster.
    pub fn seq_interval(mut self, value: i64) -> Self {
        self.seq_interval = value;
        self
    }

    /// Specifies how many revisions are returned in the changes array. The default, `main_only`, will only return the current “winning” revision;
    ///
    /// `all_docs` will return all leaf revisions (including conflicts and deleted former conflicts).
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Borrow<Style>,
    {
        self.style = style.borrow().to_string();
        self
    }

    ///  Maximum period in milliseconds to wait for a change before the response is sent, even if there are no results.
    ///
    /// Only applicable for `longpoll` or `continuous` feeds. Default value is specified by `chttpd/changes_timeout` configuration option.
    ///
    ///  Note that `60000` value is also the default maximum timeout to prevent undetected dead connections.
    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = value;
        self
    }

    /// Allows to use view functions as filters. Documents counted as “passed” for view filter in case if map function emits at least one record for them.
    pub fn view<A>(mut self, value: A) -> Self
    where
        A: Into<String>,
    {
        self.view = value.into();
        self
    }

    ///  Return the change results in descending sequence order (most recent change first). Default is `false`.
    pub fn descending(mut self, enable: bool) -> Self {
        self.descending = enable;
        self
    }
}

impl ChangesQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Include encoding information in attachment stubs if `include_docs` is `true` and the particular attachment is compressed. \
    ///
    /// Ignored if `include_docs` isn’t `true`. Default is `false`.
    pub fn att_encoding_info(mut self, enable: bool) -> Self {
        self.att_encoding_info = enable;
        self
    }

    /// Include the Base64-encoded content of attachments in the documents that are included if `include_docs` is `true`.
    ///
    ///  Ignored if `include_docs` isn’t `true`. Default is `false`.
    pub fn attachments(mut self, enable: bool) -> Self {
        self.attachments = enable;
        self
    }

    /// Includes conflicts information in response. Ignored if isn’t `true`
    pub fn conflicts(mut self, enable: bool) -> Self {
        self.conflicts = enable;
        self
    }

    /// Reference to a filter function from a design document that will filter whole stream emitting only filtered events.
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Borrow<Filter>,
    {
        self.filter = filter.borrow().to_string();
        self
    }

    /// Include the associated document with each result. If there are conflicts, only the winning revision is returned. Default is `false`
    pub fn include_docs(mut self, enable: bool) -> Self {
        self.include_docs = enable;
        self
    }

    /// Limit number of result rows to the specified value (note that using 0 here has the same effect as 1).
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = value;
        self
    }

    /// When fetching changes in a batch, setting the seq_interval parameter tells CouchDB to only calculate the update seq with every Nth result returned.
    ///
    /// By setting `seq_interval=<batch size>` , where `<batch size>` is the number of results requested per batch, load can be reduced on the source CouchDB database;
    /// computing the seq value across many shards (esp. in highly-sharded databases) is expensive in a heavily loaded CouchDB cluster.
    pub fn seq_interval(mut self, value: i64) -> Self {
        self.seq_interval = value;
        self
    }

    /// Specifies how many revisions are returned in the changes array. The default, `main_only`, will only return the current “winning” revision;
    ///
    /// `all_docs` will return all leaf revisions (including conflicts and deleted former conflicts).
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Borrow<Style>,
    {
        self.style = style.borrow().to_string();
        self
    }

    /// Allows to use view functions as filters. Documents counted as “passed” for view filter in case if map function emits at least one record for them.
    pub fn view<A>(mut self, value: A) -> Self
    where
        A: Into<String>,
    {
        self.view = value.into();
        self
    }

    ///  Return the change results in descending sequence order (most recent change first). Default is `false`.
    pub fn descending(mut self, enable: bool) -> Self {
        self.descending = enable;
        self
    }
}
