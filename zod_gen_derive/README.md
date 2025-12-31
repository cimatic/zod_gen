# zod_gen_derive

[![Crates.io](https://img.shields.io/crates/v/zod_gen_derive.svg)](https://crates.io/crates/zod_gen_derive)
[![Documentation](https://docs.rs/zod_gen_derive/badge.svg)](https://docs.rs/zod_gen_derive)

Derive macro for `zod_gen` - automatically generate Zod schemas from Rust types.

## Features

- `#[derive(ZodSchema)]` procedural macro
- Supports structs with named fields
- Supports Serde enum representations (externally tagged, internally tagged, adjacently tagged, untagged) and generates appropriate unions/discriminated unions  
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

### Serde enum representations

- Externally tagged (default) → `z.union([ ... ])`
- Internally tagged (`#[serde(tag = "...")]`) → `z.discriminatedUnion(...)`
- Adjacently tagged (`#[serde(tag = "...", content = "...")]`) → `z.discriminatedUnion(...)`
- Untagged (`#[serde(untagged)]`) → `z.union([ ... ])`

Internally tagged enums do not allow tuple variants, and internally tagged newtype variants must wrap object-like payloads.

For more examples and documentation, see the [main repository](https://github.com/cimatic/zod_gen).