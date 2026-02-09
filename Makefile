.PHONY: run build clean quick

# rust-analyzer now uses target-ra/ so no more lock contention

# Quick run - skip build if binary exists and is newer than sources
quick:
	@./target/debug/httpbin-server

# Full run - always rebuild first
run: build
	./target/debug/httpbin-server

build:
	cargo build --bin httpbin-server

clean:
	cargo clean
