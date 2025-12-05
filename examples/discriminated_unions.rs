//! Demonstrating discriminated union support for enums that contain data.
use serde::{Deserialize, Serialize};
use zod_gen::ZodSchema;
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema)]
#[allow(dead_code)]
enum LiteralUnionVariant {
    One,
    Two,
    Three,
}

#[derive(ZodSchema, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct DiscriminatedUnionVariant {
    text_here: String,
}

#[derive(ZodSchema, Deserialize, Serialize)]
#[allow(dead_code)]
enum DiscriminatedUnion {
    Unit,
    Unnamed(u32),
    // UnnamedTuple((u32, String)), DOES NOT WORK
    Named(DiscriminatedUnionVariant),
}

// Discriminated Unions
#[derive(ZodSchema, Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct RenamedDiscriminatedUnionVariant {
    text_here: String,
}

#[derive(ZodSchema, Debug, Deserialize, Serialize)]
#[serde(tag = "foobar")]
#[allow(dead_code)]
enum RenamedDiscriminatedUnion {
    Unit,
    Unnamed(u32),
    // UnnamedTuple((u32, String)), DOES NOT WORK
    Named(RenamedDiscriminatedUnionVariant),
}

fn main() {
    println!("=== Derive Macro Example ===");
    println!();

    println!("Literal Unions schema:");
    println!("{}", LiteralUnionVariant::zod_schema());
    println!();

    println!("Discriminated Unions schema:");
    println!("{}", DiscriminatedUnion::zod_schema());
    println!();

    println!("Renamed Discriminated Unions schema:");
    println!("{}", RenamedDiscriminatedUnion::zod_schema());
    println!();

    println!("Serializing and Deserializing discriminated union schemas:");

    let original = RenamedDiscriminatedUnion::Named(RenamedDiscriminatedUnionVariant {
        text_here: "Hi".into(),
    });
    let serialized = serde_json::to_string_pretty(&original).unwrap();
    let deserialized: RenamedDiscriminatedUnion = serde_json::from_str(&serialized).unwrap();

    println!("Serialized: {}", serialized);
    println!("Deserialized: {:#?}", deserialized);
    println!();

    println!("All schemas were generated automatically using #[derive(ZodSchema)]!");
}
