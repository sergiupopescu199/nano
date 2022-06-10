use serde::{Deserialize, Serialize};
use serde_json::Value;

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
#[derive(Debug, Serialize, Deserialize, Default)]
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
    pub fn fields(mut self, values: Vec<&str>) -> Self {
        self.fields = Some(values.iter().map(|s| s.to_string()).collect());
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
