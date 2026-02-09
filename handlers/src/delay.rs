use crate::{Request, Response, HttpBinResponse};
use tokio::time::{sleep, Duration};

/// Handles /delay/{n} endpoint
/// Delays response for n seconds (max 10)
pub async fn delay_handler(req: &Request) -> Response {
    let seconds = crate::extract_param::<u64>(&req.path, r"/(?:h[123]/)?delay/(\d+)")
        .unwrap_or(1);
    
    // Cap at 10 seconds to prevent abuse
    let seconds = seconds.min(10);
    
    // Sleep for the specified duration
    sleep(Duration::from_secs(seconds)).await;
    
    let response_data = HttpBinResponse {
        args: Some(crate::parse_query(&req.path.split('?').nth(1).unwrap_or(""))),
        headers: Some(req.headers.clone()),
        url: Some(format!("https://httpbin.org{}", req.path)),
        origin: crate::get_client_ip(&req.headers),
    };
    
    Response::new(200).with_json(&response_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_delay_handler() {
        let req = Request {
            method: "GET".to_string(),
            path: "/delay/2".to_string(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let start = Instant::now();
        let response = delay_handler(&req).await;
        let elapsed = start.elapsed();
        
        assert_eq!(response.status, 200);
        assert!(elapsed.as_secs() >= 2);
        assert!(elapsed.as_secs() < 3);
    }
    
    #[tokio::test]
    async fn test_delay_handler_max_cap() {
        let req = Request {
            method: "GET".to_string(),
            path: "/delay/100".to_string(), // Request 100 seconds
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_string(),
        };
        
        let start = Instant::now();
        let response = delay_handler(&req).await;
        let elapsed = start.elapsed();
        
        assert_eq!(response.status, 200);
        // Should be capped at 10 seconds
        assert!(elapsed.as_secs() >= 10);
        assert!(elapsed.as_secs() < 11);
    }
}
