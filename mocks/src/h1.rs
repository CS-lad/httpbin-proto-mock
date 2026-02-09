//! HTTP/1.1-only endpoint registrations
//! Returns 421 on protocol mismatch

use orb_mockhttp::{TestServer, ResponseBuilder};
use httpbin_handlers as handlers;
use crate::adapter::{to_handler_request, to_orb_response};

/// Helper macro to reduce boilerplate for H1-only endpoints
macro_rules! h1_endpoint {
    ($server:expr, $path:expr, $method:expr, $handler:expr) => {
        $server.on_request($path)
            .expect_method($method)
            .respond_with_fn(|req| {
                if req.version() != http::Version::HTTP_11 {
                    return crate::adapter::misdirected_request();
                }
                let handler_req = to_handler_request(&req);
                let handler_resp = $handler(&handler_req);
                to_orb_response(handler_resp)
            });
    };
}

pub fn register_h1_mocks(server: &TestServer) {
    // ===== STATIC ENDPOINTS (no path parameters) =====

    h1_endpoint!(server, "/h1/bearer", "GET", handlers::auth::bearer_handler);
    h1_endpoint!(server, "/h1/brotli", "GET", handlers::compression::brotli_handler);
    h1_endpoint!(server, "/h1/cache", "GET", handlers::caching::cache_handler);
    h1_endpoint!(server, "/h1/cookies", "GET", handlers::cookies::cookies_handler);
    h1_endpoint!(server, "/h1/cookies/delete", "GET", handlers::cookies::cookies_delete_handler);
    h1_endpoint!(server, "/h1/cookies/set", "GET", handlers::cookies::cookies_set_handler);
    h1_endpoint!(server, "/h1/deflate", "GET", handlers::compression::deflate_handler);
    h1_endpoint!(server, "/h1/delete", "DELETE", handlers::http_methods::delete_handler);
    h1_endpoint!(server, "/h1/deny", "GET", handlers::response_formats::deny_handler);
    h1_endpoint!(server, "/h1/drip", "GET", handlers::streaming::drip_handler);
    h1_endpoint!(server, "/h1/encoding/utf8", "GET", handlers::response_formats::encoding_utf8_handler);
    h1_endpoint!(server, "/h1/get", "GET", handlers::http_methods::get_handler);
    h1_endpoint!(server, "/h1/gzip", "GET", handlers::compression::gzip_handler);
    h1_endpoint!(server, "/h1/headers", "GET", handlers::inspection::headers_handler);
    h1_endpoint!(server, "/h1/html", "GET", handlers::response_formats::html_handler);
    h1_endpoint!(server, "/h1/image", "GET", handlers::images::image_handler);
    h1_endpoint!(server, "/h1/image/jpeg", "GET", handlers::images::image_jpeg_handler);
    h1_endpoint!(server, "/h1/image/png", "GET", handlers::images::image_png_handler);
    h1_endpoint!(server, "/h1/image/svg", "GET", handlers::images::image_svg_handler);
    h1_endpoint!(server, "/h1/image/webp", "GET", handlers::images::image_webp_handler);
    h1_endpoint!(server, "/h1/ip", "GET", handlers::inspection::ip_handler);
    h1_endpoint!(server, "/h1/json", "GET", handlers::response_formats::json_handler);
    h1_endpoint!(server, "/h1/patch", "PATCH", handlers::http_methods::patch_handler);
    h1_endpoint!(server, "/h1/post", "POST", handlers::http_methods::post_handler);
    h1_endpoint!(server, "/h1/put", "PUT", handlers::http_methods::put_handler);
    h1_endpoint!(server, "/h1/redirect-to", "GET", handlers::redirect::redirect_to_handler);
    h1_endpoint!(server, "/h1/response-headers", "GET", handlers::forms::response_headers_handler);
    h1_endpoint!(server, "/h1/robots.txt", "GET", handlers::response_formats::robots_txt_handler);
    h1_endpoint!(server, "/h1/user-agent", "GET", handlers::inspection::user_agent_handler);
    h1_endpoint!(server, "/h1/uuid", "GET", handlers::inspection::uuid_handler);
    h1_endpoint!(server, "/h1/xml", "GET", handlers::response_formats::xml_handler);
    h1_endpoint!(server, "/h1/anything", "GET", handlers::anything::anything_handler);

    // ===== PARAMETERIZED ENDPOINTS =====
    // orb-mockhttp doesn't support path parameters, so we register common values

    // /h1/status/{code} - HTTP status codes
    for code in [100, 200, 201, 202, 204, 301, 302, 303, 304, 307, 308,
                 400, 401, 403, 404, 405, 406, 408, 409, 410, 418,
                 429, 500, 501, 502, 503, 504] {
        let path = format!("/h1/status/{}", code);
        h1_endpoint!(server, &path, "GET", handlers::status::status_handler);
    }

    // /h1/bytes/{n} - random bytes
    for n in [1, 10, 100, 256, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/h1/bytes/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::streaming::bytes_handler);
    }

    // /h1/delay/{n} - delayed response (seconds)
    // Note: delay_handler is async, so we use a sync placeholder
    for n in [1, 2, 3, 5, 10] {
        let path = format!("/h1/delay/{}", n);
        server.on_request(&path)
            .expect_method("GET")
            .respond_with_fn(move |req| {
                if req.version() != http::Version::HTTP_11 {
                    return crate::adapter::misdirected_request();
                }
                // Async handler - returning placeholder (actual delay not supported in sync context)
                ResponseBuilder::new()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(format!(r#"{{"delay": {}}}"#, n).into_bytes())
                    .build()
            });
    }

    // /h1/stream/{n} - streaming n lines
    for n in [1, 5, 10, 20, 50, 100] {
        let path = format!("/h1/stream/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::streaming::stream_handler);
    }

    // /h1/stream-bytes/{n} - streaming bytes
    for n in [100, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/h1/stream-bytes/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::streaming::stream_bytes_handler);
    }

    // /h1/redirect/{n} - redirect n times
    for n in 1..=10 {
        let path = format!("/h1/redirect/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::redirect::redirect_handler);
    }

    // /h1/absolute-redirect/{n}
    for n in 1..=10 {
        let path = format!("/h1/absolute-redirect/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::redirect::absolute_redirect_handler);
    }

    // /h1/relative-redirect/{n}
    for n in 1..=10 {
        let path = format!("/h1/relative-redirect/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::redirect::relative_redirect_handler);
    }

    // /h1/cache/{n} - cache for n seconds
    for n in [10, 30, 60, 120, 300, 600, 3600] {
        let path = format!("/h1/cache/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::caching::cache_n_handler);
    }

    // /h1/range/{n} - range request for n bytes
    for n in [100, 256, 512, 1024, 2048, 4096] {
        let path = format!("/h1/range/{}", n);
        h1_endpoint!(server, &path, "GET", handlers::streaming::range_handler);
    }

    // /h1/links/{n}/{offset} - generate n links starting at offset
    for n in [5, 10, 20] {
        for offset in [0, 1, 5] {
            let path = format!("/h1/links/{}/{}", n, offset);
            h1_endpoint!(server, &path, "GET", handlers::streaming::links_handler);
        }
    }

    // /h1/base64/{value} - decode base64
    // Common test values: "hello" = "aGVsbG8=", "test" = "dGVzdA==", "httpbin" = "aHR0cGJpbg=="
    for value in ["aGVsbG8=", "dGVzdA==", "aHR0cGJpbg==", "SGVsbG8gV29ybGQh"] {
        let path = format!("/h1/base64/{}", value);
        h1_endpoint!(server, &path, "GET", handlers::inspection::base64_handler);
    }

    // /h1/anything/{path} - echo anything
    for path_seg in ["test", "foo", "bar", "hello", "api", "v1", "data"] {
        let path = format!("/h1/anything/{}", path_seg);
        h1_endpoint!(server, &path, "GET", handlers::anything::anything_handler);
    }

    // /h1/basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin"), ("test", "test")] {
        let path = format!("/h1/basic-auth/{}/{}", user, passwd);
        h1_endpoint!(server, &path, "GET", handlers::auth::basic_auth_handler);
    }

    // /h1/hidden-basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
        let path = format!("/h1/hidden-basic-auth/{}/{}", user, passwd);
        h1_endpoint!(server, &path, "GET", handlers::auth::hidden_basic_auth_handler);
    }

    // /h1/digest-auth/{qop}/{user}/{passwd}
    for qop in ["auth", "auth-int"] {
        for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
            let path = format!("/h1/digest-auth/{}/{}/{}", qop, user, passwd);
            h1_endpoint!(server, &path, "GET", handlers::auth::digest_auth_algorithm_handler);
        }
    }

    // /h1/cookies/set/{name}/{value}
    for (name, value) in [("session", "abc123"), ("foo", "bar"), ("test", "value")] {
        let path = format!("/h1/cookies/set/{}/{}", name, value);
        h1_endpoint!(server, &path, "GET", handlers::cookies::cookies_set_specific_handler);
    }

    // /h1/etag/{etag}
    for etag in ["test", "abc123", "etag1"] {
        let path = format!("/h1/etag/{}", etag);
        h1_endpoint!(server, &path, "GET", handlers::caching::etag_handler);
    }
}
