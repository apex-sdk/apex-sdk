#!/bin/bash
set -e

echo "building docs."

mkdir -p dist

if [ -d "docs" ]; then
    echo "copying docs files..."
    cp -r docs/* dist/
    echo "docs copied to dist/"
else
    echo "docs dir not found"
    exit 1
fi

echo "docs build completed successfully!"
echo "files ready for Cloudflare Pages deployment in dist/"