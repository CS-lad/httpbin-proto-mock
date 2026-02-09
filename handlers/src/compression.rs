use crate::{Request, Response};
use flate2::write::{GzEncoder, DeflateEncoder};
use flate2::Compression;
use std::io::Write;

/// Handles /gzip endpoint
/// Returns gzip-compressed response
pub fn gzip_handler(req: &Request) -> Response {
    let data = get_sample_data(req);
    
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();
    
    let mut response = Response::new(200);
    response.body = compressed;
    response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    response.headers.insert("Content-Encoding".to_string(), "gzip".to_string());
    response
}

/// Handles /deflate endpoint
/// Returns deflate-compressed response
pub fn deflate_handler(req: &Request) -> Response {
    let data = get_sample_data(req);
    
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();
    
    let mut response = Response::new(200);
    response.body = compressed;
    response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    response.headers.insert("Content-Encoding".to_string(), "deflate".to_string());
    response
}

/// Handles /brotli endpoint
/// Returns brotli-compressed response
pub fn brotli_handler(req: &Request) -> Response {
    let data = get_sample_data(req);
    
    let mut compressed = Vec::new();
    let mut compressor = brotli::CompressorWriter::new(&mut compressed, 4096, 6, 22);
    compressor.write_all(data.as_bytes()).unwrap();
    drop(compressor);
    
    let mut response = Response::new(200);
    response.body = compressed;
    response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    response.headers.insert("Content-Encoding".to_string(), "br".to_string());
    response
}

fn get_sample_data(req: &Request) -> String {
    serde_json::json!({
        "headers": req.headers,
        "origin": crate::get_client_ip(&req.headers).unwrap_or_else(|| "127.0.0.1".to_string()),
        "url": format!("https://httpbin.org{}", req.path),
        "gzipped": true,
        "deflated": true,
        "brotli": true,
        "method": req.method
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_gzip_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/gzip".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = gzip_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Content-Encoding"), Some(&"gzip".to_string()));
        assert!(!response.body.is_empty());
    }
    
    #[test]
    fn test_deflate_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/deflate".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = deflate_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Content-Encoding"), Some(&"deflate".to_string()));
        assert!(!response.body.is_empty());
    }
    
    #[test]
    fn test_brotli_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/brotli".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = brotli_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Content-Encoding"), Some(&"br".to_string()));
        assert!(!response.body.is_empty());
    }
}
