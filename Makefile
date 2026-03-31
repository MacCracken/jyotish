.PHONY: check fmt clippy test audit deny bench coverage build doc clean

# All features except `orbital` (requires local `falak` crate).
# Use `--all-features` locally if you have falak checked out.
CI_FEATURES = meeus,logging

check: fmt clippy test audit

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --features $(CI_FEATURES) --all-targets -- -D warnings

test:
	cargo test --features $(CI_FEATURES)
	cargo test --no-default-features

audit:
	cargo audit

deny:
	cargo deny check

bench:
	./scripts/bench-history.sh

coverage:
	cargo llvm-cov --features $(CI_FEATURES) --html --output-dir coverage/

build:
	cargo build --release --features $(CI_FEATURES)

doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --features $(CI_FEATURES)

clean:
	cargo clean
