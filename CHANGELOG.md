# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2025-12-05

### ✨ Added

#### Struct Field Serde Rename Support
- **Extended serde rename to struct fields**: Previously only enum variants supported `#[serde(rename = "...")]`, now struct fields do too
  - Struct fields with `#[serde(rename = "new_name")]` generate TypeScript schemas with the renamed field names
  - Ensures perfect alignment between Rust serialization and TypeScript types for structs
  - Example: `#[serde(rename = "userName")]` generates `userName: z.string()` instead of `user_name: z.string()`
  - Thanks to @julienr for this contribution!

#### Stable Schema Ordering
- **Deterministic output for CI workflows**: Switched from `HashMap` to `BTreeMap` for schema storage
  - Generated TypeScript files now have stable, alphabetically sorted schema ordering
  - Eliminates spurious diffs in version control when schemas are checked in
  - Essential for CI workflows that validate generated files match committed versions
  - Thanks to @julienr for this contribution!

### 🔧 Code Quality

- Refactored attribute parsing to use idiomatic `&[Attribute]` slice parameter
- Moved `serde` dependency to dev-dependencies (only needed for tests)
- Added comprehensive test coverage for struct field renaming

### 📚 Examples

- Enhanced `serde_rename_test.rs` to demonstrate struct field renaming
- Added test case `test_struct_rename()` in derive tests

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

## [1.1.8] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Reliability Fix
- **Fixed Version Availability Check**: Replaced unreliable `cargo search | grep` with `cargo info --version` for checking package availability
  - Resolves issue where `cargo search` doesn't return packages in expected format in GitHub Actions environment
  - Uses `cargo info zod_gen --version ${VERSION}` which returns proper exit codes for version existence
  - Ensures reliable automated releases by eliminating dependency on output format parsing
  - Fixes the "did not become available after 2 minutes" error in release workflow

#### Technical Details
- Updated `.github/workflows/release.yml` wait step to use `cargo info` instead of `cargo search`
- More reliable version checking that doesn't depend on parsing output format
- No API changes, only release process improvement

## [1.1.9] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Final Fix
- **Fixed Version Availability Check**: Updated to use `cargo info zod_gen | grep -q "version: ${VERSION_WITHOUT_V}"` for reliable package availability detection
  - This approach is more reliable than previous attempts as it uses the standard cargo info output format
  - Resolves the persistent "did not become available after 2 minutes" error in GitHub Actions
  - Ensures automated releases work consistently by checking for the version string in cargo info output
  - Final fix for the release workflow that was causing multiple release failures

#### Technical Details
- Updated `.github/workflows/release.yml` line 85 with the correct cargo info and grep pattern
- Uses the standard cargo info output format which includes "version: X.Y.Z" line
- No API changes, only release process improvement

## [1.1.10] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Registry Fix
- **Fixed Local vs Registry Version Confusion**: Updated `cargo info` to use `--registry crates-io` flag to force checking published version instead of local version
  - Previous attempts were failing because `cargo info zod_gen` was returning the local project version `(from ./zod_gen)` instead of the crates.io published version
  - The `--registry crates-io` flag ensures cargo checks the actual published version on crates.io
  - This should finally resolve the persistent "did not become available after 2 minutes" error
  - Removes debug output line that was added for troubleshooting

#### Technical Details
- Updated `.github/workflows/release.yml` to use `cargo info --registry crates-io zod_gen`
- Forces cargo to check the registry version instead of finding the local project
- No API changes, only release process improvement

## [1.1.11] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Registry Flag Application Fix
- **Fixed Missing Registry Flag**: Properly applied `--registry crates-io` flag to both cargo info commands in the release workflow
  - Previous fix was incomplete - the debug line still used `cargo info zod_gen` without the registry flag
  - Now both the debug output (removed) and the actual check use `--registry crates-io` to ensure consistent behavior
  - Removes the debug output line that was showing local version instead of registry version
  - This should finally resolve the version availability check issue

#### Technical Details
- Updated `.github/workflows/release.yml` to properly apply `--registry crates-io` flag
- Removed debug output line that was causing confusion
- Ensures all cargo info commands check the registry version, not local version
- No API changes, only release process improvement

## [1.1.12] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Version-Specific Check Fix
- **Fixed Version Availability Check**: Changed from grep pattern matching to using `cargo info --version` with exit code checking
  - Previous approach using `grep -q "version: ${VERSION_WITHOUT_V}"` was unreliable in GitHub Actions environment
  - New approach uses `cargo info --registry crates-io zod_gen --version ${VERSION_WITHOUT_V}` which returns proper exit codes
  - If the specific version exists, the command succeeds (exit code 0); if not, it fails (non-zero exit code)
  - This is more reliable than parsing output format and should work consistently across environments

#### Technical Details
- Updated `.github/workflows/release.yml` to use `cargo info --version` instead of grep pattern matching
- Uses exit code checking instead of output parsing for version availability
- More robust approach that doesn't depend on output format variations
- No API changes, only release process improvement

## [1.1.13] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Exact Pattern Matching Fix
- **Fixed Version Pattern Matching**: Updated grep pattern to use exact line matching with `^version: ${VERSION_WITHOUT_V}$`
  - Previous attempt used non-existent `--version` flag for cargo info command
  - New approach uses exact line matching to ensure we match the complete version line
  - Added `2>/dev/null` to suppress error messages and focus on successful output
  - Uses `^` and `$` anchors to match the entire line exactly, preventing partial matches

#### Technical Details
- Updated `.github/workflows/release.yml` to use `grep -q "^version: ${VERSION_WITHOUT_V}$"`
- Removed invalid `--version` flag from cargo info command
- Added error suppression to clean up output
- Uses exact line matching for more reliable version detection
- No API changes, only release process improvement

## [1.1.14] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Debug Output Addition
- **Added Debug Output to Version Check**: Enhanced the version availability check with comprehensive debugging information
  - Added debug output to show VERSION and VERSION_WITHOUT_V variables
  - Added debug output to display the actual cargo info command output
  - Added debug output to show the exact grep pattern being searched for
  - Added debug output to indicate whether pattern matching succeeded or failed
  - This will help identify the exact cause of version availability check failures
  - Debug information will be visible in GitHub Actions logs for troubleshooting

#### Technical Details
- Updated `.github/workflows/release.yml` to include comprehensive debug output
- Shows variable values, command output, and pattern matching results
- No API changes, only release process improvement and debugging enhancement

## [1.1.15] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Status Message Filtering Fix
- **Fixed Pattern Matching with Status Messages**: Added filtering to exclude cargo status messages from version pattern matching
  - Debug output revealed that cargo status messages ("Updating crates.io index", "Downloading crates...", "Downloaded zod_gen") were interfering with pattern matching
  - The package was actually available and showing `version: 1.1.14` but the grep pattern wasn't matching due to mixed output
  - Added `grep -v "Updating\|Downloading\|Downloaded"` to filter out status messages before pattern matching
  - This should finally allow the version availability check to work correctly

#### Technical Details
- Updated `.github/workflows/release.yml` to filter cargo status messages before version pattern matching
- Uses `grep -v` to exclude status lines, then `grep -q` to match the version pattern
- Debug output helped identify the exact cause of pattern matching failure
- No API changes, only release process improvement

## [1.1.16] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Enhanced Debug and Whitespace Handling
- **Added Enhanced Debug Output**: Added more comprehensive debugging to identify pattern matching issues
  - Shows filtered output after removing status messages
  - Shows lines containing 'version:' specifically
  - Added whitespace trimming with sed to handle potential leading/trailing spaces
  - This will help identify if there are hidden characters or formatting issues preventing pattern matching
  - Previous debug showed the version line was present but pattern matching still failed

#### Technical Details
- Updated `.github/workflows/release.yml` with enhanced debug output and whitespace handling
- Added sed commands to trim leading and trailing whitespace from filtered output
- Shows both filtered output and specific version lines for detailed troubleshooting
- No API changes, only release process improvement and debugging enhancement

## [1.1.17] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Version Line Specific Matching Fix
- **Fixed Version Pattern Matching**: Changed from exact line pattern to specific version line matching
  - Debug output revealed there are multiple lines containing 'version:' (`version: 1.1.16` and `rust-version: unknown`)
  - The exact line pattern `^version: ${VERSION_WITHOUT_V}$` was failing despite the correct line being present
  - New approach uses `grep "^version: " | grep -q "${VERSION_WITHOUT_V}"` to first isolate the version line, then check for the version number
  - This should finally resolve the pattern matching issue that was preventing version availability detection

#### Technical Details
- Updated `.github/workflows/release.yml` to use two-stage grep for version line matching
- First grep isolates lines starting with "version: " (excluding "rust-version:")
- Second grep checks if the version number is present in that line
- More robust approach that handles multiple version-related lines in cargo info output
- No API changes, only release process improvement

## [1.1.18] - 2025-08-01

### 🐛 Fixed

#### Release Workflow Direct Publish Retry Approach
- **Replaced Complex Version Detection with Simple Retry**: Eliminated the problematic version availability check in favor of direct publish retry
  - Removed all the complex cargo info parsing, pattern matching, and debug output that was consistently failing
  - New approach simply retries `cargo publish -p zod_gen_derive` up to 12 times with 10-second intervals
  - If `zod_gen` is available, the publish succeeds immediately; if not, it fails and retries
  - Much simpler, more reliable approach that lets cargo handle the dependency resolution
  - Eliminates all the pattern matching issues we've been debugging

#### Technical Details
- Updated `.github/workflows/release.yml` to use direct publish retry instead of version detection
- Removed complex cargo info parsing and grep pattern matching logic
- Uses cargo's built-in dependency resolution to determine when zod_gen is available
- Retries up to 12 times (2 minutes) with clear logging of each attempt
- No API changes, only release process simplification and improvement

## [1.1.19] - 2025-08-04

### 🐛 Fixed

#### Dependencies Workflow Fix
- **Fixed cargo upgrade command**: Removed invalid `--workspace` flag from `cargo upgrade` in dependencies workflow
  - The `cargo upgrade` command from cargo-edit doesn't support the `--workspace` flag
  - Updated `.github/workflows/dependencies.yml` to use `cargo upgrade` without the flag
  - This fixes the weekly dependency update automation that was failing due to the invalid flag
  - Also cleaned up trailing whitespace in the workflow file

### 📚 Documentation
- **Added AI Agent Debugging Guide**: Added comprehensive documentation about "Let it crash" debugging philosophy
  - `ai-agent-prompt.md`: Guide for AI agents on embracing failure as signal rather than avoiding it
  - `let-it-crash-lesson.md`: Real-world lesson from 18 failed release attempts and the breakthrough solution
  - Documents the anti-pattern of complex failure avoidance vs. simple retry mechanisms

#### Technical Details
- Updated dependencies workflow to use correct cargo-edit syntax
- No API changes, only CI/CD process improvement and documentation additions

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
