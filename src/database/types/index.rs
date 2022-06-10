use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    /// JSON object describing the index to create.
    index: IndexData,
    /// Name of the design document in which the index will be created. By default, each index will be created in its own design document.
    ///
    /// Indexes can be grouped into design documents for efficiency. However, a change to one index in a design document will invalidate all
    /// other indexes in the same document (similar to views)
    #[serde(skip_serializing_if = "Option::is_none")]
    ddoc: Option<String>,
    /// Name of the index. If no name is provided, a name will be generated automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Can be `json` or `text`. Defaults to `json`.
    #[serde(rename = "type")]
    index_type: String,
    /// Determines whether a JSON index is partitioned or global.
    ///
    /// The default value of partitioned is the partitioned property of the database. To create a global index on a partitioned database,
    /// specify false for the `partitioned` field. If you specify true for the `partitioned` field on an unpartitioned database, an error occurs.
    #[serde(skip_serializing_if = "Option::is_none")]
    partitioned: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IndexType {
    Text,
    Json,
}

impl Default for IndexType {
    fn default() -> Self {
        Self::Json
    }
}

impl std::fmt::Display for IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IndexType::Text => write!(f, "text"),
            IndexType::Json => write!(f, "json"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexData {
    /// A selector to apply to documents at indexing time, creating a partial index.
    #[serde(skip_serializing_if = "Option::is_none")]
    partial_filter_selector: Option<Value>,
    /// Vector of field names following the sort syntax. Nested fields are also allowed, e.g. `person.name`.
    fields: Vec<String>,
}

impl Default for IndexData {
    fn default() -> Self {
        Self {
            partial_filter_selector: Option::default(),
            fields: vec![],
        }
    }
}

impl IndexData {
    pub fn new() -> Self {
        Self::default()
    }

    /// A selector to apply to documents at indexing time, creating a partial index.
    pub fn partial_filter_selector(mut self, value: Value) -> Self {
        self.partial_filter_selector = Some(value);
        self
    }

    /// Vector of field names following the sort syntax. Nested fields are also allowed, e.g. `person.name`.
    pub fn fields(mut self, fields: Vec<&str>) -> Self {
        self.fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }
}

impl Default for Index {
    fn default() -> Self {
        Self {
            index: IndexData::default(),
            ddoc: Option::default(),
            name: Option::default(),
            index_type: IndexType::Json.to_string(),
            partitioned: Option::default(),
        }
    }
}

impl Index {
    pub fn new() -> Self {
        Self::default()
    }
    /// JSON object describing the index to create.
    pub fn add_index(mut self, index: IndexData) -> Self {
        self.index = index;
        self
    }

    /// Name of the design document in which the index will be created. By default, each index will be created in its own design document.
    ///
    /// Indexes can be grouped into design documents for efficiency. However, a change to one index in a design document will invalidate all
    /// other indexes in the same document (similar to views)
    pub fn design_doc_index<A>(mut self, ddoc: A) -> Self
    where
        A: Into<String>,
    {
        self.ddoc = Some(ddoc.into());
        self
    }

    /// Name of the index. If no name is provided, a name will be generated automatically.
    pub fn name<A>(mut self, index_name: A) -> Self
    where
        A: Into<String>,
    {
        self.name = Some(index_name.into());
        self
    }

    /// Can be `json` or `text`. Defaults to `json`.
    pub fn index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type.to_string();
        self
    }

    /// Determines whether a JSON index is partitioned or global.
    ///
    /// The default value of partitioned is the partitioned property of the database. To create a global index on a partitioned database,
    /// specify false for the `partitioned` field. If you specify true for the `partitioned` field on an unpartitioned database, an error occurs.
    pub fn partitioned(mut self, enable: bool) -> Self {
        self.partitioned = Some(enable);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexResponse {
    /// Flag to show whether the index was created or one already exists. Can be `created` or `exists`.
    pub result: String,
    /// Id of the design document the index was created in.
    pub id: String,
    /// Name of the index created
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIndexResponse {
    /// Number of indexes
    pub total_rows: i64,
    ///  Vector of index definitions
    pub indexes: Vec<IndexObj>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexObj {
    /// D of the design document the index belongs to
    pub ddoc: Option<String>,
    /// Name of the index
    pub name: String,
    /// Type of the index. Currently `json` is the only
    #[serde(rename = "type")]
    pub index_type: String,
    /// Definition of the index, containing the indexed fields
    pub def: IndexFields,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexFields {
    /// indexed fields
    fields: Vec<Value>,
}
