# zod_gen

[![Crates.io](https://img.shields.io/crates/v/zod_gen.svg)](https://crates.io/crates/zod_gen)
[![Documentation](https://docs.rs/zod_gen/badge.svg)](https://docs.rs/zod_gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/cimatic/zod_gen/workflows/CI/badge.svg)](https://github.com/cimatic/zod_gen/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/cimatic/zod_gen/branch/main/graph/badge.svg)](https://codecov.io/gh/cimatic/zod_gen)

Generate [Zod](https://github.com/colinhacks/zod) schemas and TypeScript types from Rust types with zero runtime overhead and full type safety.

## Why zod_gen?

We built `zod_gen` to solve a specific gap in Rust→TypeScript sharing: many projects need more than generated TypeScript declarations. They also need runtime validation for untrusted JSON coming from APIs, forms, queues, and other boundaries.

`zod_gen` keeps Rust as the source of truth and generates [Zod](https://github.com/colinhacks/zod) schemas that can be used directly in TypeScript. That gives you `.parse()` / `.safeParse()` for runtime validation and `z.infer` for static typing from the same generated output.

## 🚀 Features

- **Zero-cost abstractions** - No runtime overhead, pure compile-time code generation
- **Full type safety** - End-to-end type safety from Rust to TypeScript
- **Serde rename support** - Automatic handling of `#[serde(rename = "...")]` attributes
- **Derive macro support** - `#[derive(ZodSchema)]` for automatic schema generation
- **Primitive type support** - Built-in support for all common Rust types
- **Generic types** - Automatic handling of `Option<T>`, `Vec<T>`, and more
- **Custom schemas** - Manual implementation for complex types
- **Batch generation** - Generate multiple schemas in a single TypeScript file

## 📦 Installation

Add both crates to your `Cargo.toml`:

```toml
[dependencies]
zod_gen = "1.4.0"
zod_gen_derive = "1.4.0"
```

## 🔧 Quick Start

### Using the derive macro (recommended)

```rust
use zod_gen::ZodSchema;
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
    is_admin: bool,
    tags: Vec<String>,
    profile: Option<UserProfile>,
}

#[derive(ZodSchema)]
struct UserProfile {
    bio: String,
    avatar_url: Option<String>,
}

#[derive(ZodSchema)]
enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
}

fn main() {
    // Generate Zod schema
    println!("{}", User::zod_schema());
    // Output: z.object({
    //   id: z.number(),
    //   name: z.string(),
    //   email: z.string(),
    //   is_admin: z.boolean(),
    //   tags: z.array(z.string()),
    //   profile: z.object({ bio: z.string(), avatar_url: z.string().optional() }).optional()
    // })
}
```

### Manual implementation

```rust
use zod_gen::{ZodSchema, zod_object, zod_string, zod_number, zod_boolean};

struct User {
    id: u64,
    name: String,
    is_admin: bool,
}

impl ZodSchema for User {
    fn zod_schema() -> String {
        zod_object(&[
            ("id", zod_number()),
            ("name", zod_string()),
            ("is_admin", zod_boolean()),
        ])
    }
}
```

### Single file generation with ZodGenerator

```rust
use zod_gen::ZodGenerator;
use std::fs;

fn generate_types() {
    let mut generator = ZodGenerator::new();

    // Add all your types with meaningful names
    generator.add_schema::<User>("User");
    generator.add_schema::<UserProfile>("UserProfile");
    generator.add_schema::<UserStatus>("UserStatus");

    // Generate a single TypeScript file with all schemas
    let content = generator.generate();
    fs::write("types/schemas.ts", content).unwrap();
}
```

## 📚 Generated TypeScript

The generated TypeScript provides both Zod schemas and inferred types in a single file:

```typescript
// Generated schemas.ts
import * as z from 'zod';

export const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
  email: z.string(),
  is_admin: z.boolean(),
  tags: z.array(z.string()),
  profile: z.object({
    bio: z.string(),
    avatar_url: z.string().optional()
  }).optional()
});
export type User = z.infer<typeof UserSchema>;

export const UserProfileSchema = z.object({
  bio: z.string(),
  avatar_url: z.string().optional()
});
export type UserProfile = z.infer<typeof UserProfileSchema>;

export const UserStatusSchema = z.union([z.literal('active'), z.literal('inactive'), z.literal('suspended')]);
export type UserStatus = z.infer<typeof UserStatusSchema>;
```

Use it in your TypeScript code:

```typescript
import { UserSchema, type User } from './types/schemas';

// Runtime validation
const validateUser = (data: unknown): User => {
  return UserSchema.parse(data);
};

// Type-safe API calls
const createUser = async (user: User): Promise<User> => {
  const response = await fetch('/api/users', {
    method: 'POST',
    body: JSON.stringify(user),
  });
  return UserSchema.parse(await response.json());
};
```

## 🏗️ Architecture

This repository contains two crates:

### [`zod_gen`](./zod_gen) - Core Library

- `ZodSchema` trait for defining schemas
- Helper functions for building Zod expressions
- `ZodGenerator` for batch file generation
- Built-in implementations for primitive types

### [`zod_gen_derive`](./zod_gen_derive) - Derive Macro

- `#[derive(ZodSchema)]` procedural macro
- Supports structs with named fields
- Supports Serde enum representations (externally tagged, internally tagged, adjacently tagged, untagged)
- Automatic dependency resolution

## 🎯 Supported Types

### Primitives
- `String`, `&str` → `z.string()` (TypeScript: `string`)
- `i32`, `i64`, `u32`, `u64`, `f32`, `f64` → `z.number()` (TypeScript: `number`)
- `bool` → `z.boolean()` (TypeScript: `boolean`)

### Generics
- `Option<T>` → `T.optional()` (TypeScript: `Optional<T>`)
- `Vec<T>` → `z.array(T)` (TypeScript: `Array<T>`)
- `HashMap<String, T>` → `z.record(z.string(), T)` (TypeScript: `Record<string, T>`)
- Custom collections via manual implementation

### Structs
- Named fields → `z.object({ ... })`
- Nested structs supported

### Enums
- Externally tagged (default) → `z.union([ ... ])` with literals/objects
- Internally tagged (`#[serde(tag = "...")]`) → `z.discriminatedUnion(...)`
- Adjacently tagged (`#[serde(tag = "...", content = "...")]`) → `z.discriminatedUnion(...)`
- Untagged (`#[serde(untagged)]`) → `z.union([ ... ])`
- Serde rename support → `#[serde(rename = "custom_name")]` → `z.literal('custom_name')`
- Internally tagged newtype structs are flattened via `z.intersection(...)`

## 🎯 Serde Rename Support

`zod_gen` automatically handles `#[serde(rename = "...")]` attributes, ensuring your TypeScript types match your serialized JSON exactly:

```rust
use serde::{Serialize, Deserialize};
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema, Serialize, Deserialize)]
enum ApiStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(ZodSchema, Serialize, Deserialize)]
enum UserRole {
    #[serde(rename = "admin")]
    Administrator,
    #[serde(rename = "user")]
    RegularUser,
    #[serde(rename = "guest")]
    GuestUser,
}
```

**Generated TypeScript:**
```typescript
export const ApiStatusSchema = z.union([z.literal('success'), z.literal('error'), z.literal('pending')]);
export type ApiStatus = z.infer<typeof ApiStatusSchema>;

export const UserRoleSchema = z.union([z.literal('admin'), z.literal('user'), z.literal('guest')]);
export type UserRole = z.infer<typeof UserRoleSchema>;
```

**Type Safety Benefits:**
```typescript
// ✅ TypeScript accepts the serde-renamed values
const status: ApiStatus = 'success';
const role: UserRole = 'admin';

// ❌ TypeScript catches typos with the Rust variant names
const badStatus: ApiStatus = 'Success'; // Error: Type '"Success"' is not assignable to type 'ApiStatus'
const badRole: UserRole = 'Administrator'; // Error: Type '"Administrator"' is not assignable to type 'UserRole'
```

This ensures perfect alignment between your Rust API and TypeScript frontend, catching serialization mismatches at compile time.

## 🎯 Serde Enum Representations

zod_gen mirrors Serde's JSON representations for enums:

- Externally tagged (default) → `z.union([ ... ])` of literals/objects
- Internally tagged (`#[serde(tag = "...")]`) → `z.discriminatedUnion(...)`
- Adjacently tagged (`#[serde(tag = "...", content = "...")]`) → `z.discriminatedUnion(...)`
- Untagged (`#[serde(untagged)]`) → `z.union([ ... ])`

Internally tagged newtype variants that wrap structs are flattened via `z.intersection(...)`.

## 🔧 Advanced Usage

### Schema Generation Strategy

By default, `zod_gen` generates **inline schemas** for nested objects. This means that complex types are expanded directly into their Zod representation:

```rust
#[derive(ZodSchema)]
struct User {
    id: u64,
    name: String,
    profile: Option<UserProfile>,
}

#[derive(ZodSchema)]
struct UserProfile {
    bio: String,
    avatar_url: Option<String>,
}

// Generates inline schema:
// z.object({
//   id: z.number(),
//   name: z.string(),
//   profile: z.object({
//     bio: z.string(),
//     avatar_url: z.string().nullable()
//   }).nullable()
// })
```

This approach works consistently across all generic types:

```rust
use std::collections::HashMap;

// HashMap<String, User> generates:
// z.record(z.string(), z.object({
//   id: z.number(),
//   name: z.string(),
//   profile: z.object({
//     bio: z.string(),
//     avatar_url: z.string().nullable()
//   }).nullable()
// }))
```

**Benefits of inline schemas:**
- ✅ Self-contained - no external dependencies
- ✅ Works with any nesting level
- ✅ Consistent behavior across all types
- ✅ No need to manage schema imports

**Considerations:**
- Schema duplication if the same type is used in multiple places
- Larger generated schemas for deeply nested structures

### User-Controlled Naming

You provide the TypeScript type names when adding schemas to the generator:

```rust
let mut gen = ZodGenerator::new();

// Use meaningful names for your TypeScript types
gen.add_schema::<HashMap<String, User>>("UserMap");
gen.add_schema::<Vec<User>>("UserList");
gen.add_schema::<Option<UserProfile>>("OptionalProfile");
```

This generates clean TypeScript with your chosen names:

```typescript
export const UserMapSchema = z.record(z.string(), UserSchema);
export type UserMap = z.infer<typeof UserMapSchema>;

export const UserListSchema = z.array(UserSchema);
export type UserList = z.infer<typeof UserListSchema>;
```

**Benefits:**
- **Full control** over TypeScript type names
- **Meaningful names** instead of auto-generated ones
- **No magic** - you decide what gets exported
- **TypeScript types are inferred** from Zod schemas using `z.infer<>`

### Single File Output

The `ZodGenerator` creates a single TypeScript file containing all your schemas. This approach:

- **Simplifies file management** - No need to track multiple files
- **Reduces complexity** - One file, one import
- **Improves maintainability** - All schemas in one place
- **Enables tree-shaking** - Import only what you need

If you need multiple files, simply create multiple generators:

```rust
// Generate API types
let mut api_gen = ZodGenerator::new();
api_gen.add_schema::<User>("User");
api_gen.add_schema::<Post>("Post");
std::fs::write("types/api.ts", api_gen.generate()).unwrap();

// Generate config types
let mut config_gen = ZodGenerator::new();
config_gen.add_schema::<AppConfig>("AppConfig");
std::fs::write("types/config.ts", config_gen.generate()).unwrap();
```

### Custom Schema Implementation

```rust
use zod_gen::ZodSchema;
use chrono::{DateTime, Utc};

impl ZodSchema for DateTime<Utc> {
    fn zod_schema() -> String {
        "z.string().datetime()".to_string()
    }
}
```

### Integration with Build Scripts

Create a `build.rs` file:

```rust
use zod_gen::ZodGenerator;

fn main() {
    let mut generator = ZodGenerator::new();
    generator.add_schema::<MyType>("MyType");

    // Generate during build
    let content = generator.generate();
    std::fs::write("frontend/types/schemas.ts", content).unwrap();
}
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### 🔄 CI/CD Pipeline

This project uses GitHub Actions for comprehensive automation:

- **🧪 Continuous Integration**: Tests run on every PR and push
  - Multi-version Rust testing (stable, beta, nightly)
  - Code formatting and linting with clippy
  - Documentation building and security audits
  - Example validation and code coverage

- **🚀 Automated Releases**: Tag-triggered releases
  - Automatic publishing to crates.io
  - GitHub release creation with changelog
  - Full validation before publishing

- **🔒 Security & Maintenance**:
  - Weekly dependency updates via automated PRs
  - Security vulnerability scanning
  - Breaking change detection on PRs

- **📚 Documentation**: Auto-deployed to GitHub Pages

### 📋 PR Guidelines

- Follow [Conventional Commits](https://conventionalcommits.org/) format
- Update `CHANGELOG.md` for non-documentation changes
- Ensure all tests pass and examples work
- Code must be formatted (`cargo fmt`) and pass clippy lints

### Development Setup

```bash
git clone https://github.com/cimatic/zod_gen.git
cd zod_gen

# The rust-toolchain.toml ensures you use the same Rust version as CI
cargo test --workspace

# Run clippy with CI-equivalent flags
./scripts/clippy-ci.sh
```

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development guidelines and CI consistency tips.

### Running Examples

```bash
# Manual ZodSchema implementation
cargo run --example basic_usage

# Using the derive macro for automatic schema generation
cargo run --example derive_example

# Using ZodGenerator for multiple schemas with custom naming
cargo run --example generator_example
```

## zod_gen vs ts-rs

[`ts-rs`](https://github.com/Aleph-Alpha/ts-rs) is an excellent Rust→TypeScript generator, especially if you only need TypeScript type declarations.

| | `zod_gen` | `ts-rs` |
| --- | --- | --- |
| Primary output | Zod schemas + `z.infer` TypeScript types | TypeScript type declarations |
| Runtime validation | Yes, via generated Zod schemas | No, focuses on TypeScript bindings |
| Frontend dependency | Zod | None required by generated types |
| Typical workflow | Generate schemas once, then validate and infer from the same file | Export TS bindings for compile-time sharing |
| Best fit | API contracts, form validation, parsing untrusted JSON, Zod-first codebases | Projects that only need shared TS types |

### When to choose zod_gen

Consider `zod_gen` over `ts-rs` when you:

- already use Zod in your frontend or TypeScript services
- want runtime validation and static typing from one generated artifact
- want generated schemas ready for `.parse()` / `.safeParse()` at API boundaries
- prefer a single generated file containing named schemas and inferred types

### When ts-rs may be enough

Consider `ts-rs` when you:

- only need TypeScript type declarations
- do not want a Zod dependency in generated output
- already validate payloads some other way and just want Rust/TypeScript type sharing

## 🙏 Acknowledgments

- [Zod](https://github.com/colinhacks/zod) - Runtime type validation for TypeScript
- [serde](https://github.com/serde-rs/serde) - Inspiration for the derive macro pattern
- [ts-rs](https://github.com/Aleph-Alpha/ts-rs) - Alternative approach to Rust→TypeScript codegen

---

**Built with ❤️ by the [Cimatic](https://cimatic.io) team**

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
