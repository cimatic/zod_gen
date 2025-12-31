# zod_gen

[![Crates.io](https://img.shields.io/crates/v/zod_gen.svg)](https://crates.io/crates/zod_gen)
[![Documentation](https://docs.rs/zod_gen/badge.svg)](https://docs.rs/zod_gen)

Core library for generating Zod schemas from Rust types.

## Features

- `ZodSchema` trait for defining schemas
- Helper functions for building Zod expressions  
- `ZodGenerator` for batch file generation
- Built-in implementations for primitive types
- Serde enum representations supported by the derive macro

## Usage

```rust
use zod_gen::{ZodSchema, zod_object, zod_string, zod_number};

struct User {
    id: u64,
    name: String,
}

impl ZodSchema for User {
    fn type_name() -> String { "User".to_string() }
    fn zod_schema() -> String {
        zod_object(&[
            ("id", zod_number()),
            ("name", zod_string()),
        ])
    }
}
```

For more examples and documentation, see the [main repository](https://github.com/cimatic/zod_gen).