#!/bin/bash
# Run clippy with the same flags as CI
# This ensures consistency between local and CI runs

set -e

echo "🔍 Running clippy with CI-equivalent flags..."
echo "This matches the GitHub Actions clippy job exactly."
echo ""

# Clean build to avoid cache issues
echo "🧹 Cleaning build cache..."
cargo clean

# Check if we should run strict mode
if [[ "$1" == "--strict" ]]; then
    echo "🔥 Running in STRICT mode (pedantic + nursery lints)"
    echo "Command: cargo clippy --workspace --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings"
    echo ""
    cargo clippy --workspace --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings
else
    echo "📋 Running in STANDARD mode (same as CI)"
    echo "Command: cargo clippy --workspace --all-targets --all-features -- -D warnings"
    echo ""
    # Set environment variables that CI uses
    export CARGO_TERM_COLOR=always
    cargo clippy --workspace --all-targets --all-features -- -D warnings
fi

echo ""
echo "✅ Clippy completed successfully!"
echo "This should match the CI results exactly."
echo ""
echo "💡 Tips:"
echo "  - Use './scripts/clippy-ci.sh --strict' to run with pedantic lints"
echo "  - This script runs 'cargo clean' first to avoid cache issues"
echo "  - CI may use a different clippy version - check rust-toolchain.toml"