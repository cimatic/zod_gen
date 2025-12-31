//! Compile-fail tests for zod_gen_derive using trybuild.
//!
//! These tests verify that certain invalid enum configurations produce
//! compile-time errors, ensuring strict alignment with Serde's rules.

use std::path::PathBuf;
use trybuild::TestCases;

fn ui_test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/ui")
}

#[test]
fn test_internal_tag_tuple_fails() {
    let t = TestCases::new();
    t.compile_fail(ui_test_dir().join("internal_tag_tuple.rs"));
}

#[test]
fn test_content_without_tag_fails() {
    let t = TestCases::new();
    t.compile_fail(ui_test_dir().join("content_without_tag.rs"));
}

#[test]
fn test_tag_with_untagged_fails() {
    let t = TestCases::new();
    t.compile_fail(ui_test_dir().join("tag_with_untagged.rs"));
}
