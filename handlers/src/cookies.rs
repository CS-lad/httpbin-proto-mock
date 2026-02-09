use crate::{Request, Response};
use serde_json::json;

/// Handles /cookies endpoint
/// Returns cookie data from request
pub fn cookies_handler(req: &Request) -> Response {
    let cookies = parse_cookies(&req.headers);
    
    let response_data = json!({
        "cookies": cookies
    });
    
    Response::new(200).with_json(&response_data)
}

/// Handles /cookies/set endpoint
/// Sets cookies from query parameters
pub fn cookies_set_handler(req: &Request) -> Response {
    let query = crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""));
    
    let mut response = Response::new(302);
    
    // Set cookies from query parameters
    for (key, value) in query.iter() {
        let cookie = format!("{}={}", key, value);
        response.headers.insert("Set-Cookie".to_string(), cookie);
    }
    
    // Redirect to /cookies to show the set cookies
    response.headers.insert("Location".to_string(), "/cookies".to_string());
    response
}

/// Handles /cookies/delete endpoint
/// Deletes cookies specified in query parameters
pub fn cookies_delete_handler(req: &Request) -> Response {
    let query = crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""));
    
    let mut response = Response::new(302);
    
    // Delete cookies by setting them with Max-Age=0
    for key in query.keys() {
        let cookie = format!("{}=deleted; Max-Age=0", key);
        response.headers.insert("Set-Cookie".to_string(), cookie);
    }
    
    // Redirect to /cookies
    response.headers.insert("Location".to_string(), "/cookies".to_string());
    response
}

/// Handles /cookies/set/{name}/{value} endpoint
/// Sets a specific cookie
pub fn cookies_set_specific_handler(req: &Request) -> Response {
    let params = crate::extract_params(&req.path, r"/(?:h[123]/)?cookies/set/([^/]+)/([^/?]+)")
        .unwrap_or_default();
    
    if params.len() < 2 {
        return Response::new(400).with_text("Invalid cookie parameters");
    }
    
    let name = &params[0];
    let value = &params[1];
    
    let mut response = Response::new(302);
    response.headers.insert("Set-Cookie".to_string(), format!("{}={}", name, value));
    response.headers.insert("Location".to_string(), "/cookies".to_string());
    response
}

/// Parse cookies from Cookie header
fn parse_cookies(headers: &std::collections::HashMap<String, String>) -> std::collections::HashMap<String, String> {
    let cookie_header = headers.get("Cookie")
        .or_else(|| headers.get("cookie"))
        .cloned()
        .unwrap_or_default();
    
    cookie_header
        .split(';')
        .filter_map(|pair| {
            let mut parts = pair.trim().splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let value = parts.next()?.trim().to_string();
            Some((key, value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_cookies_handler() {
        let mut headers = HashMap::new();
        headers.insert("Cookie".to_string(), "foo=bar; baz=qux".to_string());
        
        let req = Request {
            method: "GET".to_string(),
            path: "/cookies".to_string(),
            headers,
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = cookies_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_cookies_set_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/cookies/set?foo=bar&baz=qux".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = cookies_set_handler(&req);
        assert_eq!(response.status, 302);
        assert!(response.headers.contains_key("Set-Cookie"));
    }
    
    #[test]
    fn test_cookies_delete_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/cookies/delete?foo&baz".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = cookies_delete_handler(&req);
        assert_eq!(response.status, 302);
    }
}
