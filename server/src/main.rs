//! httpbin-proto-mock server
//! Protocol-aware HTTP testing server

use orb_mockhttp::{TestServerBuilder, HttpProtocol};

fn main() {
    println!("=== httpbin-proto-mock Server ===");
    println!();
    println!("Building test server with HTTP/1.1, HTTP/2, and HTTP/3 support...");
    
    let server = TestServerBuilder::new()
        .with_tls()
        .with_protocols(&[
            HttpProtocol::Http1,
            HttpProtocol::Http2,
            HttpProtocol::Http3,
        ])
        .build();
    
    println!("Registering protocol-agnostic endpoints (52)...");
    httpbin_mocks::register_any_protocol_mocks(&server);
    
    println!("Registering HTTP/1.1-only endpoints (52)...");
    httpbin_mocks::register_h1_mocks(&server);
    
    println!("Registering HTTP/2-only endpoints (52)...");
    httpbin_mocks::register_h2_mocks(&server);
    
    println!("Registering HTTP/3-only endpoints (52)...");
    httpbin_mocks::register_h3_mocks(&server);
    
    println!();
    println!("✅ All 208 endpoints registered!");
    println!();
    println!("Server ready at: {}", server.url("/"));
    println!("  - HTTP/1.1 over TLS");
    println!("  - HTTP/2 over TLS");
    println!("  - HTTP/3 over QUIC");
    println!();
    println!("Example endpoints:");
    println!("  {}", server.url("/status/200"));
    println!("  {}", server.url("/get"));
    println!("  {}", server.url("/uuid"));
    println!();
    println!("Note: Handlers are registered but not yet integrated.");
    println!("      All endpoints return placeholder responses.");
    println!();
    println!("Press Ctrl+C to stop");
    
    // Keep server running
    std::thread::park();
}
