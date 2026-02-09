use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

/// Extract a path parameter using regex
/// 
/// Example: extract_param::<u16>("/status/200", r"/status/(\d+)") -> Some(200)
pub fn extract_param<T: FromStr>(path: &str, pattern: &str) -> Option<T> {
    let re = Regex::new(pattern).ok()?;
    re.captures(path)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

/// Extract multiple path parameters
pub fn extract_params(path: &str, pattern: &str) -> Option<Vec<String>> {
    let re = Regex::new(pattern).ok()?;
    let caps = re.captures(path)?;
    
    let mut params = Vec::new();
    for i in 1..caps.len() {
        if let Some(m) = caps.get(i) {
            params.push(m.as_str().to_string());
        }
    }
    
    Some(params)
}

/// Parse query string into HashMap
/// 
/// Example: parse_query("foo=bar&baz=qux") -> {"foo": "bar", "baz": "qux"}
pub fn parse_query(query: &str) -> HashMap<String, String> {
    if query.is_empty() {
        return HashMap::new();
    }
    
    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

/// Get client IP from request headers
/// Checks X-Forwarded-For first, then X-Real-IP
pub fn get_client_ip(headers: &HashMap<String, String>) -> Option<String> {
    headers
        .get("X-Forwarded-For")
        .or_else(|| headers.get("x-forwarded-for"))
        .or_else(|| headers.get("X-Real-IP"))
        .or_else(|| headers.get("x-real-ip"))
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
}

/// Generate random bytes
pub fn random_bytes(n: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen()).collect()
}

/// Decode base64 string
pub fn decode_base64(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.decode(s)
}

/// Encode bytes to base64 string
pub fn encode_base64(data: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_param() {
        let result: Option<u16> = extract_param("/status/200", r"/status/(\d+)");
        assert_eq!(result, Some(200));
        
        let result: Option<u16> = extract_param("/status/abc", r"/status/(\d+)");
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_parse_query() {
        let result = parse_query("foo=bar&baz=qux");
        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.get("baz"), Some(&"qux".to_string()));
        
        let result = parse_query("");
        assert!(result.is_empty());
    }
    
    #[test]
    fn test_get_client_ip() {
        let mut headers = HashMap::new();
        headers.insert("X-Forwarded-For".to_string(), "192.168.1.1".to_string());
        
        assert_eq!(get_client_ip(&headers), Some("192.168.1.1".to_string()));
        
        headers.clear();
        assert_eq!(get_client_ip(&headers), None);
    }
    
    #[test]
    fn test_base64() {
        let data = b"Hello, World!";
        let encoded = encode_base64(data);
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
