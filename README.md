# httpbin-proto-mock

Protocol-aware HTTP mock server based on httpbin, built with Rust and orb-mockhttp.

## What This Is

An HTTP mock server that extends httpbin's 52 endpoints into 208 protocol-aware endpoints:

- **52 protocol-agnostic** endpoints (`/`) — work on any HTTP version
- **52 HTTP/1.1-only** endpoints (`/h1/`) — return 421 Misdirected Request on other protocols
- **52 HTTP/2-only** endpoints (`/h2/`) — return 421 Misdirected Request on other protocols
- **52 HTTP/3-only** endpoints (`/h3/`) — return 421 Misdirected Request on other protocols

## Project Structure

```
httpbin-proto-mock/
├── Cargo.toml              # Workspace config
├── Makefile                # Build/run commands
├── openapi/
│   ├── httpbin-spec.json   # Original httpbin spec (52 endpoints)
│   └── httpbin-proto.yaml  # Generated protocol-aware spec (208 endpoints)
├── generator/              # Code generator (JSON → YAML, mock registration)
├── handlers/               # Endpoint handler implementations
├── mocks/                  # orb-mockhttp endpoint registrations
│   └── src/
│       ├── any.rs          # Protocol-agnostic endpoints
│       ├── h1.rs           # HTTP/1.1-only endpoints
│       ├── h2.rs           # HTTP/2-only endpoints
│       ├── h3.rs           # HTTP/3-only endpoints
│       └── adapter.rs      # Request/response conversion layer
└── server/                 # Server binary
```

## Quick Start

### Build and run
```bash
make run
```

### Run without rebuilding (if binary already exists)
```bash
make quick
```

### Clean build artifacts
```bash
make clean
```

The server starts on a random port and prints the URL:
```
Server ready at: https://127.0.0.1:<PORT>/
```

## Testing Endpoints

### Protocol-agnostic (works with any HTTP version)
```bash
curl -k https://127.0.0.1:<PORT>/get
curl -k https://127.0.0.1:<PORT>/status/200
curl -k https://127.0.0.1:<PORT>/uuid
curl -k -X POST https://127.0.0.1:<PORT>/post
```

### HTTP/1.1-only (use --http1.1)
```bash
curl -k --http1.1 https://127.0.0.1:<PORT>/h1/get          # 200 OK
curl -k --http1.1 https://127.0.0.1:<PORT>/h1/status/200    # 200 OK
curl -k https://127.0.0.1:<PORT>/h1/get                     # 421 Misdirected Request
```

### HTTP/2-only (curl defaults to HTTP/2)
```bash
curl -k https://127.0.0.1:<PORT>/h2/get                     # 200 OK
curl -k https://127.0.0.1:<PORT>/h2/status/200               # 200 OK
curl -k --http1.1 https://127.0.0.1:<PORT>/h2/get            # 421 Misdirected Request
```

### HTTP/3-only (requires HTTP/3-enabled curl)
```bash
curl -k --http3 https://127.0.0.1:<PORT>/h3/get              # 200 OK
curl -k https://127.0.0.1:<PORT>/h3/get                      # 421 Misdirected Request
```

## Available Endpoints

| Category | Endpoints |
|----------|-----------|
| HTTP Methods | `/get`, `/post`, `/put`, `/patch`, `/delete` |
| Status Codes | `/status/{code}` (100-504) |
| Auth | `/basic-auth/{user}/{pass}`, `/bearer`, `/digest-auth/{qop}/{user}/{pass}` |
| Response Formats | `/json`, `/html`, `/xml`, `/deny`, `/robots.txt`, `/encoding/utf8` |
| Inspection | `/ip`, `/headers`, `/user-agent`, `/uuid` |
| Compression | `/gzip`, `/deflate`, `/brotli` |
| Cookies | `/cookies`, `/cookies/set`, `/cookies/set/{name}/{value}`, `/cookies/delete` |
| Redirects | `/redirect/{n}`, `/absolute-redirect/{n}`, `/relative-redirect/{n}`, `/redirect-to` |
| Streaming | `/bytes/{n}`, `/stream/{n}`, `/stream-bytes/{n}`, `/range/{n}`, `/drip` |
| Delays | `/delay/{n}` |
| Caching | `/cache`, `/cache/{n}`, `/etag/{value}` |
| Images | `/image`, `/image/png`, `/image/jpeg`, `/image/svg`, `/image/webp` |
| Other | `/anything`, `/anything/{path}`, `/base64/{value}`, `/links/{n}/{offset}` |

All endpoints are available under `/`, `/h1/`, `/h2/`, and `/h3/` prefixes.

## Technology

- **Language:** Rust
- **Mock Server:** orb-mockhttp
- **TLS:** Built-in HTTPS support
- **Protocols:** HTTP/1.1, HTTP/2, HTTP/3 (QUIC)

## License

MIT
