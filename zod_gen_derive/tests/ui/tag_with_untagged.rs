// This file should NOT compile.
// Cannot combine tag with untagged.

use serde::{Deserialize, Serialize};
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type", untagged)]
#[allow(dead_code)]
enum BadTagUntagged {
    Value(u32),
}

fn main() {}
