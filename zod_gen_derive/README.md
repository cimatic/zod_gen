# zod_gen_derive

[![Crates.io](https://img.shields.io/crates/v/zod_gen_derive.svg)](https://crates.io/crates/zod_gen_derive)
[![Documentation](https://docs.rs/zod_gen_derive/badge.svg)](https://docs.rs/zod_gen_derive)

Derive macro for `zod_gen` - automatically generate Zod schemas from Rust types.

## Features

- `#[derive(ZodSchema)]` procedural macro
- Supports structs with named fields
- Supports enums with unit variants  
- Automatic dependency resolution

## Usage

```rust
use zod_gen::ZodSchema;
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema)]
struct User {
    id: u64,
    name: String,
    is_admin: bool,
}

#[derive(ZodSchema)]
enum Status {
    Active,
    Inactive,
}
```

For more examples and documentation, see the [main repository](https://github.com/cimatic/zod_gen).