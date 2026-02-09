use crate::{Request, Response};
use serde_json::json;

/// Handles /anything endpoint
/// Returns anything (echoes request data)
pub fn anything_handler(req: &Request) -> Response {
    let response_data = json!({
        "args": crate::parse_query(&req.path.split('?').nth(1).unwrap_or("")),
        "headers": req.headers,
        "method": req.method,
        "origin": crate::get_client_ip(&req.headers).unwrap_or_else(|| "127.0.0.1".to_string()),
        "url": format!("https://httpbin.org{}", req.path)
    });
    
    Response::new(200).with_json(&response_data)
}

/// Handles /anything/{anything} endpoint
/// Same as /anything but with path capture
pub fn anything_path_handler(req: &Request) -> Response {
    anything_handler(req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_anything_handler() {
        let req = Request {
            method: "POST".to_string(),
            path: "/anything?foo=bar".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = anything_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_anything_path_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/anything/some/path".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = anything_path_handler(&req);
        assert_eq!(response.status, 200);
    }
}
