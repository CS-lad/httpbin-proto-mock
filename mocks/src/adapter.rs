//! Adapter to convert between orb-mockhttp and httpbin-handlers types

use orb_mockhttp::{Request as OrbRequest, Response as OrbResponse, ResponseBuilder};
use httpbin_handlers::{Request, Response};
use std::collections::HashMap;

/// Convert orb-mockhttp Request to our Request type
pub fn to_handler_request(orb_req: &OrbRequest) -> Request {
    let mut headers = HashMap::new();

    // Convert headers
    for (name, value) in orb_req.headers() {
        if let Ok(val_str) = value.to_str() {
            headers.insert(name.to_string(), val_str.to_string());
        }
    }

    Request {
        method: orb_req.method().to_string(),
        path: orb_req.uri().path().to_string(),
        headers,
        http_version: format!("{:?}", orb_req.version()),
    }
}

/// Convert our Response to orb-mockhttp Response
pub fn to_orb_response(handler_resp: Response) -> OrbResponse {
    let mut builder = ResponseBuilder::new()
        .status(handler_resp.status);

    // Add headers
    for (name, value) in handler_resp.headers {
        builder = builder.header(&name, &value);
    }

    // Add body
    builder = builder.body(handler_resp.body);

    builder.build()
}

/// Create a 421 Misdirected Request response
pub fn misdirected_request() -> OrbResponse {
    ResponseBuilder::new()
        .status(421)
        .body(b"Misdirected Request".to_vec())
        .build()
}
