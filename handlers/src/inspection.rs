use crate::{Request, Response};
use serde_json::json;
use uuid::Uuid;

/// Handles /headers endpoint
/// Returns all request headers
pub fn headers_handler(req: &Request) -> Response {
    let response_data = json!({
        "headers": req.headers
    });
    
    Response::new(200).with_json(&response_data)
}

/// Handles /user-agent endpoint
/// Returns the user agent string
pub fn user_agent_handler(req: &Request) -> Response {
    let user_agent = req.headers
        .get("User-Agent")
        .or_else(|| req.headers.get("user-agent"))
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());
    
    let response_data = json!({
        "user-agent": user_agent
    });
    
    Response::new(200).with_json(&response_data)
}

/// Handles /ip endpoint
/// Returns origin IP address
pub fn ip_handler(req: &Request) -> Response {
    let origin = crate::get_client_ip(&req.headers)
        .unwrap_or_else(|| "127.0.0.1".to_string());
    
    let response_data = json!({
        "origin": origin
    });
    
    Response::new(200).with_json(&response_data)
}

/// Handles /uuid endpoint
/// Returns a UUID4
pub fn uuid_handler(_req: &Request) -> Response {
    let uuid = Uuid::new_v4();
    
    let response_data = json!({
        "uuid": uuid.to_string()
    });
    
    Response::new(200).with_json(&response_data)
}

/// Handles /base64/{value} endpoint
/// Decodes base64 value
pub fn base64_handler(req: &Request) -> Response {
    // Extract base64 value from path
    let value = crate::extract_params(&req.path, r"/(?:h[123]/)?base64/(.+)")
        .and_then(|v| v.into_iter().next())
        .unwrap_or_default();
    
    match crate::decode_base64(&value) {
        Ok(decoded) => {
            Response::new(200).with_text(&String::from_utf8_lossy(&decoded))
        }
        Err(_) => {
            Response::new(400).with_text("Invalid base64")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_headers_handler() {
        let mut headers = HashMap::new();
        headers.insert("X-Test".to_string(), "value".to_string());
        
        let req = Request {
            method: "GET".to_string(),
            path: "/headers".to_string(),
            headers,
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = headers_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_uuid_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/uuid".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = uuid_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_base64_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/base64/SGVsbG8sIFdvcmxkIQ==".to_string(), // "Hello, World!"
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = base64_handler(&req);
        assert_eq!(response.status, 200);
        let body = String::from_utf8(response.body).unwrap();
        assert_eq!(body, "Hello, World!");
    }
}
