// Test file for pre-commit hook
use std::collections::HashMap;

fn main(){
let mut map=HashMap::new();
map.insert("key","value");
println!("{}",map.get("key").unwrap());
}