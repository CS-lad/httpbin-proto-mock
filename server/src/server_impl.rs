use anyhow::Result;
use orb_mockhttp::MockServer;

pub async fn run() -> Result<()> {
    println!("Initializing mock server...");
    println!();
    
    let mut server = MockServer::new().await;
    
    println!("Registering protocol-agnostic mocks (52 endpoints)...");
    httpbin_mocks::any::register_any_protocol_mocks(&mut server).await;
    
    println!("Registering HTTP/1.1 mocks (104 registrations)...");
    httpbin_mocks::h1::register_h1_mocks(&mut server).await;
    
    println!("Registering HTTP/2 mocks (104 registrations)...");
    httpbin_mocks::h2::register_h2_mocks(&mut server).await;
    
    println!("Registering HTTP/3 mocks (104 registrations)...");
    httpbin_mocks::h3::register_h3_mocks(&mut server).await;
    
    println!();
    println!("All 364 mocks registered successfully!");
    println!();
    println!("Starting server on 0.0.0.0:8080...");
    
    server.start("0.0.0.0:8080").await?;
    
    Ok(())
}
