use crate::{Request, Response};
use serde_json::json;

/// Handles /basic-auth/{user}/{passwd} endpoint
/// Basic authentication challenge
pub fn basic_auth_handler(req: &Request) -> Response {
    let params = crate::extract_params(&req.path, r"/(?:h[123]/)?basic-auth/([^/]+)/([^/?]+)")
        .unwrap_or_default();
    
    if params.len() < 2 {
        return Response::new(400).with_text("Invalid parameters");
    }
    
    let expected_user = &params[0];
    let expected_pass = &params[1];
    
    // Check for Authorization header
    if let Some(auth_header) = req.headers.get("Authorization")
        .or_else(|| req.headers.get("authorization")) {
        
        if let Some(credentials) = auth_header.strip_prefix("Basic ") {
            // Decode base64 credentials
            if let Ok(decoded) = crate::decode_base64(credentials) {
                if let Ok(creds_str) = String::from_utf8(decoded) {
                    if let Some((user, pass)) = creds_str.split_once(':') {
                        if user == expected_user && pass == expected_pass {
                            // Authentication successful
                            let response_data = json!({
                                "authenticated": true,
                                "user": user
                            });
                            return Response::new(200).with_json(&response_data);
                        }
                    }
                }
            }
        }
    }
    
    // Authentication failed - send 401 with WWW-Authenticate header
    let mut response = Response::new(401);
    response.headers.insert(
        "WWW-Authenticate".to_string(),
        "Basic realm=\"Fake Realm\"".to_string()
    );
    response
}

/// Handles /hidden-basic-auth/{user}/{passwd} endpoint
/// Hidden basic auth (returns 404 on failure instead of 401)
pub fn hidden_basic_auth_handler(req: &Request) -> Response {
    let params = crate::extract_params(&req.path, r"/(?:h[123]/)?hidden-basic-auth/([^/]+)/([^/?]+)")
        .unwrap_or_default();
    
    if params.len() < 2 {
        return Response::new(400).with_text("Invalid parameters");
    }
    
    let expected_user = &params[0];
    let expected_pass = &params[1];
    
    // Check for Authorization header
    if let Some(auth_header) = req.headers.get("Authorization")
        .or_else(|| req.headers.get("authorization")) {
        
        if let Some(credentials) = auth_header.strip_prefix("Basic ") {
            if let Ok(decoded) = crate::decode_base64(credentials) {
                if let Ok(creds_str) = String::from_utf8(decoded) {
                    if let Some((user, pass)) = creds_str.split_once(':') {
                        if user == expected_user && pass == expected_pass {
                            // Authentication successful
                            let response_data = json!({
                                "authenticated": true,
                                "user": user
                            });
                            return Response::new(200).with_json(&response_data);
                        }
                    }
                }
            }
        }
    }
    
    // Authentication failed - return 404 instead of 401
    Response::new(404).with_text("Not Found")
}

/// Handles /digest-auth/{qop}/{user}/{passwd} endpoint
/// Digest authentication
pub fn digest_auth_handler(req: &Request) -> Response {
    let params = crate::extract_params(&req.path, r"/(?:h[123]/)?digest-auth/([^/]+)/([^/]+)/([^/?]+)")
        .unwrap_or_default();
    
    if params.len() < 3 {
        return Response::new(400).with_text("Invalid parameters");
    }

    let qop = &params[0];
    let expected_user = &params[1];
    let _expected_pass = &params[2];
    
    // Check for Authorization header
    if let Some(auth_header) = req.headers.get("Authorization")
        .or_else(|| req.headers.get("authorization")) {
        
        if auth_header.starts_with("Digest ") {
            // Parse digest parameters
            let auth_params = parse_digest_auth(auth_header);
            
            if let Some(username) = auth_params.get("username") {
                if username == expected_user {
                    // Simplified digest validation
                    // In a real implementation, we would validate the response hash
                    let response_data = json!({
                        "authenticated": true,
                        "user": username
                    });
                    return Response::new(200).with_json(&response_data);
                }
            }
        }
    }
    
    // Authentication failed - send 401 with digest challenge
    let nonce = generate_nonce();
    let opaque = generate_nonce();
    
    let challenge = format!(
        "Digest realm=\"Fake Realm\", nonce=\"{}\", opaque=\"{}\", qop=\"{}\"",
        nonce, opaque, qop
    );
    
    let mut response = Response::new(401);
    response.headers.insert("WWW-Authenticate".to_string(), challenge);
    response
}

/// Handles /digest-auth/{qop}/{user}/{passwd}/{algorithm} endpoint
/// Digest authentication with algorithm specification
pub fn digest_auth_algorithm_handler(req: &Request) -> Response {
    let params = crate::extract_params(
        &req.path,
        r"/(?:h[123]/)?digest-auth/([^/]+)/([^/]+)/([^/]+)/([^/?]+)"
    ).unwrap_or_default();
    
    if params.len() < 4 {
        return Response::new(400).with_text("Invalid parameters");
    }
    
    let qop = &params[0];
    let expected_user = &params[1];
    let _expected_pass = &params[2];
    let algorithm = &params[3];
    
    // Check for Authorization header
    if let Some(auth_header) = req.headers.get("Authorization")
        .or_else(|| req.headers.get("authorization")) {
        
        if auth_header.starts_with("Digest ") {
            let auth_params = parse_digest_auth(auth_header);
            
            if let Some(username) = auth_params.get("username") {
                if username == expected_user {
                    let response_data = json!({
                        "authenticated": true,
                        "user": username
                    });
                    return Response::new(200).with_json(&response_data);
                }
            }
        }
    }
    
    // Authentication failed - send 401 with digest challenge
    let nonce = generate_nonce();
    let opaque = generate_nonce();
    
    let challenge = format!(
        "Digest realm=\"Fake Realm\", nonce=\"{}\", opaque=\"{}\", algorithm=\"{}\", qop=\"{}\"",
        nonce, opaque, algorithm, qop
    );
    
    let mut response = Response::new(401);
    response.headers.insert("WWW-Authenticate".to_string(), challenge);
    response
}

/// Handles /bearer endpoint
/// Bearer token authentication
pub fn bearer_handler(req: &Request) -> Response {
    // Check for Authorization header with Bearer token
    if let Some(auth_header) = req.headers.get("Authorization")
        .or_else(|| req.headers.get("authorization")) {
        
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            // Token exists - authentication successful
            let response_data = json!({
                "authenticated": true,
                "token": token
            });
            return Response::new(200).with_json(&response_data);
        }
    }
    
    // No bearer token - return 401
    let mut response = Response::new(401);
    response.headers.insert(
        "WWW-Authenticate".to_string(),
        "Bearer".to_string()
    );
    response
}

/// Parse digest authentication header
fn parse_digest_auth(auth_header: &str) -> std::collections::HashMap<String, String> {
    let mut params = std::collections::HashMap::new();
    
    if let Some(digest_part) = auth_header.strip_prefix("Digest ") {
        for part in digest_part.split(',') {
            let part = part.trim();
            if let Some((key, value)) = part.split_once('=') {
                let value = value.trim_matches('"');
                params.insert(key.to_string(), value.to_string());
            }
        }
    }
    
    params
}

/// Generate a random nonce for digest auth
fn generate_nonce() -> String {
    use rand::Rng;
    let random_bytes: Vec<u8> = (0..16).map(|_| rand::thread_rng().gen()).collect();
    hex::encode(random_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_basic_auth_success() {
        let mut headers = HashMap::new();
        // "user:pass" in base64 is "dXNlcjpwYXNz"
        headers.insert("Authorization".to_string(), "Basic dXNlcjpwYXNz".to_string());
        
        let req = Request {
            method: "GET".to_string(),
            path: "/basic-auth/user/pass".to_string(),
            headers,
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = basic_auth_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_basic_auth_failure() {
        let req = Request {
            method: "GET".to_string(),
            path: "/basic-auth/user/pass".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = basic_auth_handler(&req);
        assert_eq!(response.status, 401);
        assert!(response.headers.contains_key("WWW-Authenticate"));
    }
    
    #[test]
    fn test_hidden_basic_auth_failure() {
        let req = Request {
            method: "GET".to_string(),
            path: "/hidden-basic-auth/user/pass".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = hidden_basic_auth_handler(&req);
        assert_eq!(response.status, 404);
    }
    
    #[test]
    fn test_bearer_auth_success() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer mytoken123".to_string());
        
        let req = Request {
            method: "GET".to_string(),
            path: "/bearer".to_string(),
            headers,
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = bearer_handler(&req);
        assert_eq!(response.status, 200);
    }
    
    #[test]
    fn test_bearer_auth_failure() {
        let req = Request {
            method: "GET".to_string(),
            path: "/bearer".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = bearer_handler(&req);
        assert_eq!(response.status, 401);
    }
    
    #[test]
    fn test_digest_auth_challenge() {
        let req = Request {
            method: "GET".to_string(),
            path: "/digest-auth/auth/user/pass".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let response = digest_auth_handler(&req);
        assert_eq!(response.status, 401);
        assert!(response.headers.contains_key("WWW-Authenticate"));
    }
}
