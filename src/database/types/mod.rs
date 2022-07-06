use crate::{Convert, ParseQueryParams};
use reqwest::Client;
use serde::{Deserialize, Serialize};

mod changes;
mod documents;
mod index;
mod query;
pub use changes::*;
pub use documents::*;
pub use index::*;
pub use query::*;

impl Convert for DBInfo {}
impl Convert for ChangesResponse {}
impl Convert for GetMultipleDocs {}
impl Convert for DocResponse {}
impl Convert for FindResponse {}
impl Convert for DBOperationSuccess {}

impl ParseQueryParams for ChangesQueryParamsStream {}
impl ParseQueryParams for ChangesQueryParams {}
impl ParseQueryParams for GetDocRequestParams {}

/// DB information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DBInfo {
    /// Database name
    pub db_name: String,
    /// An opaque string that describes the purge state of the database.
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sizes {
    /// The size of the database file on disk in bytes.
    /// Views indexes are not included in the calculation.
    pub file: i64,
    /// The uncompressed size of database contents in bytes.
    pub external: i64,
    /// he size of live data inside the database, in bytes.
    pub active: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Props {
    /// If present and true, this indicates that the database is partitioned.
    pub partitioned: Option<bool>,
}

/// Connected Database
///
/// After creating a database, when connecting to a database from now on this struct will be used to interact with it
#[derive(Debug, Clone)]
pub struct DBInUse {
    /// CouchDB node url
    pub url: String,
    /// Database name
    pub db_name: String,
    /// reqwest client which will be used to perform HTTP requests to CouchDB server
    pub client: Client,
}

/// Success creating/deleting a database response from CouchDB
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DBOperationSuccess {
    /// Operation status
    pub ok: bool,
}
