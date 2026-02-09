use crate::{Request, Response};
use serde_json::json;

/// Handles /cache endpoint
/// Returns 304 if If-Modified-Since or If-None-Match provided
pub fn cache_handler(req: &Request) -> Response {
    let has_if_modified_since = req.headers.contains_key("If-Modified-Since") 
        || req.headers.contains_key("if-modified-since");
    
    let has_if_none_match = req.headers.contains_key("If-None-Match")
        || req.headers.contains_key("if-none-match");
    
    if has_if_modified_since || has_if_none_match {
        // Return 304 Not Modified
        Response::new(304)
    } else {
        // Return normal response with cache headers
        let response_data = json!({
            "headers": req.headers,
            "url": format!("https://httpbin.org{}", req.path)
        });
        
        let mut response = Response::new(200);
        response = response.with_json(&response_data);
        response.headers.insert("Last-Modified".to_string(), "Mon, 01 Jan 2024 00:00:00 GMT".to_string());
        response.headers.insert("ETag".to_string(), "\"sample-etag\"".to_string());
        response
    }
}

/// Handles /cache/{n} endpoint
/// Sets Cache-Control header for n seconds
pub fn cache_n_handler(req: &Request) -> Response {
    let n = crate::extract_param::<u64>(&req.path, r"/(?:h[123]/)?cache/(\d+)")
        .unwrap_or(60);
    
    let response_data = json!({
        "headers": req.headers,
        "url": format!("https://httpbin.org{}", req.path)
    });
    
    let mut response = Response::new(200);
    response = response.with_json(&response_data);
    response.headers.insert("Cache-Control".to_string(), format!("public, max-age={}", n));
    response
}

/// Handles /etag/{etag} endpoint
/// Tests ETag validation with If-Match/If-None-Match
pub fn etag_handler(req: &Request) -> Response {
    let etag = crate::extract_params(&req.path, r"/(?:h[123]/)?etag/(.+)")
        .and_then(|v| v.into_iter().next())
        .unwrap_or_else(|| "default-etag".to_string());
    
    let etag_quoted = format!("\"{}\"", etag);
    
    // Check If-None-Match header
    if let Some(if_none_match) = req.headers.get("If-None-Match")
        .or_else(|| req.headers.get("if-none-match")) {
        if if_none_match == &etag_quoted || if_none_match == "*" {
            // ETag matches - return 304
            let mut response = Response::new(304);
            response.headers.insert("ETag".to_string(), etag_quoted);
            return response;
        }
    }
    
    // Check If-Match header
    if let Some(if_match) = req.headers.get("If-Match")
        .or_else(|| req.headers.get("if-match")) {
        if if_match != &etag_quoted && if_match != "*" {
            // ETag doesn't match - return 412 Precondition Failed
            return Response::new(412);
        }
    }
    
    // Normal response
    let response_data = json!({
        "headers": req.headers,
        "url": format!("https://httpbin.org{}", req.path)
    });
    
    let mut response = Response::new(200);
    response = response.with_json(&response_data);
    response.headers.insert("ETag".to_string(), etag_quoted);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_cache_handler_no_cache() {
        let req = Request {
            method: "GET".to_string(),
            path: "/cache".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = cache_handler(&req);
        assert_eq!(response.status, 200);
        assert!(response.headers.contains_key("ETag"));
    }
    
    #[test]
    fn test_cache_handler_with_cache() {
        let mut headers = HashMap::new();
        headers.insert("If-None-Match".to_string(), "\"sample-etag\"".to_string());
        
        let req = Request {
            method: "GET".to_string(),
            path: "/cache".to_string(),
            headers,
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = cache_handler(&req);
        assert_eq!(response.status, 304);
    }
    
    #[test]
    fn test_cache_n_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/cache/3600".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = cache_n_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Cache-Control"), Some(&"public, max-age=3600".to_string()));
    }
    
    #[test]
    fn test_etag_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/etag/test123".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = etag_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("ETag"), Some(&"\"test123\"".to_string()));
    }
}
