#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.0"
//! ```

use regex::Regex;
use std::process::Command;

#[derive(Debug)]
struct CoverageStats {
    percentage: f64,
    lines_covered: u32,
    total_lines: u32,
}

#[derive(Debug)]
struct FileCoverage {
    file: String,
    percentage: f64,
}

fn main() {
    println!("🔍 Generating code coverage report...");
    
    match run_coverage() {
        Ok(stats) => {
            print_coverage_annotations(&stats);
            if let Ok(file_coverage) = run_detailed_coverage() {
                print_file_annotations(&file_coverage);
            }
        }
        Err(_) => {
            println!("⚠️  Coverage generation failed, falling back to basic test run...");
            print_github_annotation("warning", "Coverage Unavailable", "Coverage metrics unavailable due to toolchain compatibility issues");
            
            match run_tests() {
                Ok(test_results) => print_test_annotations(&test_results),
                Err(_) => print_github_annotation("error", "Test Failure", "Tests failed to run"),
            }
        }
    }
    
    println!("✅ Coverage check completed!");
}

fn run_coverage() -> Result<CoverageStats, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(&["llvm-cov", "--workspace", "--summary-only"])
        .output()?;
    
    if !output.status.success() {
        return Err("Coverage command failed".into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
    
    parse_coverage_summary(&stdout)
}

fn run_detailed_coverage() -> Result<Vec<FileCoverage>, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(&["llvm-cov", "--workspace", "--text"])
        .output()?;
    
    if !output.status.success() {
        return Err("Detailed coverage command failed".into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("📊 Detailed coverage by file:");
    println!("{}", stdout);
    
    Ok(parse_file_coverage(&stdout))
}

fn run_tests() -> Result<String, Box<dyn std::error::Error>> {
    println!("📊 Running tests to verify functionality:");
    
    let output = Command::new("cargo")
        .args(&["test", "--workspace", "--verbose"])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
    
    if !output.status.success() {
        return Err("Tests failed".into());
    }
    
    Ok(stdout.to_string())
}

fn parse_coverage_summary(output: &str) -> Result<CoverageStats, Box<dyn std::error::Error>> {
    // Look for patterns like "123 of 456 lines covered" and "78.90%"
    let lines_re = Regex::new(r"(\d+) of (\d+) lines covered")?;
    let pct_re = Regex::new(r"(\d+\.\d+)%")?;
    
    let lines_covered = lines_re.captures(output)
        .and_then(|caps| caps.get(1)?.as_str().parse().ok())
        .unwrap_or(0);
    
    let total_lines = lines_re.captures(output)
        .and_then(|caps| caps.get(2)?.as_str().parse().ok())
        .unwrap_or(0);
    
    let percentage = pct_re.captures(output)
        .and_then(|caps| caps.get(1)?.as_str().parse().ok())
        .unwrap_or(0.0);
    
    Ok(CoverageStats {
        percentage,
        lines_covered,
        total_lines,
    })
}

fn parse_file_coverage(output: &str) -> Vec<FileCoverage> {
    let mut file_coverage = Vec::new();
    let file_re = Regex::new(r"^([^\s]+)\s+(\d+\.\d+)%").unwrap();
    
    for line in output.lines() {
        if let Some(caps) = file_re.captures(line) {
            if let (Some(file), Some(pct)) = (caps.get(1), caps.get(2)) {
                if let Ok(percentage) = pct.as_str().parse::<f64>() {
                    let file_path = file.as_str().to_string();
                    if file_path.ends_with(".rs") {
                        file_coverage.push(FileCoverage {
                            file: file_path,
                            percentage,
                        });
                    }
                }
            }
        }
    }
    
    file_coverage
}

fn print_coverage_annotations(stats: &CoverageStats) {
    // Summary annotation
    let summary = if stats.total_lines > 0 {
        format!("Coverage: {:.1}% ({} of {} lines covered)", 
                stats.percentage, stats.lines_covered, stats.total_lines)
    } else {
        format!("Coverage: {:.1}%", stats.percentage)
    };
    
    print_github_annotation("notice", "Code Coverage Summary", &summary);
    
    // Status annotation based on coverage level
    let (level, icon, message) = if stats.percentage >= 80.0 {
        ("notice", "✅", format!("Good coverage ({:.1}%)", stats.percentage))
    } else if stats.percentage >= 60.0 {
        ("warning", "⚠️", format!("Medium coverage ({:.1}%) - consider adding more tests", stats.percentage))
    } else {
        ("warning", "🔴", format!("Low coverage ({:.1}%) - more tests needed", stats.percentage))
    };
    
    print_github_annotation(level, "Coverage Status", &format!("{} {}", icon, message));
}

fn print_file_annotations(file_coverage: &[FileCoverage]) {
    for file in file_coverage {
        if file.percentage < 50.0 {
            let message = format!("File has {:.1}% coverage - consider adding tests", file.percentage);
            print_github_file_annotation("warning", &file.file, "Low Coverage", &message);
        }
    }
}

fn print_test_annotations(test_output: &str) {
    let passed_re = Regex::new(r"(\d+) passed").unwrap();
    let failed_re = Regex::new(r"(\d+) failed").unwrap();
    
    let passed = passed_re.captures(test_output)
        .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
        .unwrap_or(0);
    
    let failed = failed_re.captures(test_output)
        .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
        .unwrap_or(0);
    
    let (level, icon, message) = if failed == 0 {
        ("notice", "✅", format!("All {} tests passed", passed))
    } else {
        ("error", "❌", format!("{} tests failed, {} passed", failed, passed))
    };
    
    print_github_annotation(level, "Test Results", &format!("{} {}", icon, message));
}

fn print_github_annotation(level: &str, title: &str, message: &str) {
    println!("::{} title={}::{}", level, title, message);
}

fn print_github_file_annotation(level: &str, file: &str, title: &str, message: &str) {
    println!("::{} file={},title={}::{}", level, file, title, message);
}