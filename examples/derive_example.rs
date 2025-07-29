//! Derive macro example showing automatic ZodSchema generation

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

    println!("All schemas were generated automatically using #[derive(ZodSchema)]!");
}
