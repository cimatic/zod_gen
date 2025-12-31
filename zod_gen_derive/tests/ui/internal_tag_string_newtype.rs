// This file should NOT compile.
// Internally tagged newtype variants must contain a struct or map.

use serde::{Deserialize, Serialize};
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum BadInternalStringNewtype {
    Payload(String),
}

fn main() {}
