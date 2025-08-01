# AGENTS Guide for zod_gen

## Development Commands (see DEVELOPMENT.md)
- `cargo test --workspace` (all tests)
- `./scripts/clippy-ci.sh` (CI-equivalent linting)
- `cargo fmt --all` (format code)
- `cargo run --example <name>` (test examples)

## Build & Release
- `cargo build` (debug)
- `cargo build --release`

## Lint & Format
- `cargo fmt -- --check` (format check)
- `cargo clippy -- -D warnings` (lint)

## Test
- `cargo test` (full suite)
- `cargo test <test_name>` (single test)

## Release Process (see CHANGELOG.md)
1. Update version in `Cargo.toml` workspace
2. Update `CHANGELOG.md` with changes
3. Update version in `README.md` 
4. Run checks: `./scripts/clippy-ci.sh && cargo test`
5. Commit: `git commit -m "Release vX.Y.Z"`
6. Tag: `git tag vX.Y.Z`
7. Push: `git push origin main --tags`

## Code Style
- rustfmt (Rust 2021) enforced
- Import order: std ➜ external ➜ internal
- Naming: `snake_case` funcs/vars/modules, `PascalCase` types/traits, `SCREAMING_SNAKE_CASE` consts
- Errors: return `Result<T, E>`, use `?`, prefer `thiserror`; avoid `unwrap`/panics
