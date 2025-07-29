# GitHub Actions Setup Guide

This document explains how to configure the GitHub Actions workflows for this repository.

## Required Secrets

To enable all workflows, you need to configure these secrets in your GitHub repository settings:

### 1. CARGO_REGISTRY_TOKEN

**Purpose**: Allows automated publishing to crates.io

**Setup**:
1. Go to [crates.io](https://crates.io) and log in
2. Go to Account Settings → API Tokens
3. Create a new token with "Publish new crates and new versions of existing crates" scope
4. Copy the token
5. In GitHub: Settings → Secrets and variables → Actions → New repository secret
6. Name: `CARGO_REGISTRY_TOKEN`
7. Value: Your crates.io token

### 2. GITHUB_TOKEN

**Purpose**: Create releases, issues, and PRs

**Setup**: This is automatically provided by GitHub Actions. No manual setup required.

## Workflow Overview

### 🧪 CI Workflow (`ci.yml`)
- **Triggers**: Push to main, PRs
- **Purpose**: Run tests, formatting, clippy, docs, security audit, coverage
- **Rust versions**: stable, beta, nightly
- **Outputs**: Test results, coverage reports

### 🚀 Release Workflow (`release.yml`)
- **Triggers**: Git tags matching `v*`
- **Purpose**: Validate, publish to crates.io, create GitHub release
- **Requirements**: `CARGO_REGISTRY_TOKEN` secret
- **Process**: 
  1. Run full validation
  2. Publish `zod_gen` first
  3. Publish `zod_gen_derive` second
  4. Create GitHub release with changelog

### 🔍 PR Workflow (`pr.yml`)
- **Triggers**: Pull requests to main
- **Purpose**: Enhanced PR validation
- **Checks**:
  - Breaking change detection
  - Commit message validation (conventional commits)
  - CHANGELOG.md update requirement
  - Full test suite + examples
  - Binary size monitoring

### 🔄 Dependencies Workflow (`dependencies.yml`)
- **Triggers**: Weekly schedule (Mondays 9 AM UTC), manual dispatch
- **Purpose**: Keep dependencies updated and secure
- **Actions**:
  - Update all dependencies
  - Run tests
  - Create PR with updates
  - Security audit with issue creation on vulnerabilities

### 📚 Documentation Workflow (`docs.yml`)
- **Triggers**: Push to main (when docs change), manual dispatch
- **Purpose**: Deploy documentation to GitHub Pages
- **Requirements**: Enable GitHub Pages in repository settings
- **Output**: Documentation available at `https://username.github.io/repository-name`

## Repository Settings

### Enable GitHub Pages
1. Go to Settings → Pages
2. Source: GitHub Actions
3. This allows the docs workflow to deploy documentation

### Branch Protection (Recommended)
1. Go to Settings → Branches
2. Add rule for `main` branch:
   - Require status checks to pass before merging
   - Require branches to be up to date before merging
   - Select status checks: `Test Suite`, `Rustfmt`, `Clippy`, `Documentation`
   - Require pull request reviews before merging

### Auto-merge Setup (Optional)
1. Go to Settings → General
2. Enable "Allow auto-merge"
3. This allows dependabot PRs to be auto-merged after CI passes

## Testing the Setup

### Test CI Workflow
1. Create a small change and push to main
2. Check Actions tab to see CI running
3. Verify all jobs pass

### Test Release Workflow
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit and tag: `git tag v1.0.1 && git push origin main --tags`
4. Check Actions tab to see release workflow
5. Verify packages are published to crates.io
6. Check that GitHub release is created

### Test PR Workflow
1. Create a branch with changes
2. Open a PR
3. Verify enhanced checks run
4. Test commit message validation
5. Test CHANGELOG requirement

## Troubleshooting

### Release Fails with "dependency not found"
- The derive crate depends on the main crate
- Ensure `zod_gen` publishes successfully before `zod_gen_derive`
- The workflow includes a 30-second wait between publications

### Documentation Deployment Fails
- Ensure GitHub Pages is enabled in repository settings
- Check that the workflow has `pages: write` permission
- If you see "deprecated version" errors, update action versions in workflows

### Dependency Updates Create Conflicts
- The workflow will create PRs for dependency updates
- Review and merge manually if there are conflicts
- Consider pinning problematic dependencies

### Security Audit Fails
- The workflow will create issues for security vulnerabilities
- Review the issue and update affected dependencies
- Run `cargo audit` locally for details

## Customization

### Modify Test Matrix
Edit `ci.yml` to change Rust versions:
```yaml
strategy:
  matrix:
    rust:
      - stable
      - beta  # Remove if not needed
      - nightly  # Remove if not needed
```

### Change Release Schedule
Edit `dependencies.yml` cron schedule:
```yaml
schedule:
  - cron: '0 9 * * 1'  # Weekly on Mondays at 9 AM UTC
```

### Add More Checks
Add additional steps to any workflow as needed for your specific requirements.