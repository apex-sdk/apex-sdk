#!/bin/bash
# Apex SDK Documentation Migration Script
# This script automates the migration of documentation to a separate repository

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOCS_REPO_NAME="apex-sdk-docs"
DOCS_REPO_ORG="eurybits"
DOCS_REPO_URL="https://github.com/${DOCS_REPO_ORG}/${DOCS_REPO_NAME}.git"
DOCS_DIR="../${DOCS_REPO_NAME}"

echo -e "${BLUE}╔════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Apex SDK Documentation Migration Tool   ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════╝${NC}"
echo ""

# Function to print step header
print_step() {
    echo -e "${GREEN}▶ $1${NC}"
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Function to print error
print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Check prerequisites
print_step "Checking prerequisites..."

if ! command -v git &> /dev/null; then
    print_error "git is not installed"
    exit 1
fi

if ! command -v gh &> /dev/null; then
    print_warning "GitHub CLI (gh) is not installed - you'll need to create the repository manually"
    print_warning "Install with: brew install gh (macOS) or sudo apt install gh (Ubuntu)"
    USE_GH_CLI=false
else
    print_success "GitHub CLI found"
    USE_GH_CLI=true
fi

# Check if we're in the apex-sdk directory
if [ ! -f "Cargo.toml" ] || [ ! -d "docs" ]; then
    print_error "Please run this script from the apex-sdk root directory"
    exit 1
fi

print_success "Prerequisites check passed"
echo ""

# Step 1: Create new repository on GitHub
print_step "Step 1: Creating GitHub repository..."

if [ "$USE_GH_CLI" = true ]; then
    read -p "Create repository ${DOCS_REPO_ORG}/${DOCS_REPO_NAME} on GitHub? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        gh repo create ${DOCS_REPO_ORG}/${DOCS_REPO_NAME} \
            --public \
            --description "Documentation for Apex SDK Protocol - Unified Rust SDK for Substrate & EVM" \
            --homepage "https://apexsdk.dev" \
            || print_warning "Repository might already exist"
        print_success "Repository created"
    else
        print_warning "Skipping repository creation"
    fi
else
    print_warning "Please create the repository manually:"
    echo "  1. Go to: https://github.com/organizations/${DOCS_REPO_ORG}/repositories/new"
    echo "  2. Repository name: ${DOCS_REPO_NAME}"
    echo "  3. Description: Documentation for Apex SDK Protocol - Unified Rust SDK for Substrate & EVM"
    echo "  4. Homepage: https://apexsdk.dev"
    echo "  5. Public repository"
    echo ""
    read -p "Press Enter when repository is created..."
fi

echo ""

# Step 2: Prepare documentation directory
print_step "Step 2: Preparing documentation directory..."

# Create docs directory if it doesn't exist
mkdir -p "$DOCS_DIR"

# Initialize git if needed
if [ ! -d "${DOCS_DIR}/.git" ]; then
    cd "$DOCS_DIR"
    git init
    git branch -M main
    cd - > /dev/null
    print_success "Initialized git repository"
else
    print_warning "Git repository already initialized"
fi

echo ""

# Step 3: Copy documentation files
print_step "Step 3: Copying documentation files..."

# Copy all docs content
rsync -av --exclude='.git' docs/ "${DOCS_DIR}/" || cp -r docs/* "${DOCS_DIR}/"
print_success "Copied docs directory contents"

# Copy deployment guides
cp APEXSDK_DEV_DEPLOYMENT_GUIDE.md "${DOCS_DIR}/" 2>/dev/null || true
cp DEPLOYMENT_QUICKSTART.md "${DOCS_DIR}/" 2>/dev/null || true
cp DOMAIN_MIGRATION_SUMMARY.md "${DOCS_DIR}/" 2>/dev/null || true
print_success "Copied deployment guides"

echo ""

# Step 4: Create docs-specific files
print_step "Step 4: Creating documentation-specific files..."

# Create README.md
cat > "${DOCS_DIR}/README.md" << 'EOFREADME'
# Apex SDK Documentation

Official documentation for [Apex SDK Protocol](https://apexsdk.dev) - A unified Rust SDK for Substrate & EVM blockchain development.

## 🌐 Live Documentation

Visit [**apexsdk.dev**](https://apexsdk.dev) to view the complete documentation.

## 📚 Quick Links

- [Quick Start Guide](https://apexsdk.dev/QUICK_START.md) - Get started in 5 minutes
- [API Reference](https://apexsdk.dev/API.md) - Complete API documentation
- [CLI Guide](https://apexsdk.dev/CLI_GUIDE.md) - Command-line reference
- [Contributing](https://apexsdk.dev/CONTRIBUTING.md) - Contribution guidelines

## 🚀 Deployment

This documentation is automatically deployed to [apexsdk.dev](https://apexsdk.dev) via Cloudflare Pages.

## 🔗 Related Repositories

- [apex-sdk](https://github.com/eurybits/apex-sdk) - Main SDK repository

## 📄 License

Documentation: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)
Code examples: [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)

---

**Visit [apexsdk.dev](https://apexsdk.dev) for the complete documentation**
EOFREADME

print_success "Created README.md"

# Create .gitignore
cat > "${DOCS_DIR}/.gitignore" << 'EOFGITIGNORE'
# Jekyll
_site/
.sass-cache/
.jekyll-cache/
.jekyll-metadata

# macOS
.DS_Store

# IDEs
.vscode/
.idea/
*.swp
*.swo

# Logs
*.log

# Temp files
*~
.tmp/

# Node modules
node_modules/
EOFGITIGNORE

print_success "Created .gitignore"

# Create GitHub Actions workflow directory
mkdir -p "${DOCS_DIR}/.github/workflows"

# Create deployment workflow
cat > "${DOCS_DIR}/.github/workflows/deploy.yml" << 'EOFWORKFLOW'
name: Documentation Validation

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  validate:
    name: Validate Documentation
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Check required files
        run: |
          required_files=("_headers" "_redirects" "sitemap.xml" "robots.txt" "manifest.json" "index.html")
          for file in "${required_files[@]}"; do
            if [ ! -f "$file" ]; then
              echo "❌ Missing required file: $file"
              exit 1
            fi
          done
          echo "✅ All required files present"

      - name: Validate sitemap
        run: |
          if ! grep -q "apexsdk.dev" sitemap.xml; then
            echo "❌ sitemap.xml doesn't contain apexsdk.dev"
            exit 1
          fi
          echo "✅ Sitemap valid"

      - name: Validate robots.txt
        run: |
          if ! grep -q "apexsdk.dev" robots.txt; then
            echo "❌ robots.txt doesn't contain apexsdk.dev"
            exit 1
          fi
          echo "✅ robots.txt valid"

  notify:
    name: Deployment Notification
    runs-on: ubuntu-latest
    needs: validate
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - name: Notify
        run: |
          echo "📚 Documentation will be deployed to https://apexsdk.dev"
          echo "🚀 Cloudflare Pages handles automatic deployment"
EOFWORKFLOW

print_success "Created GitHub Actions workflow"

echo ""

# Step 5: Commit and push
print_step "Step 5: Committing and pushing to GitHub..."

cd "$DOCS_DIR"

# Add remote if not exists
if ! git remote | grep -q origin; then
    git remote add origin "$DOCS_REPO_URL"
    print_success "Added remote origin"
fi

# Stage all files
git add .

# Commit
git commit -m "feat: initialize apex-sdk-docs repository

This repository contains the official documentation for Apex SDK Protocol,
migrated from the main apex-sdk repository for better organization.

Features:
✅ Deployed to apexsdk.dev via Cloudflare Pages
✅ Complete documentation (20+ pages)
✅ SEO optimized with sitemap, robots.txt
✅ PWA support with manifest.json
✅ Security headers and caching via Cloudflare
✅ Subdomain shortcuts (start, api, cli, docs)

Documentation for: https://github.com/eurybits/apex-sdk" || print_warning "Nothing to commit or already committed"

# Push to GitHub
read -p "Push to GitHub? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push -u origin main
    print_success "Pushed to GitHub"
else
    print_warning "Skipped push - you can push manually later with: git push -u origin main"
fi

cd - > /dev/null

echo ""

# Step 6: Update main SDK repository
print_step "Step 6: Updating main SDK repository..."

# Create minimal docs/README.md
cat > docs/README.md << 'EOFSDK'
# Apex SDK Documentation

📚 **Official documentation has moved to a dedicated repository!**

## 🌐 View Documentation

Visit [**apexsdk.dev**](https://apexsdk.dev) for the complete documentation.

## 📖 Documentation Repository

👉 [**eurybits/apex-sdk-docs**](https://github.com/eurybits/apex-sdk-docs)

## Quick Links

- [Quick Start](https://apexsdk.dev/QUICK_START.md)
- [API Reference](https://apexsdk.dev/API.md)
- [CLI Guide](https://apexsdk.dev/CLI_GUIDE.md)

## Contributing to Documentation

Visit [apex-sdk-docs](https://github.com/eurybits/apex-sdk-docs) to contribute.
EOFSDK

print_success "Created docs/README.md redirect"

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Migration Complete! 🎉             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════╝${NC}"
echo ""

print_success "Documentation repository created at: ${DOCS_DIR}"
print_success "GitHub repository: https://github.com/${DOCS_REPO_ORG}/${DOCS_REPO_NAME}"
echo ""

print_step "Next Steps:"
echo "1. Configure Cloudflare Pages:"
echo "   - Go to: https://dash.cloudflare.com"
echo "   - Workers & Pages → Select your project"
echo "   - Settings → Builds & deployments"
echo "   - Update repository to: ${DOCS_REPO_ORG}/${DOCS_REPO_NAME}"
echo "   - Build output directory: /"
echo ""
echo "2. Verify deployment:"
echo "   - Visit: https://apexsdk.dev"
echo "   - Test subdomains: start.apexsdk.dev, api.apexsdk.dev, cli.apexsdk.dev"
echo ""
echo "3. Clean up main repository (optional):"
echo "   - Remove old docs files from apex-sdk/docs/"
echo "   - Keep only docs/README.md"
echo "   - Update main README.md"
echo ""
echo "For detailed instructions, see: DOCS_REPOSITORY_SETUP.md"
echo ""
