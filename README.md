# zod_gen

[![Crates.io](https://img.shields.io/crates/v/zod_gen.svg)](https://crates.io/crates/zod_gen)
[![Documentation](https://docs.rs/zod_gen/badge.svg)](https://docs.rs/zod_gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Generate [Zod](https://github.com/colinhacks/zod) schemas and TypeScript types from Rust types with zero runtime overhead and full type safety.

## 🚀 Features

- **Zero-cost abstractions** - No runtime overhead, pure compile-time code generation
- **Full type safety** - End-to-end type safety from Rust to TypeScript
- **Derive macro support** - `#[derive(ZodSchema)]` for automatic schema generation
- **Primitive type support** - Built-in support for all common Rust types
- **Generic types** - Automatic handling of `Option<T>`, `Vec<T>`, and more
- **Custom schemas** - Manual implementation for complex types
- **Batch generation** - Generate multiple schemas in a single TypeScript file

## 📦 Installation

Add both crates to your `Cargo.toml`:

```toml
[dependencies]
zod_gen = "0.1"
zod_gen_derive = "0.1"
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
    Active,
    Inactive,
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
    fn type_name() -> String {
        "User".to_string()
    }

    fn zod_schema() -> String {
        zod_object(&[
            ("id", zod_number()),
            ("name", zod_string()),
            ("is_admin", zod_boolean()),
        ])
    }
}
```

### Batch generation with ZodGenerator

```rust
use zod_gen::ZodGenerator;
use std::fs;

fn generate_types() {
    let mut generator = ZodGenerator::new();
    
    // Add all your types
    generator.add_schema::<User>();
    generator.add_schema::<UserProfile>();
    generator.add_schema::<UserStatus>();
    
    // Generate TypeScript files
    for type_name in generator.list_types() {
        let content = generator.generate_file(type_name).unwrap();
        fs::write(format!("types/{}.ts", type_name), content).unwrap();
    }
}
```

## 📚 Generated TypeScript

The generated TypeScript provides both Zod schemas and inferred types:

```typescript
// Generated User.ts
import { z } from 'zod';

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
```

Use it in your TypeScript code:

```typescript
import { UserSchema, type User } from './types/User';

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
- Supports enums with unit variants
- Automatic dependency resolution

## 🎯 Supported Types

### Primitives
- `String`, `&str` → `z.string()`
- `i32`, `i64`, `u32`, `u64`, `f32`, `f64` → `z.number()`
- `bool` → `z.boolean()`

### Generics
- `Option<T>` → `T.optional()`
- `Vec<T>` → `z.array(T)`
- Custom collections via manual implementation

### Structs
- Named fields → `z.object({ ... })`
- Nested structs supported

### Enums
- Unit variants → `z.union([z.literal('A'), z.literal('B')])`

## 🔧 Advanced Usage

### Custom Schema Implementation

```rust
use zod_gen::ZodSchema;
use chrono::{DateTime, Utc};

impl ZodSchema for DateTime<Utc> {
    fn type_name() -> String {
        "DateTime".to_string()
    }
    
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
    generator.add_schema::<MyType>();
    
    // Generate during build
    let content = generator.generate_file("MyType").unwrap();
    std::fs::write("frontend/types/MyType.ts", content).unwrap();
}
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
git clone https://github.com/cimatic/zod_gen.git
cd zod_gen
cargo test
```

### Running Examples

```bash
cargo run --example basic_usage
cargo run --example derive_example
cargo run --example generator_example
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Zod](https://github.com/colinhacks/zod) - Runtime type validation for TypeScript
- [serde](https://github.com/serde-rs/serde) - Inspiration for the derive macro pattern
- [ts-rs](https://github.com/Aleph-Alpha/ts-rs) - Alternative approach to Rust→TypeScript codegen

---

**Built with ❤️ by the [Cimatic](https://cimatic.io) team**