use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level registry of all known Reverb API resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSchema {
    pub base_url: String,
    pub resources: HashMap<String, Resource>,
}

/// A top-level API resource (e.g., "listings", "orders").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub description: Option<String>,
    pub methods: HashMap<String, Method>,
}

/// A single API method on a resource (e.g., "list", "get", "create").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub http_method: HttpMethod,
    /// Path relative to base_url. May contain `{id}` style placeholders.
    pub path: String,
    pub description: Option<String>,
    pub parameters: HashMap<String, Parameter>,
    pub request_body: Option<RequestBody>,
    pub response: Option<ResponseSchema>,
    /// Field name in the response that holds the paginated collection.
    pub page_key: Option<String>,
    /// Query parameter name used to pass the next-page cursor.
    pub cursor_param: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub description: Option<String>,
    pub location: ParameterLocation,
    pub param_type: ParamType,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterLocation {
    Path,
    Query,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
    String,
    Integer,
    Boolean,
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseSchema {
    pub schema: serde_json::Value,
}
