#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
usage() {
    echo "Usage: $0 <version> [--dry-run]"
    echo ""
    echo "Examples:"
    echo "  $0 0.3.1        # Release version 0.3.1"
    echo "  $0 0.4.0        # Release version 0.4.0"
    echo "  $0 0.3.1 --dry-run  # Dry run (no actual changes)"
    echo ""
    echo "The script will:"
    echo "  1. Update version in Cargo.toml"
    echo "  2. Run cargo check to update Cargo.lock"
    echo "  3. Run tests to ensure everything works"
    echo "  4. Commit the changes"
    echo "  5. Create and push a git tag"
    echo "  6. Push changes to origin"
    exit 1
}

# Check if version argument is provided
if [ $# -eq 0 ]; then
    print_error "Version argument required"
    usage
fi

VERSION=$1
DRY_RUN=false

# Check for dry-run flag
if [ "$2" = "--dry-run" ]; then
    DRY_RUN=true
    print_warning "DRY RUN MODE - No changes will be made"
fi

# Validate version format (basic semver check)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Invalid version format. Use semantic versioning (e.g., 1.0.0)"
    exit 1
fi

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check if working directory is clean
if [ "$DRY_RUN" = false ] && ! git diff-index --quiet HEAD --; then
    print_error "Working directory is not clean. Please commit or stash changes first."
    exit 1
fi

# Check if tag already exists
if git tag -l | grep -q "^v${VERSION}$"; then
    print_error "Tag v${VERSION} already exists"
    exit 1
fi

print_info "Preparing release for version ${VERSION}"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
print_info "Current version: ${CURRENT_VERSION}"
print_info "New version: ${VERSION}"

if [ "$DRY_RUN" = true ]; then
    print_info "Would update Cargo.toml version from ${CURRENT_VERSION} to ${VERSION}"
    print_info "Would run: cargo check"
    print_info "Would run: cargo test"
    print_info "Would commit: 'bump version to v${VERSION}'"
    print_info "Would create tag: v${VERSION}"
    print_info "Would push to origin: main and v${VERSION}"
    print_success "Dry run complete - no changes made"
    exit 0
fi

# Update version in Cargo.toml
print_info "Updating Cargo.toml version..."
sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

# Update Cargo.lock
print_info "Updating Cargo.lock..."
cargo check

# Run tests
print_info "Running tests..."
cargo test

# Commit changes
print_info "Committing version bump..."
git add Cargo.toml Cargo.lock
git commit -m "bump version to v${VERSION}"

# Create tag
print_info "Creating tag v${VERSION}..."
git tag "v${VERSION}"

# Push changes and tag
print_info "Pushing changes to origin..."
git push origin main
git push origin "v${VERSION}"

print_success "Release v${VERSION} completed successfully!"
print_info "GitHub Actions will now build and create the release automatically."
print_info "Check: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"