//! Test serde rename support in ZodSchema derive
//!
//! This demonstrates that the enhanced derive macro now properly handles
//! #[serde(rename = "...")] attributes on enum variants

use serde::{Deserialize, Serialize};
use zod_gen::{ZodGenerator, ZodSchema};
use zod_gen_derive::ZodSchema;

#[derive(ZodSchema, Debug, Clone, Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_name")]
    name: String,
    status: UserStatus,
    priority: Priority,
}

#[derive(ZodSchema, Debug, Clone, Serialize, Deserialize)]
enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
}

#[derive(ZodSchema, Debug, Clone, Serialize, Deserialize)]
enum Priority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

// Test enum without serde rename (should use variant names)
#[derive(ZodSchema, Debug, Clone, Serialize, Deserialize)]
enum SimpleStatus {
    Pending,
    Approved,
    Rejected,
}

fn main() {
    println!("🔧 Serde Rename Support Test\n");

    let mut generator = ZodGenerator::new();
    generator.add_schema::<User>("User");
    generator.add_schema::<UserStatus>("UserStatus");
    generator.add_schema::<Priority>("Priority");
    generator.add_schema::<SimpleStatus>("SimpleStatus");

    let schemas = generator.generate();

    println!("=== Generated Schemas ===");
    println!("{schemas}");

    println!("=== Individual Schema Verification ===");
    println!();

    println!("User schema (with serde rename):");
    let user_schema = User::zod_schema();
    println!("Generated: {user_schema}");
    // Test that it contains the serde rename values
    assert!(
        user_schema.contains("user_name"),
        "Schema should contain user_name"
    );
    println!("✅ User correctly uses serde rename values");
    println!();

    println!("UserStatus schema (with serde rename):");
    let user_status_schema = UserStatus::zod_schema();
    println!("Generated: {user_status_schema}");

    // Test that it contains the serde rename values
    assert!(
        user_status_schema.contains("'active'"),
        "Schema should contain 'active'"
    );
    assert!(
        user_status_schema.contains("'inactive'"),
        "Schema should contain 'inactive'"
    );
    assert!(
        user_status_schema.contains("'suspended'"),
        "Schema should contain 'suspended'"
    );

    // Test that it does NOT contain the Rust variant names
    assert!(
        !user_status_schema.contains("'Active'"),
        "Schema should NOT contain 'Active'"
    );
    assert!(
        !user_status_schema.contains("'Inactive'"),
        "Schema should NOT contain 'Inactive'"
    );
    assert!(
        !user_status_schema.contains("'Suspended'"),
        "Schema should NOT contain 'Suspended'"
    );

    println!("✅ UserStatus correctly uses serde rename values");
    println!();

    println!("Priority schema (with serde rename):");
    let priority_schema = Priority::zod_schema();
    println!("Generated: {priority_schema}");

    // Test that it contains the serde rename values
    assert!(
        priority_schema.contains("'low'"),
        "Schema should contain 'low'"
    );
    assert!(
        priority_schema.contains("'medium'"),
        "Schema should contain 'medium'"
    );
    assert!(
        priority_schema.contains("'high'"),
        "Schema should contain 'high'"
    );
    assert!(
        priority_schema.contains("'critical'"),
        "Schema should contain 'critical'"
    );

    // Test that it does NOT contain the Rust variant names
    assert!(
        !priority_schema.contains("'Low'"),
        "Schema should NOT contain 'Low'"
    );
    assert!(
        !priority_schema.contains("'Medium'"),
        "Schema should NOT contain 'Medium'"
    );
    assert!(
        !priority_schema.contains("'High'"),
        "Schema should NOT contain 'High'"
    );
    assert!(
        !priority_schema.contains("'Critical'"),
        "Schema should NOT contain 'Critical'"
    );

    println!("✅ Priority correctly uses serde rename values");
    println!();

    println!("SimpleStatus schema (without serde rename):");
    let simple_status_schema = SimpleStatus::zod_schema();
    println!("Generated: {simple_status_schema}");

    // Test that it contains the Rust variant names (no serde rename)
    assert!(
        simple_status_schema.contains("'Pending'"),
        "Schema should contain 'Pending'"
    );
    assert!(
        simple_status_schema.contains("'Approved'"),
        "Schema should contain 'Approved'"
    );
    assert!(
        simple_status_schema.contains("'Rejected'"),
        "Schema should contain 'Rejected'"
    );

    println!("✅ SimpleStatus correctly uses Rust variant names");
    println!();

    println!("=== Type Safety Analysis ===");
    println!();
    println!("🎯 UserStatus enum with serde rename:");
    println!("   Rust variants: Active, Inactive, Suspended");
    println!("   Serde renames: 'active', 'inactive', 'suspended'");
    println!("   Generated schema: {user_status_schema}");
    println!();
    println!("🎯 Priority enum with serde rename:");
    println!("   Rust variants: Low, Medium, High, Critical");
    println!("   Serde renames: 'low', 'medium', 'high', 'critical'");
    println!("   Generated schema: {priority_schema}");
    println!();
    println!("🎯 SimpleStatus enum without serde rename:");
    println!("   Rust variants: Pending, Approved, Rejected");
    println!("   No serde renames - uses variant names");
    println!("   Generated schema: {simple_status_schema}");
    println!();
    println!("This means TypeScript will:");
    println!("✅ ACCEPT: const user: User = {{ name: 'John', status: 'active' }};");
    println!("❌ REJECT: const user: User = {{ name: 'John', status: 'Active' }};");
    println!("           ^^ TypeScript error: Type '\"Active\"' is not assignable to type '\"active\" | \"inactive\" | \"suspended\"'");
    println!();
    println!("✅ All tests passed! Serde rename support is working correctly!");
}
