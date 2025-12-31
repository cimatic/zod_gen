//! Derive macro example showing automatic ZodSchema generation

use serde::{Deserialize, Serialize};
use zod_gen::ZodSchema;
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema)]
#[allow(dead_code)]
struct User {
    id: u64,
    name: String,
    email: String,
    is_admin: bool,
    tags: Vec<String>,
    profile: Option<UserProfile>,
}

#[derive(ZodSchema)]
#[allow(dead_code)]
struct UserProfile {
    bio: String,
    avatar_url: Option<String>,
}

#[derive(ZodSchema)]
#[allow(dead_code)]
enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum Event {
    Joined {
        user_id: u64,
    },
    #[serde(rename = "left")]
    Left {
        user_id: u64,
    },
    Payload(Payload),
}

#[derive(ZodSchema, Serialize, Deserialize)]
#[allow(dead_code)]
struct Payload {
    reason: String,
}

#[derive(ZodSchema, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
#[allow(dead_code)]
enum Envelope {
    Text(String),
    Meta { id: u64 },
}

fn main() {
    println!("=== Derive Macro Example ===");
    println!();

    println!("User schema:");
    println!("{}", User::zod_schema());
    println!();

    println!("UserProfile schema:");
    println!("{}", UserProfile::zod_schema());
    println!();

    println!("UserStatus schema:");
    println!("{}", UserStatus::zod_schema());
    println!();

    println!("Event schema (internally tagged):");
    println!("{}", Event::zod_schema());
    println!();

    println!("Envelope schema (adjacently tagged):");
    println!("{}", Envelope::zod_schema());
    println!();

    println!("All schemas were generated automatically using #[derive(ZodSchema)]!");
}
