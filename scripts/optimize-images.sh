#!/bin/bash
# Apex SDK Image Optimization Script
# Optimizes images for web deployment

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Apex SDK Image Optimization Tool    ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""

# Check if docs/assets exists
if [ ! -d "docs/assets" ]; then
    echo -e "${RED}Error: docs/assets directory not found${NC}"
    echo "Please run this script from the apex-sdk root directory"
    exit 1
fi

# Check for required tools
MISSING_TOOLS=()

if ! command -v convert &> /dev/null; then
    MISSING_TOOLS+=("imagemagick")
fi

if ! command -v optipng &> /dev/null; then
    MISSING_TOOLS+=("optipng")
fi

if [ ${#MISSING_TOOLS[@]} -gt 0 ]; then
    echo -e "${YELLOW}⚠ Missing tools: ${MISSING_TOOLS[*]}${NC}"
    echo ""
    echo "Install with:"
    echo "  Ubuntu/Debian: sudo apt-get install imagemagick optipng webp"
    echo "  macOS: brew install imagemagick optipng webp"
    echo "  Fedora: sudo dnf install ImageMagick optipng libwebp-tools"
    echo ""
    echo -e "${YELLOW}Or use online tools:${NC}"
    echo "  1. Visit https://tinypng.com/"
    echo "  2. Upload docs/assets/apex_logo.png"
    echo "  3. Download optimized version"
    echo "  4. Replace original file"
    echo ""
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo -e "${GREEN}▶ Starting image optimization...${NC}"
echo ""

# Create backup
echo "Creating backup..."
if [ -f "docs/assets/apex_logo.png" ]; then
    cp docs/assets/apex_logo.png docs/assets/apex_logo_original.png
    echo -e "${GREEN}✓ Backup created: apex_logo_original.png${NC}"
fi

# Get original size
if [ -f "docs/assets/apex_logo.png" ]; then
    ORIGINAL_SIZE=$(du -h docs/assets/apex_logo.png | cut -f1)
    echo "Original size: $ORIGINAL_SIZE"
    echo ""
fi

# 1. Create optimized PNG versions (different sizes)
echo -e "${GREEN}▶ Creating optimized PNG versions...${NC}"

if command -v convert &> /dev/null; then
    # 512x512 for general use
    convert docs/assets/apex_logo.png \
        -resize 512x512 \
        -strip \
        -quality 90 \
        docs/assets/apex_logo_512.png
    echo "✓ Created: apex_logo_512.png"

    # 192x192 for PWA manifest
    convert docs/assets/apex_logo.png \
        -resize 192x192 \
        -strip \
        -quality 90 \
        docs/assets/apex_logo_192.png
    echo "✓ Created: apex_logo_192.png"

    # 32x32 for favicon
    convert docs/assets/apex_logo.png \
        -resize 32x32 \
        -strip \
        -quality 90 \
        docs/assets/favicon-32x32.png
    echo "✓ Created: favicon-32x32.png"

    # 16x16 for favicon
    convert docs/assets/apex_logo.png \
        -resize 16x16 \
        -strip \
        -quality 90 \
        docs/assets/favicon-16x16.png
    echo "✓ Created: favicon-16x16.png"

    # 180x180 for Apple touch icon
    convert docs/assets/apex_logo.png \
        -resize 180x180 \
        -strip \
        -quality 90 \
        docs/assets/apple-touch-icon.png
    echo "✓ Created: apple-touch-icon.png"

    # Create favicon.ico with multiple sizes
    convert docs/assets/apex_logo.png \
        -define icon:auto-resize=64,48,32,16 \
        docs/favicon.ico
    echo "✓ Created: favicon.ico"

    # Create 1200x630 Open Graph image
    convert -size 1200x630 xc:'#0f172a' \
        \( docs/assets/apex_logo.png -resize 300x300 \) -geometry +100+165 -composite \
        -pointsize 48 -fill '#f8fafc' -font Arial-Bold \
        -annotate +450+300 'Apex SDK Protocol' \
        -pointsize 24 -fill '#cbd5e1' \
        -annotate +450+350 'Unified Rust SDK for Substrate & EVM' \
        docs/assets/og-image.png 2>/dev/null || \
    convert -size 1200x630 xc:'#0f172a' \
        \( docs/assets/apex_logo.png -resize 300x300 \) -geometry +100+165 -composite \
        docs/assets/og-image.png
    echo "✓ Created: og-image.png (Open Graph)"
else
    echo -e "${YELLOW}⚠ Skipping PNG creation (imagemagick not installed)${NC}"
fi

echo ""

# 2. Optimize PNG files
echo -e "${GREEN}▶ Optimizing PNG files...${NC}"

if command -v optipng &> /dev/null; then
    for file in docs/assets/*.png docs/favicon.ico; do
        if [ -f "$file" ]; then
            echo "Optimizing: $(basename $file)"
            optipng -o7 -quiet "$file" 2>/dev/null || optipng -o2 "$file" 2>/dev/null || true
        fi
    done
    echo -e "${GREEN}✓ PNG optimization complete${NC}"
else
    echo -e "${YELLOW}⚠ Skipping PNG optimization (optipng not installed)${NC}"
fi

echo ""

# 3. Create WebP versions
echo -e "${GREEN}▶ Creating WebP versions...${NC}"

if command -v cwebp &> /dev/null; then
    cwebp -q 85 docs/assets/apex_logo_512.png -o docs/assets/apex_logo.webp 2>/dev/null || echo "WebP creation skipped"
    echo "✓ Created: apex_logo.webp"
else
    echo -e "${YELLOW}⚠ Skipping WebP creation (cwebp not installed)${NC}"
    echo "   Install: sudo apt-get install webp (or brew install webp)"
fi

echo ""

# Show results
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║          Optimization Complete!        ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""

echo "Files created:"
ls -lh docs/assets/*.png docs/assets/*.webp docs/favicon.ico 2>/dev/null | awk '{printf "  %s  %s\n", $5, $9}' | grep -E "(apex_logo|favicon|og-image|apple-touch)" || true

echo ""
echo "Size comparison:"
if [ -f "docs/assets/apex_logo_original.png" ] && [ -f "docs/assets/apex_logo_512.png" ]; then
    ORIG_SIZE=$(stat -f%z docs/assets/apex_logo_original.png 2>/dev/null || stat -c%s docs/assets/apex_logo_original.png 2>/dev/null)
    NEW_SIZE=$(stat -f%z docs/assets/apex_logo_512.png 2>/dev/null || stat -c%s docs/assets/apex_logo_512.png 2>/dev/null)
    REDUCTION=$(echo "scale=1; 100 - ($NEW_SIZE * 100 / $ORIG_SIZE)" | bc)
    echo "  Original: $(numfmt --to=iec-i --suffix=B $ORIG_SIZE)"
    echo "  Optimized (512x512): $(numfmt --to=iec-i --suffix=B $NEW_SIZE)"
    echo -e "  ${GREEN}Reduction: ${REDUCTION}%${NC}"
fi

echo ""
echo "Next steps:"
echo "  1. Update manifest.json to use apex_logo_192.png and apex_logo_512.png"
echo "  2. Update index.html favicon links"
echo "  3. Commit changes:"
echo "     git add docs/assets/ docs/favicon.ico"
echo "     git commit -m 'perf: optimize images for web deployment'"
echo ""
