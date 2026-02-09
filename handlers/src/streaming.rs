use crate::{Request, Response};
use serde_json::json;

/// Handles /bytes/{n} endpoint
/// Generates n random bytes of binary data
pub fn bytes_handler(req: &Request) -> Response {
    let n = crate::extract_param::<usize>(&req.path, r"/(?:h[123]/)?bytes/(\d+)")
        .unwrap_or(1024);
    
    // Cap at 100KB to prevent abuse
    let n = n.min(102400);
    
    let data = crate::random_bytes(n);
    
    let mut response = Response::new(200);
    response.body = data;
    response.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
    response
}

/// Handles /stream-bytes/{n} endpoint
/// Streams n random bytes in chunked encoding
pub fn stream_bytes_handler(req: &Request) -> Response {
    // For now, same as bytes_handler
    // In a real implementation, this would use chunked transfer encoding
    bytes_handler(req)
}

/// Handles /stream/{n} endpoint
/// Streams n JSON responses
pub fn stream_handler(req: &Request) -> Response {
    let n = crate::extract_param::<usize>(&req.path, r"/(?:h[123]/)?stream/(\d+)")
        .unwrap_or(10);
    
    // Cap at 100 items
    let n = n.min(100);
    
    let mut lines = Vec::new();
    for i in 0..n {
        let item = json!({
            "id": i,
            "url": format!("https://httpbin.org{}", req.path),
            "headers": req.headers
        });
        lines.push(serde_json::to_string(&item).unwrap());
    }
    
    let body = lines.join("\n");
    
    let mut response = Response::new(200);
    response.body = body.into_bytes();
    response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    response
}

/// Handles /drip endpoint
/// Drips data over duration with optional delay
pub fn drip_handler(req: &Request) -> Response {
    let query = crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""));
    
    let _duration = query.get("duration")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(2);
    
    let numbytes = query.get("numbytes")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    
    let _delay = query.get("delay")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    
    // For now, just return the data at once
    // In a real implementation, this would drip data over time
    let data = crate::random_bytes(numbytes);
    
    let mut response = Response::new(200);
    response.body = data;
    response.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
    response
}

/// Handles /range/{n} endpoint
/// Streams n bytes with Range header support
pub fn range_handler(req: &Request) -> Response {
    let n = crate::extract_param::<usize>(&req.path, r"/(?:h[123]/)?range/(\d+)")
        .unwrap_or(1024);
    
    let n = n.min(102400);
    
    // Check for Range header
    let range_header = req.headers.get("Range")
        .or_else(|| req.headers.get("range"));
    
    let data = crate::random_bytes(n);
    
    if let Some(range) = range_header {
        // Parse Range header (simplified)
        if let Some(range_str) = range.strip_prefix("bytes=") {
            if let Some((start_str, end_str)) = range_str.split_once('-') {
                let start = start_str.parse::<usize>().unwrap_or(0);
                let end = if end_str.is_empty() {
                    n - 1
                } else {
                    end_str.parse::<usize>().unwrap_or(n - 1).min(n - 1)
                };
                
                if start < data.len() && start <= end {
                    let range_data = data[start..=end.min(data.len() - 1)].to_vec();
                    let mut response = Response::new(206); // Partial Content
                    response.body = range_data;
                    response.headers.insert(
                        "Content-Range".to_string(),
                        format!("bytes {}-{}/{}", start, end.min(data.len() - 1), n)
                    );
                    response.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
                    return response;
                }
            }
        }
    }
    
    // No range or invalid range - return full content
    let mut response = Response::new(200);
    response.body = data;
    response.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
    response.headers.insert("Accept-Ranges".to_string(), "bytes".to_string());
    response
}

/// Handles /links/{n}/{offset} endpoint
/// Generates page with n links
pub fn links_handler(req: &Request) -> Response {
    let params = crate::extract_params(&req.path, r"/(?:h[123]/)?links/(\d+)(?:/(\d+))?")
        .unwrap_or_default();
    
    let n = params.get(0)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10)
        .min(200); // Cap at 200 links
    
    let offset = params.get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    
    let mut html = String::from("<!DOCTYPE html>\n<html>\n<head><title>Links</title></head>\n<body>\n");
    
    for i in 0..n {
        let link_num = i + offset;
        html.push_str(&format!("<a href=\"/links/{}/{}\">Link {}</a><br>\n", n, link_num, link_num));
    }
    
    html.push_str("</body>\n</html>");
    
    let mut response = Response::new(200);
    response.body = html.into_bytes();
    response.headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_bytes_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/bytes/100".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = bytes_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.body.len(), 100);
    }
    
    #[test]
    fn test_stream_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/stream/5".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = stream_handler(&req);
        assert_eq!(response.status, 200);
        let body = String::from_utf8(response.body).unwrap();
        assert_eq!(body.lines().count(), 5);
    }
    
    #[test]
    fn test_range_handler() {
        let mut headers = HashMap::new();
        headers.insert("Range".to_string(), "bytes=0-99".to_string());
        
        let req = Request {
            method: "GET".to_string(),
            path: "/range/1024".to_string(),
            headers,
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = range_handler(&req);
        assert_eq!(response.status, 206);
        assert_eq!(response.body.len(), 100);
    }
    
    #[test]
    fn test_links_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/links/5/0".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = links_handler(&req);
        assert_eq!(response.status, 200);
        let body = String::from_utf8(response.body).unwrap();
        assert!(body.contains("<a href"));
    }
}
