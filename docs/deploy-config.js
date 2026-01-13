/**
 * Deployment Configuration Script
 * Sets up environment-specific configuration
 */

// Option 1: Set as global variable (recommended for static sites)
function setWeb3FormsKey() {
    // This should be set during build/deployment process
    const accessKey = process.env.WEB3FORMS_ACCESS_KEY;
    
    if (accessKey) {
        // Inject into page head
        const script = document.createElement('script');
        script.textContent = `window.WEB3FORMS_ACCESS_KEY = '${accessKey}';`;
        document.head.appendChild(script);
    }
}

// Option 2: Add meta tag (for static deployment)
function addWeb3FormsKeyMeta() {
    const accessKey = process.env.WEB3FORMS_ACCESS_KEY;
    
    if (accessKey) {
        const meta = document.createElement('meta');
        meta.name = 'web3forms-key';
        meta.content = accessKey;
        document.head.appendChild(meta);
    }
}

// For development: Set key from environment
if (typeof window !== 'undefined' && !window.WEB3FORMS_ACCESS_KEY) {
    // Check if running in development
    const isDev = window.location.hostname === 'localhost' || 
                  window.location.hostname === '127.0.0.1';
    
    if (isDev) {
        console.log('Development mode: Web3Forms key should be set via localStorage or environment');
        console.log('To set: localStorage.setItem("web3forms-key", "your_key_here")');
    }
}

export { setWeb3FormsKey, addWeb3FormsKeyMeta };