//! Basic usage example showing manual ZodSchema implementation

use zod_gen::{zod_boolean, zod_number, zod_object, zod_string, ZodSchema};

struct User {
    id: u64,
    name: String,
    is_admin: bool,
}

impl ZodSchema for User {
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

    println!("Zod schema:");
    println!("{}", User::zod_schema());
    println!();

    println!("This would generate the following TypeScript:");
    println!("export const UserSchema = {};", User::zod_schema());
    println!("export type User = z.infer<typeof UserSchema>;");
}
