#!/bin/bash
# Build script for Cloudflare Pages deployment

echo "Building Apex SDK documentation..."

# Create dist directory
mkdir -p dist

# Copy all documentation files
cp -r docs/* dist/

# Copy root level files that should be accessible
cp README.md dist/
cp LICENSE dist/
cp CONTRIBUTING.md dist/

echo "Build complete! Documentation ready for deployment."
echo "Output directory: dist/"
echo "Files copied: $(find dist -type f | wc -l)"