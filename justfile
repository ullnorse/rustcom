# Detect CI environment and set appropriate flags
is_ci := env_var_or_default("CI", "false")
clippy_flags := if is_ci == "true" { "--all-targets --all-features -- -D warnings" } else { "--all-targets --all-features" }
ci_mode := if is_ci == "true" { "CI" } else { "local" }
ci_release_flags := if is_ci == "true" { "--release" } else { "" }

# Show available commands
default:
    @just --list

# Run the application
run *ARGS:
    @cargo run {{ci_release_flags}} -- {{ARGS}}

# Build in debug mode
build:
    @cargo build

# Build in release mode
release:
    @cargo build --release

# Run all tests
test:
    @cargo test {{ci_release_flags}}

# Format code
fmt:
    @cargo fmt --all

# Check if code is formatted (doesn't modify files)
fmt-check:
    @cargo fmt --all -- --check

# Run clippy linter (warnings in local, errors in CI)
clippy:
    @echo "Running clippy in {{ci_mode}} mode"
    @cargo clippy {{ci_release_flags}} {{clippy_flags}}

# Run complete CI pipeline (auto-detects CI environment)
ci: fmt-check release test clippy
    @echo "All CI checks passed"

# Force strict CI mode locally (simulates what runs in GitHub Actions)
ci-strict:
    @just is_ci=true ci

# Clean build artifacts
clean:
    @cargo clean

# Update dependencies
update:
    @cargo update

# Check for security vulnerabilities (requires cargo-audit)
audit:
    @cargo audit

# Install recommended development tools
setup:
    @echo "Installing development tools"
    @cargo install cargo-audit
    @echo "Setup complete"