use crate::{Request, Response};

/// Handles /image endpoint
/// Returns image based on Accept header
pub fn image_handler(req: &Request) -> Response {
    let accept = req.headers.get("Accept")
        .or_else(|| req.headers.get("accept"))
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    
    if accept.contains("image/webp") {
        image_webp_handler(req)
    } else if accept.contains("image/svg") {
        image_svg_handler(req)
    } else if accept.contains("image/jpeg") {
        image_jpeg_handler(req)
    } else {
        image_png_handler(req)
    }
}

/// Handles /image/png endpoint
/// Returns a PNG image
pub fn image_png_handler(_req: &Request) -> Response {
    // Simple 1x1 PNG (smallest valid PNG)
    let png_data: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
        0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41,
        0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00,
        0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
        0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE,
        0x42, 0x60, 0x82,
    ];
    
    let mut response = Response::new(200);
    response.body = png_data;
    response.headers.insert("Content-Type".to_string(), "image/png".to_string());
    response
}

/// Handles /image/jpeg endpoint
/// Returns a JPEG image
pub fn image_jpeg_handler(_req: &Request) -> Response {
    // Simple 1x1 JPEG
    let jpeg_data: Vec<u8> = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46,
        0x49, 0x46, 0x00, 0x01, 0x01, 0x01, 0x00, 0x48,
        0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43,
        0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00,
        0x01, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00,
        0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x08, 0x01,
        0x01, 0x00, 0x00, 0x3F, 0x00, 0x7F, 0xFF, 0xD9,
    ];
    
    let mut response = Response::new(200);
    response.body = jpeg_data;
    response.headers.insert("Content-Type".to_string(), "image/jpeg".to_string());
    response
}

/// Handles /image/webp endpoint
/// Returns a WebP image
pub fn image_webp_handler(_req: &Request) -> Response {
    // Simple 1x1 WebP
    let webp_data: Vec<u8> = vec![
        0x52, 0x49, 0x46, 0x46, 0x1A, 0x00, 0x00, 0x00,
        0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38, 0x20,
        0x0E, 0x00, 0x00, 0x00, 0x30, 0x01, 0x00, 0x9D,
        0x01, 0x2A, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    
    let mut response = Response::new(200);
    response.body = webp_data;
    response.headers.insert("Content-Type".to_string(), "image/webp".to_string());
    response
}

/// Handles /image/svg endpoint
/// Returns an SVG image
pub fn image_svg_handler(_req: &Request) -> Response {
    let svg = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\">\n\
    <rect width=\"100\" height=\"100\" fill=\"#3498db\"/>\n\
    <text x=\"50\" y=\"55\" font-size=\"20\" text-anchor=\"middle\" fill=\"white\">SVG</text>\n\
</svg>";
    
    let mut response = Response::new(200);
    response.body = svg.as_bytes().to_vec();
    response.headers.insert("Content-Type".to_string(), "image/svg+xml".to_string());
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_image_png_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/image/png".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = image_png_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Content-Type"), Some(&"image/png".to_string()));
    }
    
    #[test]
    fn test_image_jpeg_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/image/jpeg".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = image_jpeg_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Content-Type"), Some(&"image/jpeg".to_string()));
    }
    
    #[test]
    fn test_image_svg_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/image/svg".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = image_svg_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Content-Type"), Some(&"image/svg+xml".to_string()));
    }
    
    #[test]
    fn test_image_handler_with_accept() {
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "image/webp".to_string());
        
        let req = Request {
            method: "GET".to_string(),
            path: "/image".to_string(),
            headers,
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = image_handler(&req);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("Content-Type"), Some(&"image/webp".to_string()));
    }
}
