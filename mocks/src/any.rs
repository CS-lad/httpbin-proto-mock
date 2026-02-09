//! Protocol-agnostic endpoint registrations
//! These endpoints work on any HTTP protocol version

use orb_mockhttp::{TestServer, ResponseBuilder};
use httpbin_handlers as handlers;
use crate::adapter::{to_handler_request, to_orb_response};

/// Helper macro to reduce boilerplate for protocol-agnostic endpoints
macro_rules! any_endpoint {
    ($server:expr, $path:expr, $method:expr, $handler:expr) => {
        $server.on_request($path)
            .expect_method($method)
            .respond_with_fn(|req| {
                let handler_req = to_handler_request(&req);
                let handler_resp = $handler(&handler_req);
                to_orb_response(handler_resp)
            });
    };
}

pub fn register_any_protocol_mocks(server: &TestServer) {
    // ===== STATIC ENDPOINTS (no path parameters) =====

    any_endpoint!(server, "/bearer", "GET", handlers::auth::bearer_handler);
    any_endpoint!(server, "/brotli", "GET", handlers::compression::brotli_handler);
    any_endpoint!(server, "/cache", "GET", handlers::caching::cache_handler);
    any_endpoint!(server, "/cookies", "GET", handlers::cookies::cookies_handler);
    any_endpoint!(server, "/cookies/delete", "GET", handlers::cookies::cookies_delete_handler);
    any_endpoint!(server, "/cookies/set", "GET", handlers::cookies::cookies_set_handler);
    any_endpoint!(server, "/deflate", "GET", handlers::compression::deflate_handler);
    any_endpoint!(server, "/delete", "DELETE", handlers::http_methods::delete_handler);
    any_endpoint!(server, "/deny", "GET", handlers::response_formats::deny_handler);
    any_endpoint!(server, "/drip", "GET", handlers::streaming::drip_handler);
    any_endpoint!(server, "/encoding/utf8", "GET", handlers::response_formats::encoding_utf8_handler);
    any_endpoint!(server, "/get", "GET", handlers::http_methods::get_handler);
    any_endpoint!(server, "/gzip", "GET", handlers::compression::gzip_handler);
    any_endpoint!(server, "/headers", "GET", handlers::inspection::headers_handler);
    any_endpoint!(server, "/html", "GET", handlers::response_formats::html_handler);
    any_endpoint!(server, "/image", "GET", handlers::images::image_handler);
    any_endpoint!(server, "/image/jpeg", "GET", handlers::images::image_jpeg_handler);
    any_endpoint!(server, "/image/png", "GET", handlers::images::image_png_handler);
    any_endpoint!(server, "/image/svg", "GET", handlers::images::image_svg_handler);
    any_endpoint!(server, "/image/webp", "GET", handlers::images::image_webp_handler);
    any_endpoint!(server, "/ip", "GET", handlers::inspection::ip_handler);
    any_endpoint!(server, "/json", "GET", handlers::response_formats::json_handler);
    any_endpoint!(server, "/patch", "PATCH", handlers::http_methods::patch_handler);
    any_endpoint!(server, "/post", "POST", handlers::http_methods::post_handler);
    any_endpoint!(server, "/put", "PUT", handlers::http_methods::put_handler);
    any_endpoint!(server, "/redirect-to", "GET", handlers::redirect::redirect_to_handler);
    any_endpoint!(server, "/response-headers", "GET", handlers::forms::response_headers_handler);
    any_endpoint!(server, "/robots.txt", "GET", handlers::response_formats::robots_txt_handler);
    any_endpoint!(server, "/user-agent", "GET", handlers::inspection::user_agent_handler);
    any_endpoint!(server, "/uuid", "GET", handlers::inspection::uuid_handler);
    any_endpoint!(server, "/xml", "GET", handlers::response_formats::xml_handler);
    any_endpoint!(server, "/anything", "GET", handlers::anything::anything_handler);

    // ===== PARAMETERIZED ENDPOINTS =====

    // /status/{code}
    for code in [100, 200, 201, 202, 204, 301, 302, 303, 304, 307, 308,
                 400, 401, 403, 404, 405, 406, 408, 409, 410, 418,
                 429, 500, 501, 502, 503, 504] {
        let path = format!("/status/{}", code);
        any_endpoint!(server, &path, "GET", handlers::status::status_handler);
    }

    // /bytes/{n}
    for n in [1, 10, 100, 256, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/bytes/{}", n);
        any_endpoint!(server, &path, "GET", handlers::streaming::bytes_handler);
    }

    // /delay/{n}
    for n in [1, 2, 3, 5, 10] {
        let path = format!("/delay/{}", n);
        server.on_request(&path)
            .expect_method("GET")
            .respond_with_fn(move |_req| {
                ResponseBuilder::new()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(format!(r#"{{"delay": {}}}"#, n).into_bytes())
                    .build()
            });
    }

    // /stream/{n}
    for n in [1, 5, 10, 20, 50, 100] {
        let path = format!("/stream/{}", n);
        any_endpoint!(server, &path, "GET", handlers::streaming::stream_handler);
    }

    // /stream-bytes/{n}
    for n in [100, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/stream-bytes/{}", n);
        any_endpoint!(server, &path, "GET", handlers::streaming::stream_bytes_handler);
    }

    // /redirect/{n}
    for n in 1..=10 {
        let path = format!("/redirect/{}", n);
        any_endpoint!(server, &path, "GET", handlers::redirect::redirect_handler);
    }

    // /absolute-redirect/{n}
    for n in 1..=10 {
        let path = format!("/absolute-redirect/{}", n);
        any_endpoint!(server, &path, "GET", handlers::redirect::absolute_redirect_handler);
    }

    // /relative-redirect/{n}
    for n in 1..=10 {
        let path = format!("/relative-redirect/{}", n);
        any_endpoint!(server, &path, "GET", handlers::redirect::relative_redirect_handler);
    }

    // /cache/{n}
    for n in [10, 30, 60, 120, 300, 600, 3600] {
        let path = format!("/cache/{}", n);
        any_endpoint!(server, &path, "GET", handlers::caching::cache_n_handler);
    }

    // /range/{n}
    for n in [100, 256, 512, 1024, 2048, 4096] {
        let path = format!("/range/{}", n);
        any_endpoint!(server, &path, "GET", handlers::streaming::range_handler);
    }

    // /links/{n}/{offset}
    for n in [5, 10, 20] {
        for offset in [0, 1, 5] {
            let path = format!("/links/{}/{}", n, offset);
            any_endpoint!(server, &path, "GET", handlers::streaming::links_handler);
        }
    }

    // /base64/{value}
    for value in ["aGVsbG8=", "dGVzdA==", "aHR0cGJpbg==", "SGVsbG8gV29ybGQh"] {
        let path = format!("/base64/{}", value);
        any_endpoint!(server, &path, "GET", handlers::inspection::base64_handler);
    }

    // /anything/{path}
    for path_seg in ["test", "foo", "bar", "hello", "api", "v1", "data"] {
        let path = format!("/anything/{}", path_seg);
        any_endpoint!(server, &path, "GET", handlers::anything::anything_handler);
    }

    // /basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin"), ("test", "test")] {
        let path = format!("/basic-auth/{}/{}", user, passwd);
        any_endpoint!(server, &path, "GET", handlers::auth::basic_auth_handler);
    }

    // /hidden-basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
        let path = format!("/hidden-basic-auth/{}/{}", user, passwd);
        any_endpoint!(server, &path, "GET", handlers::auth::hidden_basic_auth_handler);
    }

    // /digest-auth/{qop}/{user}/{passwd}
    for qop in ["auth", "auth-int"] {
        for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
            let path = format!("/digest-auth/{}/{}/{}", qop, user, passwd);
            any_endpoint!(server, &path, "GET", handlers::auth::digest_auth_algorithm_handler);
        }
    }

    // /cookies/set/{name}/{value}
    for (name, value) in [("session", "abc123"), ("foo", "bar"), ("test", "value")] {
        let path = format!("/cookies/set/{}/{}", name, value);
        any_endpoint!(server, &path, "GET", handlers::cookies::cookies_set_specific_handler);
    }

    // /etag/{etag}
    for etag in ["test", "abc123", "etag1"] {
        let path = format!("/etag/{}", etag);
        any_endpoint!(server, &path, "GET", handlers::caching::etag_handler);
    }
}
