//! Tests for serde enum representations and their Zod schema generation.
//!
//! This module tests that zod_gen correctly reflects Serde's actual JSON
//! representation in the generated Zod schemas.
//!
//! Representations tested:
//! - Externally tagged (Serde default)
//! - Internally tagged (`#[serde(tag = "...")]`)
//! - Adjacently tagged (`#[serde(tag = "...", content = "...")]`)
//! - Untagged (`#[serde(untagged)]`)
//!
//! Key invariants:
//! - Default behavior matches Serde's actual JSON (externally tagged).
//! - "Discriminated union" Zod shape only when Rust uses `#[serde(tag=...)]` or `#[serde(tag, content)]`.
//! - No implicit default tag key; if `#[serde(tag=...)]` exists, use exactly that string.

use serde::{Deserialize, Serialize};
use zod_gen::ZodSchema as _;
use zod_gen_derive::ZodSchema;

// ============================================================================
// BACKWARD COMPATIBILITY: All-unit enums should keep producing union of literals
// ============================================================================

#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum Status {
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

#[test]
fn test_backward_compat_unit_only_enum() {
    let schema = Status::zod_schema();
    // Must contain literal unions for unit variants
    assert!(schema.contains("z.literal('Active')"), "schema: {schema}");
    assert!(schema.contains("z.literal('inactive')"), "schema: {schema}");
    // Must be a union
    assert!(schema.contains("z.union(["), "schema: {schema}");
}

// ============================================================================
// EXTERNALLY TAGGED (Serde default)
// ============================================================================

// 1) Unit + newtype mixed
#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum ExternalUnitNewtype {
    Unit,
    Unnamed(u32),
}

#[test]
fn test_external_unit_newtype_mixed() {
    let schema = ExternalUnitNewtype::zod_schema();
    // Unit variant becomes literal
    assert!(schema.contains("z.literal('Unit')"), "schema: {schema}");
    // Newtype variant: { Unnamed: z.number() }
    assert!(schema.contains("Unnamed: z.number()"), "schema: {schema}");
    // Overall union
    assert!(schema.contains("z.union(["), "schema: {schema}");
}

// 2) Struct variant (default external tagging)
#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum ExternalStructOnly {
    Request { id: String, method: String },
}

#[test]
fn test_external_struct_variant() {
    let schema = ExternalStructOnly::zod_schema();
    // Should contain the variant name as key
    assert!(schema.contains("Request: z.object({"), "schema: {schema}");
    // Should contain fields
    assert!(schema.contains("id: z.string()"), "schema: {schema}");
    assert!(schema.contains("method: z.string()"), "schema: {schema}");
}

// 3) Tuple variant (len >= 2)
#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum ExternalTuple {
    Pair(u32, String),
}

#[test]
fn test_external_tuple_variant() {
    let schema = ExternalTuple::zod_schema();
    // Tuple becomes z.tuple([...])
    assert!(
        schema.contains("z.tuple([z.number(), z.string()])"),
        "schema: {schema}"
    );
    // Variant key wraps it
    assert!(schema.contains("Pair: z.tuple("), "schema: {schema}");
}

// 4) Externally tagged with rename on variant
#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum ExternalRenameVariant {
    #[serde(rename = "req")]
    Request { id: String },
}

#[test]
fn test_external_rename_variant() {
    let schema = ExternalRenameVariant::zod_schema();
    // Must use renamed variant name as key
    assert!(schema.contains("req: z.object({"), "schema: {schema}");
    // Must NOT use Rust variant name
    assert!(!schema.contains("Request:"), "schema: {schema}");
}

// 5) Externally tagged with rename on struct fields
#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum ExternalRenameField {
    #[serde(rename = "req")]
    Request {
        #[serde(rename = "request_id")]
        id: String,
    },
}

#[test]
fn test_external_rename_field() {
    let schema = ExternalRenameField::zod_schema();
    // Variant key uses rename
    assert!(schema.contains("req: z.object({"), "schema: {schema}");
    // Field name uses rename
    assert!(
        schema.contains("request_id: z.string()"),
        "schema: {schema}"
    );
}

// 6) Fully mixed external (unit + newtype + tuple + struct)
#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum ExternalMixedAll {
    Unit,
    Unnamed(u32),
    Tuple(u32, String),
    Struct { id: u32, name: String },
}

#[test]
fn test_external_mixed_all() {
    let schema = ExternalMixedAll::zod_schema();
    // Unit -> literal
    assert!(schema.contains("z.literal('Unit')"), "schema: {schema}");
    // Newtype -> object with number
    assert!(schema.contains("Unnamed: z.number()"), "schema: {schema}");
    // Tuple -> tuple schema
    assert!(schema.contains("Tuple: z.tuple("), "schema: {schema}");
    // Struct -> nested object
    assert!(schema.contains("Struct: z.object({"), "schema: {schema}");
    // Overall union
    assert!(schema.contains("z.union(["), "schema: {schema}");
}

// ============================================================================
// INTERNALLY TAGGED (#[serde(tag = "...")])
// ============================================================================

// 1) Basic internal tagged (struct + unit)
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum InternalStructUnit {
    Request { id: String },
    Response { ok: bool },
}

#[test]
fn test_internal_struct_unit() {
    let schema = InternalStructUnit::zod_schema();
    // Must use discriminatedUnion
    assert!(
        schema.contains("z.discriminatedUnion('type', ["),
        "schema: {schema}"
    );
    // Variant literals
    assert!(
        schema.contains("type: z.literal('Request')"),
        "schema: {schema}"
    );
    assert!(
        schema.contains("type: z.literal('Response')"),
        "schema: {schema}"
    );
    // Fields inline with tag
    assert!(schema.contains("id: z.string()"), "schema: {schema}");
    assert!(schema.contains("ok: z.boolean()"), "schema: {schema}");
}

// 2) Internal tagged with rename on variant
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum InternalRenameVariant {
    #[serde(rename = "req")]
    Request { id: String },
}

#[test]
fn test_internal_rename_variant() {
    let schema = InternalRenameVariant::zod_schema();
    // Must use renamed variant name as literal
    assert!(
        schema.contains("type: z.literal('req')"),
        "schema: {schema}"
    );
    // Must NOT use Rust variant name
    assert!(
        !schema.contains("type: z.literal('Request')"),
        "schema: {schema}"
    );
}

// 3) Internal tagged with rename on field
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum InternalRenameField {
    #[serde(rename = "req")]
    Request {
        #[serde(rename = "request_id")]
        id: String,
    },
}

#[test]
fn test_internal_rename_field() {
    let schema = InternalRenameField::zod_schema();
    // Variant rename
    assert!(
        schema.contains("type: z.literal('req')"),
        "schema: {schema}"
    );
    // Field rename
    assert!(
        schema.contains("request_id: z.string()"),
        "schema: {schema}"
    );
}

// 4) Internal tagged with newtype struct (allowed by Serde)
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum InternalNewtypeStruct {
    Payload(PayloadStruct),
}

#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
struct PayloadStruct {
    x: u32,
}

#[test]
fn test_internal_newtype_struct() {
    let schema = InternalNewtypeStruct::zod_schema();
    // Must use discriminatedUnion
    assert!(
        schema.contains("z.discriminatedUnion('type', ["),
        "schema: {schema}"
    );
    // Variant literal present
    assert!(
        schema.contains("type: z.literal('Payload')"),
        "schema: {schema}"
    );
    // Fields from struct must be present
    assert!(schema.contains("x: z.number()"), "schema: {schema}");
    // Must NOT have a "data" wrapper
    assert!(!schema.contains(r#""data""#), "schema: {schema}");
    // Should use intersection for flattening
    assert!(schema.contains("z.intersection("), "schema: {schema}");
}

// ============================================================================
// ADJACENTLY TAGGED (#[serde(tag = "...", content = "...")])
// ============================================================================

// 1) Adjacent tag + content (unit, newtype, tuple, struct)
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
#[allow(dead_code)]
enum AdjacentAll {
    Unit,
    Str(String),
    Pair(u32, String),
    Obj { id: String },
}

#[test]
fn test_adjacent_all() {
    let schema = AdjacentAll::zod_schema();
    // Must use discriminatedUnion
    assert!(
        schema.contains("z.discriminatedUnion('t', ["),
        "schema: {schema}"
    );
    // Unit variant has no "c"
    assert!(schema.contains("t: z.literal('Unit')"), "schema: {schema}");
    // Newtype (Str) has "c" with string
    assert!(schema.contains("t: z.literal('Str')"), "schema: {schema}");
    assert!(schema.contains("c: z.string()"), "schema: {schema}");
    // Tuple has "c" with tuple
    assert!(schema.contains("t: z.literal('Pair')"), "schema: {schema}");
    assert!(schema.contains("c: z.tuple("), "schema: {schema}");
    // Struct has "c" with object
    assert!(schema.contains("t: z.literal('Obj')"), "schema: {schema}");
    assert!(schema.contains("c: z.object({"), "schema: {schema}");
}

// 2) Adjacent with rename on variant
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
#[allow(dead_code)]
enum AdjacentRename {
    #[serde(rename = "unit")]
    Unit,
}

#[test]
fn test_adjacent_rename() {
    let schema = AdjacentRename::zod_schema();
    // Must use renamed variant name
    assert!(schema.contains("t: z.literal('unit')"), "schema: {schema}");
}

// 3) Adjacent with custom tag and content keys
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
#[allow(dead_code)]
enum AdjacentCustomKeys {
    Value(u32),
}

#[test]
fn test_adjacent_custom_keys() {
    let schema = AdjacentCustomKeys::zod_schema();
    // Must use custom tag key
    assert!(
        schema.contains("z.discriminatedUnion('kind', ["),
        "schema: {schema}"
    );
    // Must use custom content key
    assert!(schema.contains("payload: z.number()"), "schema: {schema}");
}

// ============================================================================
// UNTAGGED (#[serde(untagged)])
// ============================================================================

// 1) Untagged: number or object
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
enum UntaggedNumberOrObject {
    Num(u32),
    Obj { id: u32 },
}

#[test]
fn test_untagged_number_or_object() {
    let schema = UntaggedNumberOrObject::zod_schema();
    // Plain union, no tag
    assert!(schema.contains("z.union(["), "schema: {schema}");
    // Number variant
    assert!(schema.contains("z.number()"), "schema: {schema}");
    // Object variant
    assert!(schema.contains("z.object({"), "schema: {schema}");
    assert!(schema.contains("id: z.number()"), "schema: {schema}");
}

// 2) Untagged: tuple and unit (unit becomes null in JSON)
#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
enum UntaggedTupleUnit {
    Pair(String, String),
    Unit,
}

#[test]
fn test_untagged_tuple_unit() {
    let schema = UntaggedTupleUnit::zod_schema();
    // Tuple
    assert!(
        schema.contains("z.tuple([z.string(), z.string()])"),
        "schema: {schema}"
    );
    // Unit becomes null (serde serializes unit as null in untagged)
    assert!(schema.contains("z.null()"), "schema: {schema}");
}

// ============================================================================
// EDGE CASES
// ============================================================================

// Nested enum in struct
#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
struct Outer {
    inner: Inner,
}

#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
enum Inner {
    #[serde(rename = "val")]
    Value(u32),
}

#[test]
fn test_nested_enum_in_struct() {
    let schema = Outer::zod_schema();
    // Outer object with inner field
    assert!(schema.contains("inner: z.union("), "schema: {schema}");
    // Inner enum with renamed variant
    assert!(schema.contains("val: z.number()"), "schema: {schema}");
}
