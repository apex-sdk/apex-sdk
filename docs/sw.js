// Apex SDK Service Worker
const CACHE_NAME = 'apex-sdk-v1';
const CACHE_ASSETS = [
    '/',
    '/index.html',
    '/viewer.html',
    '/css/modular.css',
    '/css/variables.css',
    '/css/base.css',
    '/css/layout/grid.css',
    '/css/components/navigation.css',
    '/css/components/buttons.css',
    '/css/components/cards.css',
    '/css/components/ui-elements.css',
    '/css/utilities/animations.css',
    '/css/utilities/accessibility.css',
    '/css/utilities/error-handling.css',
    '/js/main-browser.js',
    '/js/utils/error-handler.js',
    '/js/utils/performance.js',
    '/assets/logo.svg',
    '/assets/icons/moon.svg',
    '/assets/icons/sun.svg',
    '/assets/icons/github.svg'
];

// Install event - cache resources
self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then(cache => {
                return cache.addAll(CACHE_ASSETS);
            })
            .catch(error => {
                console.error('Failed to cache resources during install:', error);
            })
    );
    
    // Skip waiting to activate immediately
    self.skipWaiting();
});

// Activate event - clean up old caches
self.addEventListener('activate', event => {
    event.waitUntil(
        caches.keys()
            .then(cacheNames => {
                return Promise.all(
                    cacheNames.map(cacheName => {
                        if (cacheName !== CACHE_NAME) {
                            return caches.delete(cacheName);
                        }
                    })
                );
            })
            .then(() => {
                // Take control of all pages
                return self.clients.claim();
            })
    );
});

// Fetch event - serve from cache with network fallback
self.addEventListener('fetch', event => {
    // Skip cross-origin requests and non-GET requests
    if (!event.request.url.startsWith(self.location.origin) || event.request.method !== 'GET') {
        return;
    }

    event.respondWith(
        caches.match(event.request)
            .then(cachedResponse => {
                // Return cached response if available
                if (cachedResponse) {
                    // Update cache in background for next time
                    updateCacheInBackground(event.request);
                    return cachedResponse;
                }

                // Fetch from network
                return fetch(event.request)
                    .then(response => {
                        // Cache successful responses
                        if (response.status === 200) {
                            const responseClone = response.clone();
                            caches.open(CACHE_NAME)
                                .then(cache => {
                                    cache.put(event.request, responseClone);
                                });
                        }
                        return response;
                    })
                    .catch(error => {
                        // Network failed, try to return offline fallback
                        if (event.request.destination === 'document') {
                            return caches.match('/offline.html');
                        }
                        
                        // For images, return a placeholder
                        if (event.request.destination === 'image') {
                            return new Response(
                                '<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200"><rect width="200" height="200" fill="#f0f0f0"/><text x="50%" y="50%" text-anchor="middle" dy=".3em" fill="#999">Image unavailable</text></svg>',
                                { headers: { 'Content-Type': 'image/svg+xml' } }
                            );
                        }
                        
                        throw error;
                    });
            })
    );
});

// Background cache update function
function updateCacheInBackground(request) {
    fetch(request)
        .then(response => {
            if (response.status === 200) {
                caches.open(CACHE_NAME)
                    .then(cache => {
                        cache.put(request, response);
                    });
            }
        })
        .catch(error => {
            // Silently fail background updates
            console.log('Background cache update failed:', error);
        });
}

// Handle message from main thread
self.addEventListener('message', event => {
    if (event.data && event.data.type === 'SKIP_WAITING') {
        self.skipWaiting();
    }
});

// Periodic background sync (if supported)
if ('sync' in self.registration) {
    self.addEventListener('sync', event => {
        if (event.tag === 'background-cache-sync') {
            event.waitUntil(syncCacheInBackground());
        }
    });
}

async function syncCacheInBackground() {
    try {
        const cache = await caches.open(CACHE_NAME);
        const requests = await cache.keys();
        
        // Update cached resources
        for (const request of requests) {
            try {
                const response = await fetch(request);
                if (response.status === 200) {
                    await cache.put(request, response);
                }
            } catch (error) {
                // Continue with next request if one fails
                console.log('Failed to update cached resource:', request.url);
            }
        }
    } catch (error) {
        console.error('Background sync failed:', error);
    }
}