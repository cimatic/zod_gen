#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.0"
//! ```

use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let version = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("❌ Usage: extract_changelog.rs <version>");
            eprintln!("Example: extract_changelog.rs 1.0.0");
            std::process::exit(1);
        }
    };
    
    println!("📋 Extracting changelog for version {}", version);
    
    match extract_changelog_section(&version) {
        Ok(changelog_content) => {
            if changelog_content.trim().is_empty() {
                println!("⚠️ No changelog entry found for version {}", version);
                println!("::warning title=Changelog Extraction::No changelog entry found for version {}", version);
                
                // Output a default message
                let default_content = format!("Release v{}\n\nSee the full changelog at: https://github.com/your-org/zod_gen/blob/main/CHANGELOG.md", version);
                output_to_github_output("CHANGELOG", &default_content);
            } else {
                println!("✅ Successfully extracted changelog for version {}", version);
                println!("::notice title=Changelog Extraction::Successfully extracted {} lines of changelog content", changelog_content.lines().count());
                
                // Output the extracted content
                output_to_github_output("CHANGELOG", &changelog_content);
                
                // Also print it for visibility
                println!("\n📄 Extracted changelog content:");
                println!("{}", changelog_content);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to extract changelog: {}", e);
            println!("::error title=Changelog Extraction::Failed to extract changelog: {}", e);
            std::process::exit(1);
        }
    }
}

fn extract_changelog_section(version: &str) -> Result<String, String> {
    // Read CHANGELOG.md
    let changelog_content = fs::read_to_string("CHANGELOG.md")
        .map_err(|e| format!("Could not read CHANGELOG.md: {}", e))?;
    
    // Create regex patterns for version headers
    // Matches patterns like:
    // ## [1.0.0] - 2023-01-01
    // ## [1.0.0]
    // ## 1.0.0
    let version_header_patterns = vec![
        format!(r"^## \[{}\]", regex::escape(version)),  // ## [1.0.0]
        format!(r"^## {}", regex::escape(version)),       // ## 1.0.0
    ];
    
    let mut section_content = String::new();
    let mut in_target_section = false;
    let mut found_section = false;
    
    for line in changelog_content.lines() {
        // Check if this line starts a version section
        let is_version_header = version_header_patterns.iter()
            .any(|pattern| Regex::new(pattern).unwrap().is_match(line));
        
        if is_version_header {
            if !found_section {
                // This is our target version
                in_target_section = true;
                found_section = true;
                continue; // Skip the header line itself
            } else {
                // We've found another version section, stop here
                break;
            }
        }
        
        // Check if this line starts any version section (to stop extraction)
        let is_any_version_header = Regex::new(r"^## (\[.*\]|\d+\.\d+\.\d+)")
            .unwrap()
            .is_match(line);
        
        if in_target_section {
            if is_any_version_header {
                // We've hit the next version section, stop
                break;
            }
            
            // Add this line to our section content
            section_content.push_str(line);
            section_content.push('\n');
        }
    }
    
    if !found_section {
        return Err(format!("No changelog section found for version {}", version));
    }
    
    // Clean up the content
    let cleaned_content = section_content
        .trim()
        .lines()
        .map(|line| line.trim_end()) // Remove trailing whitespace
        .collect::<Vec<_>>()
        .join("\n");
    
    Ok(cleaned_content)
}

fn output_to_github_output(key: &str, value: &str) {
    // GitHub Actions output format with multiline support
    println!("{}<<EOF", key);
    println!("{}", value);
    println!("EOF");
    
    // Also set it as an environment variable for the step
    if let Ok(github_output) = env::var("GITHUB_OUTPUT") {
        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&github_output)
        {
            let _ = writeln!(file, "{}<<EOF", key);
            let _ = writeln!(file, "{}", value);
            let _ = writeln!(file, "EOF");
        }
    }
}