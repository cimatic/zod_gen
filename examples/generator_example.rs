use std::collections::HashMap;
use zod_gen::ZodGenerator;
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
    metadata: HashMap<String, String>,
}

#[derive(ZodSchema)]
#[allow(dead_code)]
struct UserProfile {
    bio: String,
    avatar_url: Option<String>,
    social_links: HashMap<String, String>,
}

#[derive(ZodSchema)]
#[allow(dead_code)]
enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

fn main() {
    println!("=== ZodGenerator Example ===\n");

    let mut generator = ZodGenerator::new();

    // Add various types with meaningful names
    generator.add_schema::<User>("User");
    generator.add_schema::<UserProfile>("UserProfile");
    generator.add_schema::<UserStatus>("UserStatus");

    // Add generic types with custom names
    generator.add_schema::<Vec<User>>("UserList");
    generator.add_schema::<HashMap<String, User>>("UserMap");
    generator.add_schema::<Option<UserProfile>>("OptionalProfile");

    // Generate single TypeScript file
    let content = generator.generate();

    println!("Generated TypeScript file:");
    println!("{content}");

    println!("=== Key Features Demonstrated ===");
    println!("✅ Structs with various field types (User, UserProfile)");
    println!("✅ Enums with unit variants (UserStatus)");
    println!("✅ Generic types: Vec<T>, HashMap<String, T>, Option<T>");
    println!("✅ Nested objects with inline schemas");
    println!("✅ User-controlled naming for TypeScript exports");
    println!("✅ Single file output with all schemas");
    println!("✅ TypeScript types inferred automatically with z.infer<>");
}
