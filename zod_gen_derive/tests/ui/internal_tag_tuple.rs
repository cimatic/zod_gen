// This file should NOT compile.
// Internally tagged enums do not support tuple variants.

use serde::{Deserialize, Serialize};
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum BadInternalTuple {
    Tuple(u32, String),
}

fn main() {}
