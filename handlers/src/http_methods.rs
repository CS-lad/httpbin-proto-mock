use crate::{Request, Response, HttpBinResponse};

/// Handles /get endpoint
/// Returns GET request data
pub fn get_handler(req: &Request) -> Response {
    let response_data = HttpBinResponse {
        args: Some(crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""))),
        headers: Some(req.headers.clone()),
        url: Some(format!("https://httpbin.org{}", req.path)),
        origin: crate::get_client_ip(&req.headers),
    };
    
    Response::new(200).with_json(&response_data)
}

/// Handles /post endpoint
/// Returns POST request data
pub fn post_handler(req: &Request) -> Response {
    let response_data = HttpBinResponse {
        args: Some(crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""))),
        headers: Some(req.headers.clone()),
        url: Some(format!("https://httpbin.org{}", req.path)),
        origin: crate::get_client_ip(&req.headers),
    };
    
    // TODO: Parse body data (will implement later)
    
    Response::new(200).with_json(&response_data)
}

/// Handles /put endpoint
pub fn put_handler(req: &Request) -> Response {
    let response_data = HttpBinResponse {
        args: Some(crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""))),
        headers: Some(req.headers.clone()),
        url: Some(format!("https://httpbin.org{}", req.path)),
        origin: crate::get_client_ip(&req.headers),
    };
    
    Response::new(200).with_json(&response_data)
}

/// Handles /patch endpoint
pub fn patch_handler(req: &Request) -> Response {
    let response_data = HttpBinResponse {
        args: Some(crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""))),
        headers: Some(req.headers.clone()),
        url: Some(format!("https://httpbin.org{}", req.path)),
        origin: crate::get_client_ip(&req.headers),
    };
    
    Response::new(200).with_json(&response_data)
}

/// Handles /delete endpoint
pub fn delete_handler(req: &Request) -> Response {
    let response_data = HttpBinResponse {
        args: Some(crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""))),
        headers: Some(req.headers.clone()),
        url: Some(format!("https://httpbin.org{}", req.path)),
        origin: crate::get_client_ip(&req.headers),
    };
    
    Response::new(200).with_json(&response_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_get_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/get?foo=bar".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = get_handler(&req);
        assert_eq!(response.status, 200);
        assert!(response.headers.get("Content-Type").unwrap().contains("json"));
    }
    
    #[test]
    fn test_post_handler() {
        let req = Request {
            method: "POST".to_string(),
            path: "/post".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = post_handler(&req);
        assert_eq!(response.status, 200);
    }
}
