# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.4] - 2025-07-30

### 📚 Documentation Improvements

#### Enhanced
- **Comprehensive Crate Documentation**: Added detailed module documentation prominently featuring the derive macro
- **TypeScript Output Examples**: Show exactly what gets generated from Rust types
- **Installation Guide**: Clear instructions for using both `zod_gen` and `zod_gen_derive` together
- **Usage Examples**: Both derive macro (recommended) and manual implementation approaches
- **Crates.io Description**: Updated to mention `zod_gen_derive` for better discoverability

This release makes it much clearer to new users that `zod_gen_derive` is the recommended way to use the library, addressing feedback about the crates.io page.

## [1.1.3] - 2025-07-30

### 🔧 Development Experience Improvements

This release focuses on improving the development experience and ensuring consistency between local development and CI environments.

#### Added
- **Native Git Pre-commit Hook**: Automatically runs rustfmt, clippy, tests, and example verification before each commit
- **Rust Toolchain Pinning**: Added `rust-toolchain.toml` to ensure consistent Rust version across environments
- **CI Consistency Scripts**: 
  - `scripts/clippy-ci.sh` - Run clippy with exact CI flags locally
  - `scripts/debug-clippy.sh` - Troubleshoot clippy differences between local and CI
- **Development Documentation**: Comprehensive `DEVELOPMENT.md` with setup and troubleshooting guides
- **Hook Setup Script**: `scripts/setup-hooks.sh` for verifying pre-commit hook installation

#### Changed
- **API Cleanup**: Simplified `zod_gen/src/lib.rs` by removing unused wrapper methods
- **Format String Compliance**: Updated format strings in `zod_gen_derive` to comply with `clippy::uninlined-format-args`
- **Enhanced README**: Added development tooling section with links to new documentation

#### Fixed
- **CI Consistency**: Resolved local vs CI clippy differences that could cause unexpected CI failures
- **Code Quality**: Ensured all code passes strict clippy lints matching CI configuration

This release ensures developers can't accidentally commit code that will fail CI, significantly improving the development workflow.

## [1.1.0] - 2025-07-30

### ✨ New Features

#### Added
- **Serde Rename Support**: Full support for `#[serde(rename = "...")]` attributes on enum variants
  - TypeScript schemas now use the renamed values instead of Rust variant names
  - Provides compile-time type safety between Rust serialization and TypeScript
  - Example: `#[serde(rename = "active")]` generates `z.literal("active")` instead of `z.literal("Active")`

#### Enhanced
- **Comprehensive Examples**: Added `serde_rename_test.rs` demonstrating rename functionality
- **Better Documentation**: Updated README with serde rename examples and use cases

This feature closes the gap between Rust serialization and TypeScript type checking, ensuring developers can't accidentally use incorrect string literals.

## [1.0.0] - 2025-07-29

### 🎉 Initial Stable Release

This is the first stable release of `zod_gen` with a clean, simplified API.

### ✨ Features

- **ZodSchema Trait**: Simple trait with only `zod_schema()` method
- **Derive Macro**: `#[derive(ZodSchema)]` for automatic schema generation
- **ZodGenerator**: Single-file TypeScript output with user-controlled naming
- **Generic Type Support**: Built-in support for `Option<T>`, `Vec<T>`, `HashMap<String, T>`
- **Inline Schemas**: Self-contained schemas with no external dependencies
- **TypeScript Integration**: Automatic type inference using `z.infer<typeof Schema>`

### 🏗️ Supported Types

#### Primitives
- `String`, `i32`, `i64`, `u32`, `u64`, `f32`, `f64`, `bool`
- `serde_json::Value`

#### Generics
- `Option<T>` → `T.nullable()`
- `Vec<T>` → `z.array(T)`
- `HashMap<String, T>` → `z.record(z.string(), T)`

#### Custom Types
- Structs with named fields → `z.object({ ... })`
- Enums with unit variants → `z.union([z.literal('A'), z.literal('B')])`

### 📚 API

```rust
// Manual implementation
impl ZodSchema for MyType {
    fn zod_schema() -> String {
        // Return Zod schema string
    }
}

// Derive macro
#[derive(ZodSchema)]
struct User {
    id: u64,
    name: String,
}

// Generator
let mut gen = ZodGenerator::new();
gen.add_schema::<User>("User");
let typescript = gen.generate();
```

### 🎯 Design Principles

- **Simplicity**: Minimal API surface with maximum functionality
- **User Control**: Users provide TypeScript type names explicitly
- **Single Responsibility**: Library only generates Zod schemas
- **Zero Magic**: Predictable behavior with no hidden complexity
- **TypeScript First**: Designed for seamless Zod integration

---

## [1.1.0] - 2025-07-30

### ✨ Added

- **Serde Rename Support**: Automatic handling of `#[serde(rename = "...")]` attributes on enum variants
  - Enum variants with serde rename generate TypeScript literal types using the renamed values
  - Ensures perfect alignment between Rust serialization and TypeScript types
  - Provides compile-time type safety to catch serialization mismatches

### 🎯 Type Safety Improvements

- TypeScript now catches typos when using enum values (e.g., `'Active'` vs `'active'`)
- Generated schemas use serde-renamed values for runtime validation
- Maintains backward compatibility for enums without serde rename

### 📚 Examples

```rust
#[derive(ZodSchema, Serialize, Deserialize)]
enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}
```

Generates:
```typescript
export const StatusSchema = z.union([z.literal('active'), z.literal('inactive')]);
export type Status = z.infer<typeof StatusSchema>;
```

---

## [1.1.7] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Bug Fix
- **Fixed Version Display in Release Workflow**: Corrected double `v` prefix in logging messages
  - Changed `v${VERSION}` to `${VERSION}` since `github.ref_name` already includes `v` prefix
  - Prevents confusion in release logs and ensures proper version tracking
  - This fixes the "vv1.1.6" issue seen in release logs

#### Technical Details
- Updated `.github/workflows/release.yml` wait step logging
- Removed duplicate `v` prefix from version display messages
- No API changes, only release process improvement

## [1.1.6] - 2025-08-01

### 🚀 Improved

#### Release Process Enhancements
- **Fixed GitHub Actions Release Workflow**: Resolved dependency order issue in automated releases
  - Updated dry run validation to only check `zod_gen` since `zod_gen_derive` depends on unpublished version
  - Replaced fixed 30-second wait with intelligent polling that checks crates.io availability
  - Fixed version pattern matching to handle `v1.1.6` vs `1.1.6` format differences
  - Ensures reliable automated releases for future versions

#### Technical Details
- Modified `.github/workflows/release.yml` to handle package dependencies correctly
- Added intelligent version availability checking with retry logic
- Improved error handling and logging in release workflow
- This change only affects the release process, not the library API

## [1.1.5] - 2025-08-01

### 🐛 Fixed

#### Critical Bug Fix
- **Parcel Bundler Compatibility**: Fixed import statement generation to use `import * as z from 'zod';` instead of `import { z } from 'zod';`
  - Resolves runtime error `i.z.string is not a function` when bundled with Parcel
  - Ensures Zod validation works correctly in Parcel-based applications
  - Maintains full backward compatibility with existing functionality

#### Technical Details
- Updated `ZodGenerator::generate()` method in `zod_gen/src/lib.rs`
- Updated all documentation examples to use the correct import format
- Updated tests to verify the new import statement
- This change affects generated TypeScript files but maintains API compatibility

## [Unreleased]

### 🤖 Added
- **GitHub Actions CI/CD**: Comprehensive automation for testing, releases, and maintenance
  - `ci.yml`: Full test suite on multiple Rust versions, formatting, clippy, docs, security audit, coverage
  - `release.yml`: Automated publishing to crates.io when tags are pushed
  - `pr.yml`: PR validation with breaking change detection, commit message validation, CHANGELOG checks
  - `dependencies.yml`: Weekly dependency updates and security audits
  - `docs.yml`: Automatic documentation deployment to GitHub Pages

### 🔧 Infrastructure
- **Automated Testing**: Tests run on stable, beta, and nightly Rust
- **Code Quality**: Automated formatting, clippy lints, and documentation checks
- **Security**: Weekly security audits with automatic issue creation
- **Release Automation**: Tag-triggered releases with automatic crates.io publishing
- **Documentation**: Auto-deployed docs at GitHub Pages

---

## Release Process

### Automated Release (Recommended)

1. **Update Version**: Bump version in `Cargo.toml` workspace
2. **Update CHANGELOG**: Document all changes in this file
3. **Update README**: Update version numbers in installation instructions
4. **Commit**: `git add . && git commit -m "Release vX.Y.Z"`
5. **Tag**: `git tag vX.Y.Z`
6. **Push**: `git push origin main --tags`
7. **Automated**: GitHub Actions will automatically:
   - Run full test suite
   - Publish to crates.io
   - Create GitHub release with changelog

### Manual Release (Fallback)

To release a new version manually:

1. **Update Version**: Bump version in `Cargo.toml` workspace
2. **Update CHANGELOG**: Document all changes in this file
3. **Update README**: Update version numbers in installation instructions
4. **Test**: Run `cargo test` to ensure all tests pass
5. **Commit**: `git add . && git commit -m "Release vX.Y.Z"`
6. **Tag**: `git tag vX.Y.Z`
7. **Push**: `git push origin main --tags`
8. **Publish**: `cargo publish -p zod_gen_derive && cargo publish -p zod_gen`

### Version Guidelines

- **Major (X.0.0)**: Breaking API changes
- **Minor (X.Y.0)**: New features, backward compatible
- **Patch (X.Y.Z)**: Bug fixes, backward compatible

[1.1.0]: https://github.com/cimatic/zod_gen/releases/tag/v1.1.0
[1.0.0]: https://github.com/cimatic/zod_gen/releases/tag/v1.0.0
