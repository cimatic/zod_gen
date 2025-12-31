#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.0"
//! ```

use regex::Regex;
use std::process::Command;
use std::env;

#[derive(Debug)]
struct ValidationResult {
    success: bool,
    message: String,
    annotation_level: String,
}

impl ValidationResult {
    fn success(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            annotation_level: "notice".to_string(),
        }
    }
    
    fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            annotation_level: "error".to_string(),
        }
    }
    
    fn warning(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            annotation_level: "warning".to_string(),
        }
    }
}

fn main() {
    println!("🔍 Running PR validation checks...");
    
    let mut all_passed = true;
    let mut results = Vec::new();
    
    // Check if this is a draft PR
    let is_draft = env::var("GITHUB_EVENT_PULL_REQUEST_DRAFT")
        .unwrap_or_default() == "true";
    
    // 1. Validate commit messages
    match validate_commit_messages() {
        Ok(result) => {
            print_github_annotation(&result.annotation_level, "Commit Messages", &result.message);
            results.push(result);
        }
        Err(result) => {
            print_github_annotation(&result.annotation_level, "Commit Messages", &result.message);
            all_passed = false;
            results.push(result);
        }
    }
    
    // 2. Check CHANGELOG update
    match check_changelog_update() {
        Ok(result) => {
            print_github_annotation(&result.annotation_level, "CHANGELOG Update", &result.message);
            results.push(result);
        }
        Err(result) => {
            print_github_annotation(&result.annotation_level, "CHANGELOG Update", &result.message);
            all_passed = false;
            results.push(result);
        }
    }
    
    // 3. Check for breaking changes (only for non-draft PRs)
    if !is_draft {
        match check_breaking_changes() {
            Ok(result) => {
                print_github_annotation(&result.annotation_level, "Breaking Changes", &result.message);
                results.push(result);
            }
            Err(result) => {
                print_github_annotation(&result.annotation_level, "Breaking Changes", &result.message);
                if result.annotation_level == "error" {
                    all_passed = false;
                }
                results.push(result);
            }
        }
    } else {
        let result = ValidationResult::success("Skipped breaking changes check for draft PR");
        print_github_annotation(&result.annotation_level, "Breaking Changes", &result.message);
        results.push(result);
    }
    
    // Print summary
    let passed_count = results.iter().filter(|r| r.success).count();
    let total_count = results.len();
    
    if all_passed {
        println!("✅ All PR validation checks passed ({}/{})", passed_count, total_count);
        print_github_annotation("notice", "PR Validation", &format!("✅ All {} validation checks passed", total_count));
    } else {
        println!("❌ Some PR validation checks failed ({}/{})", passed_count, total_count);
        print_github_annotation("error", "PR Validation", &format!("❌ {}/{} validation checks failed", total_count - passed_count, total_count));
        std::process::exit(1);
    }
}

fn validate_commit_messages() -> Result<ValidationResult, ValidationResult> {
    println!("📝 Validating commit messages...");
    
    // Get commit messages from origin/main to HEAD
    let output = Command::new("git")
        .args(&["log", "--format=%s", "origin/main..HEAD"])
        .output()
        .map_err(|_| ValidationResult::error("Failed to get commit messages from git"))?;
    
    if !output.status.success() {
        return Err(ValidationResult::error("Git log command failed"));
    }
    
    let commits = String::from_utf8_lossy(&output.stdout);
    let commit_lines: Vec<&str> = commits
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("Merge "))
        .collect();
    
    if commit_lines.is_empty() {
        return Ok(ValidationResult::success("No commits to validate"));
    }
    
    // Conventional commits regex pattern
    let conventional_regex = Regex::new(
        r"^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?: .+"
    ).unwrap();
    
    let mut invalid_commits = Vec::new();
    
    for commit_msg in &commit_lines {
        if !conventional_regex.is_match(commit_msg) {
            invalid_commits.push(commit_msg.to_string());
        }
    }
    
    if invalid_commits.is_empty() {
        Ok(ValidationResult::success(&format!(
            "✅ All {} commit messages follow conventional commits format", 
            commit_lines.len()
        )))
    } else {
        let error_msg = format!(
            "❌ {} invalid commit messages found:\n{}\n\nPlease use conventional commits format: type(scope): description\nValid types: feat, fix, docs, style, refactor, test, chore, perf, ci, build, revert",
            invalid_commits.len(),
            invalid_commits.join("\n")
        );
        Err(ValidationResult::error(&error_msg))
    }
}

fn check_changelog_update() -> Result<ValidationResult, ValidationResult> {
    println!("📋 Checking CHANGELOG update...");
    
    // Get changed files
    let output = Command::new("git")
        .args(&["diff", "--name-only", "origin/main..HEAD"])
        .output()
        .map_err(|_| ValidationResult::error("Failed to get changed files from git"))?;
    
    if !output.status.success() {
        return Err(ValidationResult::error("Git diff command failed"));
    }
    
    let changed_files = String::from_utf8_lossy(&output.stdout);
    let file_lines: Vec<&str> = changed_files.lines().collect();
    
    if file_lines.is_empty() {
        return Ok(ValidationResult::success("No files changed"));
    }
    
    // Check if CHANGELOG.md was updated
    let changelog_updated = file_lines.iter().any(|file| file.contains("CHANGELOG.md"));
    
    // Check if there are non-documentation changes
    let docs_only_regex = Regex::new(r"^(README\.md|\.github/|docs/)").unwrap();
    let has_non_docs_changes = file_lines.iter()
        .any(|file| !docs_only_regex.is_match(file));
    
    if has_non_docs_changes && !changelog_updated {
        Err(ValidationResult::error(
            "❌ CHANGELOG.md should be updated for non-documentation changes\n\
            Please add an entry to CHANGELOG.md describing your changes"
        ))
    } else if !has_non_docs_changes {
        Ok(ValidationResult::success("✅ Documentation-only changes, CHANGELOG update not required"))
    } else {
        Ok(ValidationResult::success("✅ CHANGELOG.md has been updated"))
    }
}

fn check_breaking_changes() -> Result<ValidationResult, ValidationResult> {
    println!("🔍 Checking for breaking changes...");
    
    // First, try to install cargo-semver-checks if not available
    let install_output = Command::new("cargo")
        .args(&["install", "cargo-semver-checks"])
        .output();
    
    match install_output {
        Ok(output) if !output.status.success() => {
            // Installation failed, but maybe it's already installed
            println!("cargo-semver-checks installation failed or already installed");
        }
        Err(_) => {
            return Err(ValidationResult::error("Failed to install cargo-semver-checks"));
        }
        _ => {
            println!("cargo-semver-checks installed successfully");
        }
    }
    
    // Run semver check
    let output = Command::new("cargo")
        .args(&["semver-checks", "check-release"])
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if output.status.success() {
                Ok(ValidationResult::success("✅ No breaking changes detected"))
            } else {
                // Check if it's a breaking change or just a tool error
                if stdout.contains("breaking") || stderr.contains("breaking") {
                    Err(ValidationResult::error(&format!(
                        "❌ Breaking changes detected:\n{}\n{}",
                        stdout, stderr
                    )))
                } else {
                    // Tool error, treat as warning
                    Ok(ValidationResult::warning(&format!(
                        "⚠️ Could not check for breaking changes (tool error):\n{}\n{}",
                        stdout, stderr
                    )))
                }
            }
        }
        Err(_) => {
            Ok(ValidationResult::warning(
                "⚠️ Could not run cargo-semver-checks - breaking change detection skipped"
            ))
        }
    }
}

fn print_github_annotation(level: &str, title: &str, message: &str) {
    println!("::{} title={}::{}", level, title, message);
}