//! HTTP/2-only endpoint registrations
//! Returns 421 on protocol mismatch

use orb_mockhttp::{TestServer, ResponseBuilder};
use httpbin_handlers as handlers;
use crate::adapter::{to_handler_request, to_orb_response};

/// Helper macro to reduce boilerplate for H2-only endpoints
macro_rules! h2_endpoint {
    ($server:expr, $path:expr, $method:expr, $handler:expr) => {
        $server.on_request($path)
            .expect_method($method)
            .respond_with_fn(|req| {
                if req.version() != http::Version::HTTP_2 {
                    return crate::adapter::misdirected_request();
                }
                let handler_req = to_handler_request(&req);
                let handler_resp = $handler(&handler_req);
                to_orb_response(handler_resp)
            });
    };
}

pub fn register_h2_mocks(server: &TestServer) {
    // ===== STATIC ENDPOINTS (no path parameters) =====

    h2_endpoint!(server, "/h2/bearer", "GET", handlers::auth::bearer_handler);
    h2_endpoint!(server, "/h2/brotli", "GET", handlers::compression::brotli_handler);
    h2_endpoint!(server, "/h2/cache", "GET", handlers::caching::cache_handler);
    h2_endpoint!(server, "/h2/cookies", "GET", handlers::cookies::cookies_handler);
    h2_endpoint!(server, "/h2/cookies/delete", "GET", handlers::cookies::cookies_delete_handler);
    h2_endpoint!(server, "/h2/cookies/set", "GET", handlers::cookies::cookies_set_handler);
    h2_endpoint!(server, "/h2/deflate", "GET", handlers::compression::deflate_handler);
    h2_endpoint!(server, "/h2/delete", "DELETE", handlers::http_methods::delete_handler);
    h2_endpoint!(server, "/h2/deny", "GET", handlers::response_formats::deny_handler);
    h2_endpoint!(server, "/h2/drip", "GET", handlers::streaming::drip_handler);
    h2_endpoint!(server, "/h2/encoding/utf8", "GET", handlers::response_formats::encoding_utf8_handler);
    h2_endpoint!(server, "/h2/get", "GET", handlers::http_methods::get_handler);
    h2_endpoint!(server, "/h2/gzip", "GET", handlers::compression::gzip_handler);
    h2_endpoint!(server, "/h2/headers", "GET", handlers::inspection::headers_handler);
    h2_endpoint!(server, "/h2/html", "GET", handlers::response_formats::html_handler);
    h2_endpoint!(server, "/h2/image", "GET", handlers::images::image_handler);
    h2_endpoint!(server, "/h2/image/jpeg", "GET", handlers::images::image_jpeg_handler);
    h2_endpoint!(server, "/h2/image/png", "GET", handlers::images::image_png_handler);
    h2_endpoint!(server, "/h2/image/svg", "GET", handlers::images::image_svg_handler);
    h2_endpoint!(server, "/h2/image/webp", "GET", handlers::images::image_webp_handler);
    h2_endpoint!(server, "/h2/ip", "GET", handlers::inspection::ip_handler);
    h2_endpoint!(server, "/h2/json", "GET", handlers::response_formats::json_handler);
    h2_endpoint!(server, "/h2/patch", "PATCH", handlers::http_methods::patch_handler);
    h2_endpoint!(server, "/h2/post", "POST", handlers::http_methods::post_handler);
    h2_endpoint!(server, "/h2/put", "PUT", handlers::http_methods::put_handler);
    h2_endpoint!(server, "/h2/redirect-to", "GET", handlers::redirect::redirect_to_handler);
    h2_endpoint!(server, "/h2/response-headers", "GET", handlers::forms::response_headers_handler);
    h2_endpoint!(server, "/h2/robots.txt", "GET", handlers::response_formats::robots_txt_handler);
    h2_endpoint!(server, "/h2/user-agent", "GET", handlers::inspection::user_agent_handler);
    h2_endpoint!(server, "/h2/uuid", "GET", handlers::inspection::uuid_handler);
    h2_endpoint!(server, "/h2/xml", "GET", handlers::response_formats::xml_handler);
    h2_endpoint!(server, "/h2/anything", "GET", handlers::anything::anything_handler);

    // ===== PARAMETERIZED ENDPOINTS =====

    // /h2/status/{code}
    for code in [100, 200, 201, 202, 204, 301, 302, 303, 304, 307, 308,
                 400, 401, 403, 404, 405, 406, 408, 409, 410, 418,
                 429, 500, 501, 502, 503, 504] {
        let path = format!("/h2/status/{}", code);
        h2_endpoint!(server, &path, "GET", handlers::status::status_handler);
    }

    // /h2/bytes/{n}
    for n in [1, 10, 100, 256, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/h2/bytes/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::streaming::bytes_handler);
    }

    // /h2/delay/{n}
    for n in [1, 2, 3, 5, 10] {
        let path = format!("/h2/delay/{}", n);
        server.on_request(&path)
            .expect_method("GET")
            .respond_with_fn(move |req| {
                if req.version() != http::Version::HTTP_2 {
                    return crate::adapter::misdirected_request();
                }
                ResponseBuilder::new()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(format!(r#"{{"delay": {}}}"#, n).into_bytes())
                    .build()
            });
    }

    // /h2/stream/{n}
    for n in [1, 5, 10, 20, 50, 100] {
        let path = format!("/h2/stream/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::streaming::stream_handler);
    }

    // /h2/stream-bytes/{n}
    for n in [100, 512, 1024, 2048, 4096, 8192] {
        let path = format!("/h2/stream-bytes/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::streaming::stream_bytes_handler);
    }

    // /h2/redirect/{n}
    for n in 1..=10 {
        let path = format!("/h2/redirect/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::redirect::redirect_handler);
    }

    // /h2/absolute-redirect/{n}
    for n in 1..=10 {
        let path = format!("/h2/absolute-redirect/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::redirect::absolute_redirect_handler);
    }

    // /h2/relative-redirect/{n}
    for n in 1..=10 {
        let path = format!("/h2/relative-redirect/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::redirect::relative_redirect_handler);
    }

    // /h2/cache/{n}
    for n in [10, 30, 60, 120, 300, 600, 3600] {
        let path = format!("/h2/cache/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::caching::cache_n_handler);
    }

    // /h2/range/{n}
    for n in [100, 256, 512, 1024, 2048, 4096] {
        let path = format!("/h2/range/{}", n);
        h2_endpoint!(server, &path, "GET", handlers::streaming::range_handler);
    }

    // /h2/links/{n}/{offset}
    for n in [5, 10, 20] {
        for offset in [0, 1, 5] {
            let path = format!("/h2/links/{}/{}", n, offset);
            h2_endpoint!(server, &path, "GET", handlers::streaming::links_handler);
        }
    }

    // /h2/base64/{value}
    for value in ["aGVsbG8=", "dGVzdA==", "aHR0cGJpbg==", "SGVsbG8gV29ybGQh"] {
        let path = format!("/h2/base64/{}", value);
        h2_endpoint!(server, &path, "GET", handlers::inspection::base64_handler);
    }

    // /h2/anything/{path}
    for path_seg in ["test", "foo", "bar", "hello", "api", "v1", "data"] {
        let path = format!("/h2/anything/{}", path_seg);
        h2_endpoint!(server, &path, "GET", handlers::anything::anything_handler);
    }

    // /h2/basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin"), ("test", "test")] {
        let path = format!("/h2/basic-auth/{}/{}", user, passwd);
        h2_endpoint!(server, &path, "GET", handlers::auth::basic_auth_handler);
    }

    // /h2/hidden-basic-auth/{user}/{passwd}
    for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
        let path = format!("/h2/hidden-basic-auth/{}/{}", user, passwd);
        h2_endpoint!(server, &path, "GET", handlers::auth::hidden_basic_auth_handler);
    }

    // /h2/digest-auth/{qop}/{user}/{passwd}
    for qop in ["auth", "auth-int"] {
        for (user, passwd) in [("user", "pass"), ("admin", "admin")] {
            let path = format!("/h2/digest-auth/{}/{}/{}", qop, user, passwd);
            h2_endpoint!(server, &path, "GET", handlers::auth::digest_auth_algorithm_handler);
        }
    }

    // /h2/cookies/set/{name}/{value}
    for (name, value) in [("session", "abc123"), ("foo", "bar"), ("test", "value")] {
        let path = format!("/h2/cookies/set/{}/{}", name, value);
        h2_endpoint!(server, &path, "GET", handlers::cookies::cookies_set_specific_handler);
    }

    // /h2/etag/{etag}
    for etag in ["test", "abc123", "etag1"] {
        let path = format!("/h2/etag/{}", etag);
        h2_endpoint!(server, &path, "GET", handlers::caching::etag_handler);
    }
}
