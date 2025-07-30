# Development Guide

## 🔧 Local Development Setup

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# The rust-toolchain.toml file will automatically install the correct version
cd zod_gen
cargo --version  # Should match CI version
```

## 🧪 Testing

### Run All Tests
```bash
cargo test --workspace
```

### Run Examples
```bash
cargo run --example basic_usage
cargo run --example derive_example
cargo run --example generator_example
cargo run --example serde_rename_test
```

## 🔍 Code Quality

### Clippy (CI-Equivalent)
```bash
# Use this script to match CI exactly
./scripts/clippy-ci.sh

# Or run manually with same flags as CI
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Formatting
```bash
cargo fmt --all
```

### Documentation
```bash
cargo doc --workspace --no-deps --document-private-items
```

## 🚀 Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Update version in `README.md`
4. Run full test suite: `./scripts/clippy-ci.sh && cargo test`
5. Commit: `git commit -m "Release vX.Y.Z"`
6. Tag: `git tag vX.Y.Z`
7. Push: `git push origin main --tags`

## 🔄 CI/CD Consistency

### Why Different Results?

Common causes of local vs CI differences:
- **Rust version differences** (solved by `rust-toolchain.toml`)
- **Different clippy flags** (solved by `scripts/clippy-ci.sh`)
- **Platform differences** (Linux CI vs local OS)
- **Feature flag differences** (CI uses `--all-features`)

### Ensuring Consistency

1. **Use the toolchain file**: `rust-toolchain.toml` pins the Rust version
2. **Use the clippy script**: `./scripts/clippy-ci.sh` matches CI flags exactly
3. **Clean builds**: `cargo clean` before important checks
4. **Same features**: Always use `--all-features` for consistency

### Debugging Differences

If you still get different results:

```bash
# Check Rust version
rustc --version
cargo --version
cargo clippy --version

# Clean build
cargo clean

# Run with exact CI flags
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check for platform-specific issues
cargo clippy --target x86_64-unknown-linux-gnu --workspace --all-targets --all-features -- -D warnings
```

## 🔒 Pre-commit Hooks

This repository includes a native Git pre-commit hook that automatically runs before each commit to ensure code quality. The hook performs the following checks:

1. **Code Formatting** - Runs `cargo fmt --all -- --check`
2. **Clippy Linting** - Runs `cargo clippy` with CI-equivalent flags
3. **Tests** - Runs the full test suite
4. **Examples** - Verifies that examples still work

### Automatic Setup

The pre-commit hook is automatically installed in `.git/hooks/pre-commit` and is ready to use. No additional setup required!

### What the Hook Checks

```bash
# Formatting check
cargo fmt --all -- --check

# Clippy with strict flags (same as CI)
cargo clippy --all-targets --all-features -- -D warnings -D clippy::uninlined-format-args

# Full test suite
cargo test

# Example verification
cargo run --example basic_usage
cargo run --example serde_rename_test
```

### If Pre-commit Fails

The hook will prevent commits when issues are found:

**Formatting Issues:**
```bash
# Fix automatically
cargo fmt --all
```

**Clippy Issues:**
```bash
# Run clippy to see issues
cargo clippy --all-targets --all-features -- -D warnings -D clippy::uninlined-format-args

# Fix issues manually, then retry commit
```

**Test Failures:**
```bash
# Run tests to see failures
cargo test

# Fix failing tests, then retry commit
```

### Bypassing the Hook (Not Recommended)

In rare cases where you need to bypass the hook:

```bash
git commit --no-verify -m "Your commit message"
```

**Note:** This should only be used in exceptional circumstances, as it defeats the purpose of maintaining code quality.

## 📝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `./scripts/clippy-ci.sh` to ensure CI compatibility
5. Run `cargo test --workspace` to ensure tests pass
6. Submit a pull request

The CI will run the same checks, so if your local checks pass, CI should too!