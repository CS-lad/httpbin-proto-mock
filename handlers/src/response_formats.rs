use crate::{Request, Response};
use serde_json::json;

/// Handles /json endpoint
/// Returns a sample JSON response
pub fn json_handler(_req: &Request) -> Response {
    let response_data = json!({
        "slideshow": {
            "author": "Yours Truly",
            "date": "date of publication",
            "slides": [
                {
                    "title": "Wake up to WonderWidgets!",
                    "type": "all"
                },
                {
                    "items": [
                        "Why <em>WonderWidgets</em> are great",
                        "Who <em>buys</em> WonderWidgets"
                    ],
                    "title": "Overview",
                    "type": "all"
                }
            ],
            "title": "Sample Slide Show"
        }
    });
    
    Response::new(200).with_json(&response_data)
}

/// Handles /html endpoint
/// Returns a simple HTML page
pub fn html_handler(_req: &Request) -> Response {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Herman Melville - Moby-Dick</title>
</head>
<body>
    <h1>Herman Melville - Moby-Dick</h1>
    <div>
        <p>
            Call me Ishmael. Some years ago—never mind how long precisely—having little or no money in my purse, 
            and nothing particular to interest me on shore, I thought I would sail about a little and see the watery 
            part of the world...
        </p>
    </div>
</body>
</html>"#;
    
    let mut response = Response::new(200);
    response.body = html.as_bytes().to_vec();
    response.headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    response
}

/// Handles /xml endpoint
/// Returns a sample XML response
pub fn xml_handler(_req: &Request) -> Response {
    let xml = r#"<?xml version='1.0' encoding='us-ascii'?>
<slideshow 
    title="Sample Slide Show"
    date="Date of publication"
    author="Yours Truly">
    <slide type="all">
        <title>Wake up to WonderWidgets!</title>
    </slide>
    <slide type="all">
        <title>Overview</title>
        <item>Why WonderWidgets are great</item>
        <item>Who buys WonderWidgets</item>
    </slide>
</slideshow>"#;
    
    let mut response = Response::new(200);
    response.body = xml.as_bytes().to_vec();
    response.headers.insert("Content-Type".to_string(), "application/xml".to_string());
    response
}

/// Handles /robots.txt endpoint
/// Returns a robots.txt file
pub fn robots_txt_handler(_req: &Request) -> Response {
    let robots = "User-agent: *\nDisallow: /deny\n";
    
    let mut response = Response::new(200);
    response.body = robots.as_bytes().to_vec();
    response.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    response
}

/// Handles /deny endpoint
/// Returns a page that denies access
pub fn deny_handler(_req: &Request) -> Response {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>httpbin.org</title>
</head>
<body>
    <h1>YOU SHOULDN'T BE HERE</h1>
    <p>This page is used by robots.txt to deny access.</p>
</body>
</html>"#;
    
    let mut response = Response::new(200);
    response.body = html.as_bytes().to_vec();
    response.headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    response
}

/// Handles /encoding/utf8 endpoint
/// Returns UTF-8 encoded data
pub fn encoding_utf8_handler(_req: &Request) -> Response {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>UTF-8 Encoding Test</title>
</head>
<body>
    <h1>UTF-8 Demo</h1>
    <p>Hello world! Привет мир! 你好世界! مرحبا بالعالم!</p>
    <p>Émojis: 🎉 �� ✅ 💻 🌍</p>
</body>
</html>"#;
    
    let mut response = Response::new(200);
    response.body = html.as_bytes().to_vec();
    response.headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_json_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/json".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = json_handler(&req);
        assert_eq!(response.status, 200);
        assert!(response.headers.get("Content-Type").unwrap().contains("json"));
    }
    
    #[test]
    fn test_html_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/html".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = html_handler(&req);
        assert_eq!(response.status, 200);
        assert!(response.headers.get("Content-Type").unwrap().contains("html"));
    }
    
    #[test]
    fn test_xml_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/xml".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = xml_handler(&req);
        assert_eq!(response.status, 200);
        assert!(response.headers.get("Content-Type").unwrap().contains("xml"));
    }
    
    #[test]
    fn test_robots_txt_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/robots.txt".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = robots_txt_handler(&req);
        assert_eq!(response.status, 200);
        let body = String::from_utf8(response.body).unwrap();
        assert!(body.contains("User-agent"));
    }
}
