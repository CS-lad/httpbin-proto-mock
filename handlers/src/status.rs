use crate::{Request, Response};
use regex::Regex;

/// Handles /status/{code} endpoint
/// Returns the specified HTTP status code
/// 
/// Examples:
/// - /status/200 -> Returns 200 OK
/// - /h2/status/404 -> Returns 404 Not Found
pub fn status_handler(req: &Request) -> Response {
    // Extract status code from path
    // Matches: /status/200, /h1/status/404, /h2/status/500, etc.
    let re = Regex::new(r"/(?:h[123]/)?status/(\d+)").unwrap();
    
    let code = re.captures(&req.path)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<u16>().ok())
        .unwrap_or(200);
    
    // Validate status code is in valid range (100-599)
    let code = if (100..=599).contains(&code) {
        code
    } else {
        400  // Return 400 Bad Request for invalid codes
    };
    
    Response::new(code)
        .with_text(&format!("Status: {}", code))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_status_200() {
        let req = Request {
            method: "GET".to_string(),
            path: "/status/200".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = status_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_status_404() {
        let req = Request {
            method: "GET".to_string(),
            path: "/status/404".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = status_handler(&req);
        assert_eq!(response.status, 404);
    }
    
    #[test]
    fn test_status_with_h2_prefix() {
        let req = Request {
            method: "GET".to_string(),
            path: "/h2/status/200".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/2".to_string(),
        };
        
        let response = status_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_invalid_status_code() {
        let req = Request {
            method: "GET".to_string(),
            path: "/status/999".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = status_handler(&req);
        assert_eq!(response.status, 400);
    }
}
