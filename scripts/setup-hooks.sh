#!/bin/bash

# Setup script for Git hooks
# This script ensures the pre-commit hook is properly installed and executable

set -e

echo "🔧 Setting up Git hooks for zod_gen..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a Git repository"
    exit 1
fi

# Check if pre-commit hook exists
HOOK_PATH=".git/hooks/pre-commit"

if [ -f "$HOOK_PATH" ]; then
    echo "✅ Pre-commit hook already exists"
    
    # Make sure it's executable
    chmod +x "$HOOK_PATH"
    echo "✅ Pre-commit hook is executable"
else
    echo "❌ Pre-commit hook not found at $HOOK_PATH"
    echo "   This should have been created automatically."
    echo "   Please check if you're in the correct repository."
    exit 1
fi

# Test the hook by running it manually (without committing)
echo ""
echo "🧪 Testing pre-commit hook..."

# Create a temporary test to verify the hook works
echo "// Test file" > test_hook_setup.rs
git add test_hook_setup.rs

# Run the hook manually
if .git/hooks/pre-commit; then
    echo "✅ Pre-commit hook test passed"
else
    echo "❌ Pre-commit hook test failed"
    git reset HEAD test_hook_setup.rs
    rm test_hook_setup.rs
    exit 1
fi

# Clean up
git reset HEAD test_hook_setup.rs
rm test_hook_setup.rs

echo ""
echo "🎉 Git hooks setup complete!"
echo ""
echo "The pre-commit hook will now run automatically before each commit to:"
echo "  • Check code formatting (rustfmt)"
echo "  • Run clippy linting"
echo "  • Run tests"
echo "  • Verify examples work"
echo ""
echo "Happy coding! 🦀"