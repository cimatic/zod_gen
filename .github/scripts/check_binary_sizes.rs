#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! ```

use std::fs;
use std::path::Path;

#[derive(Debug)]
struct BinaryInfo {
    name: String,
    path: String,
    size_bytes: u64,
    size_human: String,
}

fn main() {
    println!("📦 Checking binary sizes...");
    
    let target_dir = "target/release";
    
    if !Path::new(target_dir).exists() {
        println!("⚠️ Release target directory not found. Run 'cargo build --release' first.");
        println!("::warning title=Binary Size Check::Release target directory not found");
        return;
    }
    
    match find_and_analyze_binaries(target_dir) {
        Ok(binaries) => {
            if binaries.is_empty() {
                println!("ℹ️ No binaries found in {}", target_dir);
                println!("::notice title=Binary Size Check::No binaries found to analyze");
                return;
            }
            
            print_binary_report(&binaries);
            check_size_warnings(&binaries);
        }
        Err(e) => {
            println!("❌ Failed to analyze binaries: {}", e);
            println!("::error title=Binary Size Check::Failed to analyze binaries: {}", e);
            std::process::exit(1);
        }
    }
}

fn find_and_analyze_binaries(target_dir: &str) -> Result<Vec<BinaryInfo>, String> {
    let mut binaries = Vec::new();
    
    // Look for .rlib files (Rust library files)
    find_files_with_extension(target_dir, "rlib", &mut binaries)?;
    
    // Look for executable binaries (no extension on Unix)
    find_executable_binaries(target_dir, &mut binaries)?;
    
    // Sort by size (largest first)
    binaries.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    
    Ok(binaries)
}

fn find_files_with_extension(dir: &str, extension: &str, binaries: &mut Vec<BinaryInfo>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Could not read directory {}: {}", dir, e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == extension {
                    if let Some(binary_info) = analyze_file(&path)? {
                        binaries.push(binary_info);
                    }
                }
            }
        } else if path.is_dir() {
            // Recursively search subdirectories
            if let Some(dir_name) = path.to_str() {
                find_files_with_extension(dir_name, extension, binaries)?;
            }
        }
    }
    
    Ok(())
}

fn find_executable_binaries(dir: &str, binaries: &mut Vec<BinaryInfo>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Could not read directory {}: {}", dir, e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().is_none() {
            // Check if it's executable (basic heuristic: no extension and reasonable size)
            if let Ok(metadata) = path.metadata() {
                let size = metadata.len();
                if size > 1024 && size < 100_000_000 { // Between 1KB and 100MB
                    if let Some(binary_info) = analyze_file(&path)? {
                        binaries.push(binary_info);
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn analyze_file(path: &Path) -> Result<Option<BinaryInfo>, String> {
    let metadata = path.metadata()
        .map_err(|e| format!("Could not get metadata for {:?}: {}", path, e))?;
    
    let size_bytes = metadata.len();
    let size_human = format_bytes(size_bytes);
    
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let path_str = path.to_str().unwrap_or("unknown").to_string();
    
    Ok(Some(BinaryInfo {
        name,
        path: path_str,
        size_bytes,
        size_human,
    }))
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn print_binary_report(binaries: &[BinaryInfo]) {
    println!("\n📊 Binary Size Report:");
    println!("{:<40} {:<15} {:<50}", "Name", "Size", "Path");
    println!("{}", "-".repeat(105));
    
    // Show top 10 largest binaries
    for binary in binaries.iter().take(10) {
        println!("{:<40} {:<15} {:<50}", 
                 truncate_string(&binary.name, 39),
                 binary.size_human,
                 truncate_string(&binary.path, 49));
    }
    
    if binaries.len() > 10 {
        println!("... and {} more files", binaries.len() - 10);
    }
    
    // Summary statistics
    let total_size: u64 = binaries.iter().map(|b| b.size_bytes).sum();
    let avg_size = if !binaries.is_empty() { total_size / binaries.len() as u64 } else { 0 };
    
    println!("\n📈 Summary:");
    println!("Total files: {}", binaries.len());
    println!("Total size: {}", format_bytes(total_size));
    println!("Average size: {}", format_bytes(avg_size));
    
    if let Some(largest) = binaries.first() {
        println!("Largest file: {} ({})", largest.name, largest.size_human);
    }
}

fn check_size_warnings(binaries: &[BinaryInfo]) {
    const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB
    const HUGE_FILE_THRESHOLD: u64 = 50 * 1024 * 1024;  // 50MB
    
    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    
    for binary in binaries {
        if binary.size_bytes > HUGE_FILE_THRESHOLD {
            errors.push(format!("🚨 {} is extremely large ({})", binary.name, binary.size_human));
        } else if binary.size_bytes > LARGE_FILE_THRESHOLD {
            warnings.push(format!("⚠️ {} is quite large ({})", binary.name, binary.size_human));
        }
    }
    
    // Print warnings
    if !warnings.is_empty() {
        println!("\n⚠️ Size Warnings:");
        for warning in &warnings {
            println!("{}", warning);
            println!("::warning title=Large Binary::{}", warning);
        }
    }
    
    // Print errors
    if !errors.is_empty() {
        println!("\n🚨 Size Errors:");
        for error in &errors {
            println!("{}", error);
            println!("::error title=Huge Binary::{}", error);
        }
    }
    
    // Summary annotation
    let total_issues = warnings.len() + errors.len();
    if total_issues == 0 {
        println!("::notice title=Binary Size Check::✅ All {} binaries are within reasonable size limits", binaries.len());
    } else {
        println!("::warning title=Binary Size Check::Found {} size issues ({} warnings, {} errors)", 
                 total_issues, warnings.len(), errors.len());
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}