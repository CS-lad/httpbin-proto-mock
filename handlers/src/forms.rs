use crate::{Request, Response};

/// Handles /forms/post endpoint
/// HTML form that posts to /post
pub fn forms_post_handler(_req: &Request) -> Response {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Form Test</title>
</head>
<body>
    <h1>HTML Form</h1>
    <form method="POST" action="/post">
        <div>
            <label for="custname">Name:</label>
            <input type="text" id="custname" name="custname" required>
        </div>
        <div>
            <label for="custtel">Telephone:</label>
            <input type="tel" id="custtel" name="custtel">
        </div>
        <div>
            <label for="custemail">Email:</label>
            <input type="email" id="custemail" name="custemail">
        </div>
        <div>
            <label for="size">Size:</label>
            <select id="size" name="size">
                <option value="small">Small</option>
                <option value="medium">Medium</option>
                <option value="large">Large</option>
            </select>
        </div>
        <div>
            <label for="comments">Comments:</label>
            <textarea id="comments" name="comments" rows="4"></textarea>
        </div>
        <div>
            <input type="submit" value="Submit">
        </div>
    </form>
</body>
</html>"#;
    
    let mut response = Response::new(200);
    response.body = html.as_bytes().to_vec();
    response.headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    response
}

/// Handles /response-headers endpoint
/// Returns custom response headers specified in query parameters
pub fn response_headers_handler(req: &Request) -> Response {
    let query = crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""));
    
    let mut response = Response::new(200);
    
    // Add all query parameters as response headers
    for (key, value) in query.iter() {
        response.headers.insert(key.clone(), value.clone());
    }
    
    // Also return the headers in the body
    let response_data = serde_json::json!({
        "Content-Length": response.body.len(),
        "Content-Type": "application/json"
    });
    
    response = response.with_json(&response_data);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_forms_post_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/forms/post".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = forms_post_handler(&req);
        assert_eq!(response.status, 200);
        let body = String::from_utf8(response.body).unwrap();
        assert!(body.contains("<form"));
        assert!(body.contains("method=\"POST\""));
    }
    
    #[test]
    fn test_response_headers_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/response-headers?X-Custom-Header=value&X-Another=test".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = response_headers_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("X-Custom-Header"), Some(&"value".to_string()));
        assert_eq!(response.headers.get("X-Another"), Some(&"test".to_string()));
    }
}
