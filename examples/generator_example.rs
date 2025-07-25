//! ZodGenerator example showing batch TypeScript file generation

use zod_gen::{ZodGenerator, ZodSchema};
use zod_gen_derive::ZodSchema;
use std::fs;

#[derive(ZodSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(ZodSchema)]
struct Post {
    id: u64,
    title: String,
    content: String,
    author_id: u64,
    published: bool,
}

#[derive(ZodSchema)]
enum PostStatus {
    Draft,
    Published,
    Archived,
}

fn main() {
    println!("=== ZodGenerator Example ===");
    println!();
    
    let mut generator = ZodGenerator::new();
    
    // Add all types to the generator
    generator.add_schema::<User>();
    generator.add_schema::<Post>();
    generator.add_schema::<PostStatus>();
    
    println!("Registered types:");
    for type_name in generator.list_types() {
        println!("  - {}", type_name);
    }
    println!();
    
    // Create output directory
    fs::create_dir_all("generated_types").unwrap_or_default();
    
    // Generate TypeScript files
    for type_name in generator.list_types() {
        if let Some(content) = generator.generate_file(type_name) {
            let filename = format!("generated_types/{}.ts", type_name);
            fs::write(&filename, content).unwrap();
            println!("Generated: {}", filename);
        }
    }
    
    println!();
    println!("Example generated file content:");
    println!("================================");
    if let Some(content) = generator.generate_file("User") {
        println!("{}", content);
    }
}