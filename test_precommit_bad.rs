// Test file for pre-commit hook - with clippy issues
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    // This will trigger clippy::uninlined-format-args
    println!("{}", map.get("key").unwrap());
    
    // Unused variable will trigger clippy warning
    let unused_var = 42;
}