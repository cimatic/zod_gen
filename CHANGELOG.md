# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

## Release Process

To release a new version:

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

[1.0.0]: https://github.com/cimatic/zod_gen/releases/tag/v1.0.0