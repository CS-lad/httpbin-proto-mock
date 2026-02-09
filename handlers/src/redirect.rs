use crate::{Request, Response};

/// Handles /redirect/{n} endpoint
/// Redirects n times with absolute URLs
pub fn redirect_handler(req: &Request) -> Response {
    let n = crate::extract_param::<u32>(&req.path, r"/(?:h[123]/)?redirect/(\d+)")
        .unwrap_or(1);
    
    if n <= 1 {
        // Final redirect - go to /get
        let mut response = Response::new(302);
        response.headers.insert("Location".to_string(), "/get".to_string());
        response
    } else {
        // Redirect to n-1
        let next_n = n - 1;
        let prefix = if req.path.contains("/h1/") {
            "/h1"
        } else if req.path.contains("/h2/") {
            "/h2"
        } else if req.path.contains("/h3/") {
            "/h3"
        } else {
            ""
        };
        
        let mut response = Response::new(302);
        response.headers.insert(
            "Location".to_string(),
            format!("{}/redirect/{}", prefix, next_n)
        );
        response
    }
}

/// Handles /relative-redirect/{n} endpoint
/// Redirects n times with relative URLs
pub fn relative_redirect_handler(req: &Request) -> Response {
    let n = crate::extract_param::<u32>(&req.path, r"/(?:h[123]/)?relative-redirect/(\d+)")
        .unwrap_or(1);
    
    if n <= 1 {
        // Final redirect - go to /get
        let mut response = Response::new(302);
        response.headers.insert("Location".to_string(), "/get".to_string());
        response
    } else {
        // Redirect to n-1
        let next_n = n - 1;
        let prefix = if req.path.contains("/h1/") {
            "/h1"
        } else if req.path.contains("/h2/") {
            "/h2"
        } else if req.path.contains("/h3/") {
            "/h3"
        } else {
            ""
        };
        
        let mut response = Response::new(302);
        response.headers.insert(
            "Location".to_string(),
            format!("{}/relative-redirect/{}", prefix, next_n)
        );
        response
    }
}

/// Handles /absolute-redirect/{n} endpoint
/// Same as /redirect/{n} but with absolute URLs
pub fn absolute_redirect_handler(req: &Request) -> Response {
    redirect_handler(req)
}

/// Handles /redirect-to endpoint
/// Redirects to URL specified in query parameter
pub fn redirect_to_handler(req: &Request) -> Response {
    let query = crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""));
    
    let url = query.get("url")
        .cloned()
        .unwrap_or_else(|| "/get".to_string());
    
    let status_code = query.get("status_code")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(302);
    
    let mut response = Response::new(status_code);
    response.headers.insert("Location".to_string(), url);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_redirect_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/redirect/3".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = redirect_handler(&req);
        assert_eq!(response.status, 302);
        assert_eq!(response.headers.get("Location"), Some(&"/redirect/2".to_string()));
    }
    
    #[test]
    fn test_redirect_handler_final() {
        let req = Request {
            method: "GET".to_string(),
            path: "/redirect/1".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = redirect_handler(&req);
        assert_eq!(response.status, 302);
        assert_eq!(response.headers.get("Location"), Some(&"/get".to_string()));
    }
    
    #[test]
    fn test_redirect_to_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/redirect-to?url=https://example.com&status_code=301".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = redirect_to_handler(&req);
        assert_eq!(response.status, 301);
        assert_eq!(response.headers.get("Location"), Some(&"https://example.com".to_string()));
    }
}
