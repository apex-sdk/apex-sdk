// SEO and Structured Data Management
document.addEventListener('DOMContentLoaded', () => {
    updateStructuredData();
    initIndexNow();
});

function updateStructuredData() {
    const structuredData = {
        "@context": "https://schema.org",
        "@type": "SoftwareApplication",
        "name": "Apex SDK Protocol",
        "softwareVersion": "0.1.5",
        "applicationCategory": "DeveloperApplication",
        "operatingSystem": "Cross-platform",
        "description": "Unified Rust SDK for Substrate & EVM blockchain development.",
        "offers": {
            "@type": "Offer",
            "price": "0",
            "priceCurrency": "USD"
        }
    };

    const script = document.createElement('script');
    script.type = 'application/ld+json';
    script.text = JSON.stringify(structuredData);
    document.head.appendChild(script);
}

// IndexNow Implementation
function initIndexNow() {
    // IndexNow API Key (placeholder - replace with actual key)
    const INDEX_NOW_KEY = 'YOUR_INDEXNOW_API_KEY'; 
    const HOST = 'apexsdk.dev';

    // Only run in production
    if (window.location.hostname !== HOST) return;

    // Check if we've already notified for this URL recently (e.g., last 24h)
    const lastNotify = localStorage.getItem(`indexnow_${window.location.href}`);
    const now = Date.now();

    if (!lastNotify || (now - parseInt(lastNotify)) > 86400000) {
        notifyIndexNow(window.location.href, INDEX_NOW_KEY, HOST);
        localStorage.setItem(`indexnow_${window.location.href}`, now.toString());
    }
}

function notifyIndexNow(url, key, host) {
    fetch('https://api.indexnow.org/indexnow', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json; charset=utf-8'
        },
        body: JSON.stringify({
            host: host,
            key: key,
            keyLocation: `https://${host}/${key}.txt`,
            urlList: [url]
        })
    }).catch(err => console.warn('IndexNow notification failed:', err));
}

// Google Analytics 4 (GA4) - Placeholder
// Uncomment and replace G-XXXXXXXXXX with your Measurement ID
/*
(function() {
    const script = document.createElement('script');
    script.async = true;
    script.src = 'https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX';
    document.head.appendChild(script);

    window.dataLayer = window.dataLayer || [];
    function gtag(){dataLayer.push(arguments);}
    gtag('js', new Date());
    gtag('config', 'G-XXXXXXXXXX');
})();
*/
