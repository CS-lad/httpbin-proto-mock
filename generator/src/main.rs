use anyhow::{Context, Result};
use serde_json::Value as JsonValue;
use std::fs;
use std::io::Write;

fn main() -> Result<()> {
    println!("Phase 3: Code Generator for orb-mockhttp");
    println!("Generating complete handler integration");
    println!();

    let spec_path = "openapi/httpbin-proto.yaml";
    
    if !std::path::Path::new(spec_path).exists() {
        eprintln!("Error: {} not found!", spec_path);
        std::process::exit(1);
    }

    println!("Reading {}...", spec_path);
    let yaml_str = fs::read_to_string(spec_path)?;
    let spec: JsonValue = serde_yaml::from_str(&yaml_str)?;

    println!("Analyzing endpoints...");
    let endpoints = analyze_endpoints(&spec)?;
    
    println!("  Protocol-agnostic: {}", endpoints.any.len());
    println!("  HTTP/1.1-only: {}", endpoints.h1.len());
    println!("  HTTP/2-only: {}", endpoints.h2.len());
    println!("  HTTP/3-only: {}", endpoints.h3.len());
    println!();

    println!("Generating orb-mockhttp registration code with handler integration...");
    generate_mock_files(&endpoints)?;
    
    println!();
    println!("Code generation complete!");
    println!("Generated files:");
    println!("  - mocks/src/lib.rs");
    println!("  - mocks/src/any.rs");
    println!("  - mocks/src/h1.rs");
    println!("  - mocks/src/h2.rs");
    println!("  - mocks/src/h3.rs");
    println!("  - mocks/src/adapter.rs (request/response conversion)");

    Ok(())
}

#[derive(Debug)]
struct Endpoints {
    any: Vec<EndpointInfo>,
    h1: Vec<EndpointInfo>,
    h2: Vec<EndpointInfo>,
    h3: Vec<EndpointInfo>,
}

#[derive(Debug, Clone)]
struct EndpointInfo {
    path: String,
    methods: Vec<String>,
    handler_name: String,
    _is_async: bool,
}

fn analyze_endpoints(spec: &JsonValue) -> Result<Endpoints> {
    let mut endpoints = Endpoints {
        any: Vec::new(),
        h1: Vec::new(),
        h2: Vec::new(),
        h3: Vec::new(),
    };

    let paths = spec["paths"].as_object().context("No paths")?;

    for (path, path_item) in paths {
        let methods: Vec<String> = path_item
            .as_object()
            .unwrap_or(&serde_json::Map::new())
            .keys()
            .filter(|k| ["get", "post", "put", "patch", "delete"].contains(&k.as_str()))
            .map(|s| s.to_uppercase())
            .collect();

        if methods.is_empty() {
            continue;
        }

        let (handler_name, is_async) = determine_handler_info(path);

        let info = EndpointInfo {
            path: path.clone(),
            methods,
            handler_name,
            _is_async: is_async,
        };

        if path.starts_with("/h1/") {
            endpoints.h1.push(info);
        } else if path.starts_with("/h2/") {
            endpoints.h2.push(info);
        } else if path.starts_with("/h3/") {
            endpoints.h3.push(info);
        } else {
            endpoints.any.push(info);
        }
    }

    Ok(endpoints)
}

fn determine_handler_info(path: &str) -> (String, bool) {
    let clean_path = path.trim_start_matches("/h1")
        .trim_start_matches("/h2")
        .trim_start_matches("/h3")
        .trim_start_matches('/');

    let base = clean_path.split('/').next().unwrap_or("unknown");
    
    let (handler, is_async) = match base {
        "delay" => ("delay::delay_handler", true),
        "status" => ("status::status_handler", false),
        "get" => ("http_methods::get_handler", false),
        "post" => ("http_methods::post_handler", false),
        "put" => ("http_methods::put_handler", false),
        "patch" => ("http_methods::patch_handler", false),
        "delete" => ("http_methods::delete_handler", false),
        "headers" => ("inspection::headers_handler", false),
        "user-agent" => ("inspection::user_agent_handler", false),
        "ip" => ("inspection::ip_handler", false),
        "uuid" => ("inspection::uuid_handler", false),
        "base64" => ("inspection::base64_handler", false),
        "redirect" => ("redirect::redirect_handler", false),
        "relative-redirect" => ("redirect::relative_redirect_handler", false),
        "absolute-redirect" => ("redirect::absolute_redirect_handler", false),
        "redirect-to" => ("redirect::redirect_to_handler", false),
        "cookies" => (determine_cookie_handler(path), false),
        "json" => ("response_formats::json_handler", false),
        "html" => ("response_formats::html_handler", false),
        "xml" => ("response_formats::xml_handler", false),
        "robots.txt" => ("response_formats::robots_txt_handler", false),
        "deny" => ("response_formats::deny_handler", false),
        "encoding" => ("response_formats::encoding_utf8_handler", false),
        "anything" => ("anything::anything_handler", false),
        "bytes" => ("streaming::bytes_handler", false),
        "stream-bytes" => ("streaming::stream_bytes_handler", false),
        "stream" => ("streaming::stream_handler", false),
        "drip" => ("streaming::drip_handler", false),
        "range" => ("streaming::range_handler", false),
        "links" => ("streaming::links_handler", false),
        "image" => (determine_image_handler(path), false),
        "gzip" => ("compression::gzip_handler", false),
        "deflate" => ("compression::deflate_handler", false),
        "brotli" => ("compression::brotli_handler", false),
        "cache" => (determine_cache_handler(path), false),
        "etag" => ("caching::etag_handler", false),
        "basic-auth" => ("auth::basic_auth_handler", false),
        "hidden-basic-auth" => ("auth::hidden_basic_auth_handler", false),
        "digest-auth" => (determine_digest_auth_handler(path), false),
        "bearer" => ("auth::bearer_handler", false),
        "forms" => ("forms::forms_post_handler", false),
        "response-headers" => ("forms::response_headers_handler", false),
        _ => ("unknown", false),
    };
    
    (handler.to_string(), is_async)
}

fn determine_cookie_handler(path: &str) -> &'static str {
    if path.contains("/cookies/set/") {
        "cookies::cookies_set_specific_handler"
    } else if path.contains("/cookies/set") {
        "cookies::cookies_set_handler"
    } else if path.contains("/cookies/delete") {
        "cookies::cookies_delete_handler"
    } else {
        "cookies::cookies_handler"
    }
}

fn determine_image_handler(path: &str) -> &'static str {
    if path.contains("/png") {
        "images::image_png_handler"
    } else if path.contains("/jpeg") {
        "images::image_jpeg_handler"
    } else if path.contains("/webp") {
        "images::image_webp_handler"
    } else if path.contains("/svg") {
        "images::image_svg_handler"
    } else {
        "images::image_handler"
    }
}

fn determine_cache_handler(path: &str) -> &'static str {
    if path.contains("/cache/") {
        "caching::cache_n_handler"
    } else {
        "caching::cache_handler"
    }
}

fn determine_digest_auth_handler(path: &str) -> &'static str {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() > 5 {
        "auth::digest_auth_algorithm_handler"
    } else {
        "auth::digest_auth_handler"
    }
}

fn generate_mock_files(endpoints: &Endpoints) -> Result<()> {
    fs::create_dir_all("mocks/src")?;
    
    generate_lib_rs()?;
    generate_adapter_rs()?;
    generate_any_rs(&endpoints.any)?;
    generate_h1_rs(&endpoints.h1)?;
    generate_h2_rs(&endpoints.h2)?;
    generate_h3_rs(&endpoints.h3)?;
    generate_mocks_cargo_toml()?;
    
    Ok(())
}

fn generate_lib_rs() -> Result<()> {
    let mut file = fs::File::create("mocks/src/lib.rs")?;
    
    writeln!(file, "//! Mock endpoint registration for httpbin-proto-mock")?;
    writeln!(file, "//! Using orb-mockhttp with full handler integration")?;
    writeln!(file)?;
    writeln!(file, "mod adapter;")?;
    writeln!(file, "pub mod any;")?;
    writeln!(file, "pub mod h1;")?;
    writeln!(file, "pub mod h2;")?;
    writeln!(file, "pub mod h3;")?;
    writeln!(file)?;
    writeln!(file, "pub use any::register_any_protocol_mocks;")?;
    writeln!(file, "pub use h1::register_h1_mocks;")?;
    writeln!(file, "pub use h2::register_h2_mocks;")?;
    writeln!(file, "pub use h3::register_h3_mocks;")?;
    
    Ok(())
}

fn generate_adapter_rs() -> Result<()> {
    let mut file = fs::File::create("mocks/src/adapter.rs")?;
    
    writeln!(file, "//! Adapter to convert between orb-mockhttp and httpbin-handlers types")?;
    writeln!(file)?;
    writeln!(file, "use orb_mockhttp::{{Request as OrbRequest, Response as OrbResponse, ResponseBuilder}};")?;
    writeln!(file, "use httpbin_handlers::{{Request, Response}};")?;
    writeln!(file, "use std::collections::HashMap;")?;
    writeln!(file)?;
    writeln!(file, "/// Convert orb-mockhttp Request to our Request type")?;
    writeln!(file, "pub fn to_handler_request(orb_req: &OrbRequest) -> Request {{")?;
    writeln!(file, "    let mut headers = HashMap::new();")?;
    writeln!(file)?;
    writeln!(file, "    // Convert headers")?;
    writeln!(file, "    for (name, value) in orb_req.headers() {{")?;
    writeln!(file, "        if let Ok(val_str) = value.to_str() {{")?;
    writeln!(file, "            headers.insert(name.to_string(), val_str.to_string());")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file)?;
    writeln!(file, "    Request {{")?;
    writeln!(file, "        method: orb_req.method().to_string(),")?;
    writeln!(file, "        path: orb_req.uri().path().to_string(),")?;
    writeln!(file, "        headers,")?;
    writeln!(file, "        http_version: format!(\"{{:?}}\", orb_req.version()),")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "/// Convert our Response to orb-mockhttp Response")?;
    writeln!(file, "pub fn to_orb_response(handler_resp: Response) -> OrbResponse {{")?;
    writeln!(file, "    let mut builder = ResponseBuilder::new(handler_resp.status);")?;
    writeln!(file)?;
    writeln!(file, "    // Add headers")?;
    writeln!(file, "    for (name, value) in handler_resp.headers {{")?;
    writeln!(file, "        builder = builder.header(&name, &value);")?;
    writeln!(file, "    }}")?;
    writeln!(file)?;
    writeln!(file, "    // Add body")?;
    writeln!(file, "    builder = builder.body(handler_resp.body);")?;
    writeln!(file)?;
    writeln!(file, "    builder.build()")?;
    writeln!(file, "}}")?;
    
    Ok(())
}

fn generate_any_rs(endpoints: &[EndpointInfo]) -> Result<()> {
    let mut file = fs::File::create("mocks/src/any.rs")?;
    
    writeln!(file, "//! Protocol-agnostic endpoint registrations")?;
    writeln!(file, "//! These endpoints work on any HTTP protocol version")?;
    writeln!(file)?;
    writeln!(file, "use orb_mockhttp::TestServer;")?;
    writeln!(file, "use httpbin_handlers as handlers;")?;
    writeln!(file, "use crate::adapter::{{to_handler_request, to_orb_response}};")?;
    writeln!(file)?;
    writeln!(file, "pub fn register_any_protocol_mocks(server: &TestServer) {{")?;
    
    for endpoint in endpoints {
        let method = endpoint.methods.first().map(|s| s.as_str()).unwrap_or("GET");
        let path_pattern = convert_path_to_pattern(&endpoint.path);
        
        writeln!(file)?;
        writeln!(file, "    // {} -> handlers::{}", endpoint.path, endpoint.handler_name)?;
        writeln!(file, "    server.on_request(\"{}\")", path_pattern)?;
        writeln!(file, "        .expect_method(\"{}\")", method)?;
        writeln!(file, "        .respond_with_fn(|req| {{")?;
        writeln!(file, "            let handler_req = to_handler_request(&req);")?;
        writeln!(file, "            let handler_resp = handlers::{}(&handler_req);", endpoint.handler_name)?;
        writeln!(file, "            to_orb_response(handler_resp)")?;
        writeln!(file, "        }});")?;
    }
    
    writeln!(file, "}}")?;
    
    Ok(())
}

fn generate_h1_rs(endpoints: &[EndpointInfo]) -> Result<()> {
    let mut file = fs::File::create("mocks/src/h1.rs")?;
    
    writeln!(file, "//! HTTP/1.1-only endpoint registrations")?;
    writeln!(file, "//! Returns 421 on protocol mismatch")?;
    writeln!(file)?;
    writeln!(file, "use orb_mockhttp::{{TestServer, ResponseBuilder}};")?;
    writeln!(file, "use httpbin_handlers as handlers;")?;
    writeln!(file, "use crate::adapter::{{to_handler_request, to_orb_response}};")?;
    writeln!(file)?;
    writeln!(file, "pub fn register_h1_mocks(server: &TestServer) {{")?;
    
    for endpoint in endpoints {
        let method = endpoint.methods.first().map(|s| s.as_str()).unwrap_or("GET");
        let path_pattern = convert_path_to_pattern(&endpoint.path);
        
        writeln!(file)?;
        writeln!(file, "    // {} -> handlers::{}", endpoint.path, endpoint.handler_name)?;
        writeln!(file, "    server.on_request(\"{}\")", path_pattern)?;
        writeln!(file, "        .expect_method(\"{}\")", method)?;
        writeln!(file, "        .respond_with_fn(|req| {{")?;
        writeln!(file, "            // Check if HTTP/1.1")?;
        writeln!(file, "            if req.version() != http::Version::HTTP_11 {{")?;
        writeln!(file, "                return ResponseBuilder::new(421).build();")?;
        writeln!(file, "            }}")?;
        writeln!(file, "            let handler_req = to_handler_request(&req);")?;
        writeln!(file, "            let handler_resp = handlers::{}(&handler_req);", endpoint.handler_name)?;
        writeln!(file, "            to_orb_response(handler_resp)")?;
        writeln!(file, "        }});")?;
    }
    
    writeln!(file, "}}")?;
    
    Ok(())
}

fn generate_h2_rs(endpoints: &[EndpointInfo]) -> Result<()> {
    let mut file = fs::File::create("mocks/src/h2.rs")?;
    
    writeln!(file, "//! HTTP/2-only endpoint registrations")?;
    writeln!(file, "//! Returns 421 on protocol mismatch")?;
    writeln!(file)?;
    writeln!(file, "use orb_mockhttp::{{TestServer, ResponseBuilder}};")?;
    writeln!(file, "use httpbin_handlers as handlers;")?;
    writeln!(file, "use crate::adapter::{{to_handler_request, to_orb_response}};")?;
    writeln!(file)?;
    writeln!(file, "pub fn register_h2_mocks(server: &TestServer) {{")?;
    
    for endpoint in endpoints {
        let method = endpoint.methods.first().map(|s| s.as_str()).unwrap_or("GET");
        let path_pattern = convert_path_to_pattern(&endpoint.path);
        
        writeln!(file)?;
        writeln!(file, "    // {} -> handlers::{}", endpoint.path, endpoint.handler_name)?;
        writeln!(file, "    server.on_request(\"{}\")", path_pattern)?;
        writeln!(file, "        .expect_method(\"{}\")", method)?;
        writeln!(file, "        .respond_with_fn(|req| {{")?;
        writeln!(file, "            // Check if HTTP/2")?;
        writeln!(file, "            if req.version() != http::Version::HTTP_2 {{")?;
        writeln!(file, "                return ResponseBuilder::new(421).build();")?;
        writeln!(file, "            }}")?;
        writeln!(file, "            let handler_req = to_handler_request(&req);")?;
        writeln!(file, "            let handler_resp = handlers::{}(&handler_req);", endpoint.handler_name)?;
        writeln!(file, "            to_orb_response(handler_resp)")?;
        writeln!(file, "        }});")?;
    }
    
    writeln!(file, "}}")?;
    
    Ok(())
}

fn generate_h3_rs(endpoints: &[EndpointInfo]) -> Result<()> {
    let mut file = fs::File::create("mocks/src/h3.rs")?;
    
    writeln!(file, "//! HTTP/3-only endpoint registrations")?;
    writeln!(file, "//! Returns 421 on protocol mismatch")?;
    writeln!(file)?;
    writeln!(file, "use orb_mockhttp::{{TestServer, ResponseBuilder}};")?;
    writeln!(file, "use httpbin_handlers as handlers;")?;
    writeln!(file, "use crate::adapter::{{to_handler_request, to_orb_response}};")?;
    writeln!(file)?;
    writeln!(file, "pub fn register_h3_mocks(server: &TestServer) {{")?;
    
    for endpoint in endpoints {
        let method = endpoint.methods.first().map(|s| s.as_str()).unwrap_or("GET");
        let path_pattern = convert_path_to_pattern(&endpoint.path);
        
        writeln!(file)?;
        writeln!(file, "    // {} -> handlers::{}", endpoint.path, endpoint.handler_name)?;
        writeln!(file, "    server.on_request(\"{}\")", path_pattern)?;
        writeln!(file, "        .expect_method(\"{}\")", method)?;
        writeln!(file, "        .respond_with_fn(|req| {{")?;
        writeln!(file, "            // Check if HTTP/3")?;
        writeln!(file, "            if req.version() != http::Version::HTTP_3 {{")?;
        writeln!(file, "                return ResponseBuilder::new(421).build();")?;
        writeln!(file, "            }}")?;
        writeln!(file, "            let handler_req = to_handler_request(&req);")?;
        writeln!(file, "            let handler_resp = handlers::{}(&handler_req);", endpoint.handler_name)?;
        writeln!(file, "            to_orb_response(handler_resp)")?;
        writeln!(file, "        }});")?;
    }
    
    writeln!(file, "}}")?;
    
    Ok(())
}

fn convert_path_to_pattern(path: &str) -> String {
    path.replace("{codes}", ":codes")
        .replace("{code}", ":code")
        .replace("{n}", ":n")
        .replace("{delay}", ":delay")
        .replace("{value}", ":value")
        .replace("{user}", ":user")
        .replace("{passwd}", ":passwd")
        .replace("{qop}", ":qop")
        .replace("{algorithm}", ":algorithm")
        .replace("{etag}", ":etag")
        .replace("{name}", ":name")
        .replace("{anything}", ":anything")
        .replace("{format}", ":format")
}

fn generate_mocks_cargo_toml() -> Result<()> {
    let mut file = fs::File::create("mocks/Cargo.toml")?;
    
    writeln!(file, "[package]")?;
    writeln!(file, "name = \"httpbin-mocks\"")?;
    writeln!(file, "version = \"0.1.0\"")?;
    writeln!(file, "edition = \"2021\"")?;
    writeln!(file)?;
    writeln!(file, "[dependencies]")?;
    writeln!(file, "httpbin-handlers = {{ path = \"../handlers\" }}")?;
    writeln!(file, "orb-mockhttp = \"0.1.0\"")?;
    writeln!(file, "http = \"1.0\"  # For http::Version")?;
    
    Ok(())
}
