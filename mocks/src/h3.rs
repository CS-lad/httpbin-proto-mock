//! HTTP/3-only endpoint registrations
//! Returns 421 on protocol mismatch

use orb_mockhttp::{TestServer, ResponseBuilder};
use httpbin_handlers as handlers;
use crate::adapter::{to_handler_request, to_orb_response};

/// Helper macro to reduce boilerplate for H3-only endpoints
macro_rules! h3_endpoint {
    ($server:expr, $path:expr, $method:expr, $handler:expr) => {
        $server.on_request($path)
            .expect_method($method)
            .respond_with_fn(|req| {
                if req.version() != http::Version::HTTP_3 {
                    return crate::adapter::misdirected_request();
                }
                let handler_req = to_handler_request(&req);
                let handler_resp = $handler(&handler_req);
                to_orb_response(handler_resp)
            });
    };
}

pub fn register_h3_mocks(server: &TestServer) {
    // ===== STATIC ENDPOINTS (no path parameters) =====

    h3_endpoint!(server, "/h3/bearer", "GET", handlers::auth::bearer_handler);
    h3_endpoint!(server, "/h3/brotli", "GET", handlers::compression::brotli_handler);
    h3_endpoint!(server, "/h3/cache", "GET", handlers::caching::cache_handler);
    h3_endpoint!(server, "/h3/cookies", "GET", handlers::cookies::cookies_handler);
    h3_endpoint!(server, "/h3/cookies/delete", "GET", handlers::cookies::cookies_delete_handler);
    h3_endpoint!(server, "/h3/cookies/set", "GET", handlers::cookies::cookies_set_handler);
    h3_endpoint!(server, "/h3/deflate", "GET", handlers::compression::deflate_handler);
    h3_endpoint!(server, "/h3/delete", "DELETE", handlers::http_methods::delete_handler);
    h3_endpoint!(server, "/h3/deny", "GET", handlers::response_formats::deny_handler);
    h3_endpoint!(server, "/h3/drip", "GET", handlers::streaming::drip_handler);
    h3_endpoint!(server, "/h3/encoding/utf8", "GET", handlers::response_formats::encoding_utf8_handler);
    h3_endpoint!(server, "/h3/get", "GET", handlers::http_methods::get_handler);
    h3_endpoint!(server, "/h3/gzip", "GET", handlers::compression::gzip_handler);
    h3_endpoint!(server, "/h3/headers", "GET", handlers::inspection::headers_handler);
    h3_endpoint!(server, "/h3/html", "GET", handlers::response_formats::html_handler);
    h3_endpoint!(server, "/h3/image", "GET", handlers::images::image_handler);
    h3_endpoint!(server, "/h3/image/jpeg", "GET", handlers::images::image_jpeg_handler);
    h3_endpoint!(server, "/h3/image/png", "GET", handlers::images::image_png_handler);
    h3_endpoint!(server, "/h3/image/svg", "GET", handlers::images::image_svg_handler);
    h3_endpoint!(server, "/h3/image/webp", "GET", handlers::images::image_webp_handler);
    h3_endpoint!(server, "/h3/ip", "GET", handlers::inspection::ip_handler);
    h3_endpoint!(server, "/h3/json", "GET", handlers::response_formats::json_handler);
    h3_endpoint!(server, "/h3/patch", "PATCH", handlers::http_methods::patch_handler);
    h3_endpoint!(server, "/h3/post", "POST", handlers::http_methods::post_handler);
    h3_endpoint!(server, "/h3/put", "PUT", handlers::http_methods::put_handler);
    h3_endpoint!(server, "/h3/redirect-to", "GET", handlers::redirect::redirect_to_handler);
    h3_endpoint!(server, "/h3/response-headers", "GET", handlers::forms::response_headers_handler);
    h3_endpoint!(server, "/h3/robots.txt", "GET", handlers::response_formats::robots_txt_handler);
    h3_endpoint!(server, "/h3/user-agent", "GET", handlers::inspection::user_agent_handler);
    h3_endpoint!(server, "/h3/uuid", "GET", handlers::inspection::uuid_handler);
    h3_endpoint!(server, "/h3/xml", "GET", handlers::response_formats::xml_handler);
    h3_endpoint!(server, "/h3/anything", "GET", handlers::anything::anything_handler);

    // ===== PARAMETERIZED ENDPOINTS =====

    // /h3/status/{code}
    for code in [100, 200, 201, 202, 204, 301, 302, 303, 304, 307, 308,
                 400, 401, 403, 404, 405, 406, 408, 409, 410, 418,
                 429, 500, 501, 502, 503, 504] {
        let path = format!("/h3/status/{}", code);
        h3_endpoint!(server, &path, "GET", handlers::status::status_handler);
    }

    // /h3/bytes/{n}
    for n in [1, 10, 100, 256, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/h3/bytes/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::streaming::bytes_handler);
    }

    // /h3/delay/{n}
    for n in [1, 2, 3, 5, 10] {
        let path = format!("/h3/delay/{}", n);
        server.on_request(&path)
            .expect_method("GET")
            .respond_with_fn(move |req| {
                if req.version() != http::Version::HTTP_3 {
                    return crate::adapter::misdirected_request();
                }
                ResponseBuilder::new()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(format!(r#"{{"delay": {}}}"#, n).into_bytes())
                    .build()
            });
    }

    // /h3/stream/{n}
    for n in [1, 5, 10, 20, 50, 100] {
        let path = format!("/h3/stream/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::streaming::stream_handler);
    }

    // /h3/stream-bytes/{n}
    for n in [100, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/h3/stream-bytes/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::streaming::stream_bytes_handler);
    }

    // /h3/redirect/{n}
    for n in 1..=10 {
        let path = format!("/h3/redirect/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::redirect::redirect_handler);
    }

    // /h3/absolute-redirect/{n}
    for n in 1..=10 {
        let path = format!("/h3/absolute-redirect/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::redirect::absolute_redirect_handler);
    }

    // /h3/relative-redirect/{n}
    for n in 1..=10 {
        let path = format!("/h3/relative-redirect/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::redirect::relative_redirect_handler);
    }

    // /h3/cache/{n}
    for n in [10, 30, 60, 120, 300, 600, 3600] {
        let path = format!("/h3/cache/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::caching::cache_n_handler);
    }

    // /h3/range/{n}
    for n in [100, 256, 512, 1024, 2048, 4096] {
        let path = format!("/h3/range/{}", n);
        h3_endpoint!(server, &path, "GET", handlers::streaming::range_handler);
    }

    // /h3/links/{n}/{offset}
    for n in [5, 10, 20] {
        for offset in [0, 1, 5] {
            let path = format!("/h3/links/{}/{}", n, offset);
            h3_endpoint!(server, &path, "GET", handlers::streaming::links_handler);
        }
    }

    // /h3/base64/{value}
    for value in ["aGVsbG8=", "dGVzdA==", "aHR0cGJpbg==", "SGVsbG8gV29ybGQh"] {
        let path = format!("/h3/base64/{}", value);
        h3_endpoint!(server, &path, "GET", handlers::inspection::base64_handler);
    }

    // /h3/anything/{path}
    for path_seg in ["test", "foo", "bar", "hello", "api", "v1", "data"] {
        let path = format!("/h3/anything/{}", path_seg);
        h3_endpoint!(server, &path, "GET", handlers::anything::anything_handler);
    }

    // /h3/basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin"), ("test", "test")] {
        let path = format!("/h3/basic-auth/{}/{}", user, passwd);
        h3_endpoint!(server, &path, "GET", handlers::auth::basic_auth_handler);
    }

    // /h3/hidden-basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
        let path = format!("/h3/hidden-basic-auth/{}/{}", user, passwd);
        h3_endpoint!(server, &path, "GET", handlers::auth::hidden_basic_auth_handler);
    }

    // /h3/digest-auth/{qop}/{user}/{passwd}
    for qop in ["auth", "auth-int"] {
        for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
            let path = format!("/h3/digest-auth/{}/{}/{}", qop, user, passwd);
            h3_endpoint!(server, &path, "GET", handlers::auth::digest_auth_algorithm_handler);
        }
    }

    // /h3/cookies/set/{name}/{value}
    for (name, value) in [("session", "abc123"), ("foo", "bar"), ("test", "value")] {
        let path = format!("/h3/cookies/set/{}/{}", name, value);
        h3_endpoint!(server, &path, "GET", handlers::cookies::cookies_set_specific_handler);
    }

    // /h3/etag/{etag}
    for etag in ["test", "abc123", "etag1"] {
        let path = format!("/h3/etag/{}", etag);
        h3_endpoint!(server, &path, "GET", handlers::caching::etag_handler);
    }
}
