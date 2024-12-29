default:
	just --list

# Generate documentation for default feature set.
docs *EXTRA:
	RUSTDOCFLAGS='--cfg=docsrs' cargo +nightly doc -p barse {{EXTRA}}

# Generate documentation for all features.
docs-all *EXTRA:
	RUSTDOCFLAGS='--cfg=docsrs' cargo +nightly doc --all-features -p barse {{EXTRA}}

# Generate documentation for minimal feature set.
docs-min *EXTRA:
	cargo doc --no-default-features -p barse {{EXTRA}}

# Run tests with all features.
test:
	cargo test --all-features

# Run integration tests.
test-integration:
	cargo test -p tests

# Format crates.
fmt:
	cargo fmt --all
