use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard httpbin response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpBinResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// Represents an HTTP request
#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub http_version: String,
}

/// Represents an HTTP response
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    /// Create a new response with status code
    pub fn new(status: u16) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
    
    /// Set JSON body
    pub fn with_json(mut self, value: &impl Serialize) -> Self {
        self.body = serde_json::to_vec(value).unwrap_or_default();
        self.headers.insert(
            "Content-Type".to_string(),
            "application/json".to_string()
        );
        self
    }
    
    /// Set plain text body
    pub fn with_text(mut self, text: &str) -> Self {
        self.body = text.as_bytes().to_vec();
        self.headers.insert(
            "Content-Type".to_string(),
            "text/plain".to_string()
        );
        self
    }
}
