// This file should NOT compile.
// Cannot use content attribute without tag.

use serde::{Deserialize, Serialize};
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(content = "c")]
#[allow(dead_code)]
enum BadContentOnly {
    Value(u32),
}

fn main() {}
