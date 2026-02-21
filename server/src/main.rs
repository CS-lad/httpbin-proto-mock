//! httpbin-proto-mock server
//! Protocol-aware HTTP testing server

use orb_mockhttp::{TestServerBuilder, HttpProtocol};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const DEFAULT_PORT: u16 = 8080;

fn main() {
    let fixed_port: u16 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT);

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

    let internal_port = server.port();

    println!();
    println!("All 208 endpoints registered!");
    println!();
    println!("Server ready at: https://127.0.0.1:{}/", fixed_port);
    println!("  - HTTP/1.1 over TLS");
    println!("  - HTTP/2 over TLS");
    println!("  - HTTP/3 over QUIC");
    println!();
    println!("Example endpoints:");
    println!("  https://127.0.0.1:{}/status/200", fixed_port);
    println!("  https://127.0.0.1:{}/get", fixed_port);
    println!("  https://127.0.0.1:{}/uuid", fixed_port);
    println!();
    println!("Press Ctrl+C to stop");

    // Listen on the fixed port and proxy to the internal random port
    let listener = TcpListener::bind(format!("127.0.0.1:{}", fixed_port))
        .unwrap_or_else(|e| {
            eprintln!("Failed to bind to port {}: {}", fixed_port, e);
            eprintln!("Try a different port: ./httpbin-server <port>");
            std::process::exit(1);
        });

    for incoming in listener.incoming() {
        match incoming {
            Ok(client) => {
                let target_port = internal_port;
                std::thread::spawn(move || {
                    proxy_connection(client, target_port);
                });
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}

fn proxy_connection(mut client: TcpStream, target_port: u16) {
    let mut target = match TcpStream::connect(format!("127.0.0.1:{}", target_port)) {
        Ok(t) => t,
        Err(_) => return,
    };

    let mut client_clone = match client.try_clone() {
        Ok(c) => c,
        Err(_) => return,
    };
    let mut target_clone = match target.try_clone() {
        Ok(t) => t,
        Err(_) => return,
    };

    // Client -> Target
    let t1 = std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match client.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if target.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
            }
        }
        let _ = target.shutdown(std::net::Shutdown::Write);
    });

    // Target -> Client
    let t2 = std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match target_clone.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if client_clone.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
            }
        }
        let _ = client_clone.shutdown(std::net::Shutdown::Write);
    });

    let _ = t1.join();
    let _ = t2.join();
}
