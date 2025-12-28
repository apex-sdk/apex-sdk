#!/bin/bash

# Deployment script for static sites
# This sets the Web3Forms access key during deployment

# Check if access key is provided
if [ -z "$WEB3FORMS_ACCESS_KEY" ]; then
    echo "Warning: WEB3FORMS_ACCESS_KEY environment variable not set"
    echo "Contact form will be disabled"
else
    echo "Setting Web3Forms access key..."
    
    # Option 1: Replace in HTML files (for simple static deployment)
    find docs -name "*.html" -type f -exec sed -i "s/__WEB3FORMS_ACCESS_KEY__/$WEB3FORMS_ACCESS_KEY/g" {} \;
    
    # Option 2: Create a key file that's loaded by config.js
    echo "window.WEB3FORMS_ACCESS_KEY = '$WEB3FORMS_ACCESS_KEY';" > docs/js/web3forms-key.js
    
    echo "Web3Forms access key configured successfully"
fi

# Build the site
echo "Building documentation site..."

# Deploy to your hosting platform
echo "Deploying..."