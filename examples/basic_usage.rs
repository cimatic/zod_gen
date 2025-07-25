//! Basic usage example showing manual ZodSchema implementation

use zod_gen::{ZodSchema, zod_object, zod_string, zod_number, zod_boolean};

struct User {
    id: u64,
    name: String,
    is_admin: bool,
}

impl ZodSchema for User {
    fn type_name() -> String {
        "User".to_string()
    }

    fn zod_schema() -> String {
        zod_object(&[
            ("id", zod_number()),
            ("name", zod_string()),
            ("is_admin", zod_boolean()),
        ])
    }
}

fn main() {
    println!("=== Basic Usage Example ===");
    println!();
    
    println!("Type name: {}", User::type_name());
    println!();
    
    println!("Zod schema:");
    println!("{}", User::zod_schema());
    println!();
    
    println!("This would generate the following TypeScript:");
    println!("export const UserSchema = {};", User::zod_schema());
    println!("export type User = z.infer<typeof UserSchema>;");
}