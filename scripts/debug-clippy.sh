#!/bin/bash
# Debug clippy differences between local and CI

set -e

echo "🔍 Debugging Clippy Differences"
echo "================================"
echo ""

echo "📋 Environment Info:"
echo "Rust version: $(rustc --version)"
echo "Clippy version: $(cargo clippy --version)"
echo "Platform: $(uname -a)"
echo "PWD: $(pwd)"
echo ""

echo "📁 Workspace Info:"
echo "Git status:"
git status --porcelain
echo ""
echo "Git branch: $(git branch --show-current)"
echo "Git commit: $(git rev-parse HEAD)"
echo ""

echo "📦 Cargo Info:"
echo "Workspace members:"
cargo metadata --format-version 1 | jq -r '.workspace_members[]' 2>/dev/null || echo "jq not available"
echo ""

echo "🧹 Clean Build:"
cargo clean
echo ""

echo "🔍 Running Clippy with Maximum Verbosity:"
echo "Command: cargo clippy --workspace --all-targets --all-features --verbose -- -D warnings"
echo ""

# Set environment variables that CI might use
export CARGO_TERM_COLOR=always
export RUSTFLAGS="-D warnings"

# Run with maximum verbosity
cargo clippy --workspace --all-targets --all-features --verbose -- -D warnings

echo ""
echo "✅ Local clippy completed successfully!"
echo ""
echo "If CI is still failing, please share:"
echo "1. The exact error message from CI"
echo "2. Which files/lines are causing issues"
echo "3. The CI log output"