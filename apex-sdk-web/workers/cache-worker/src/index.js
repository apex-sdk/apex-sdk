
// Simple Cache Worker with routing, logging, and error handling
addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

// In-memory cache (ephemeral, for demo only)
const cache = new Map();

async function handleRequest(request) {
  const url = new URL(request.url);
  try {
    // Example: /cache/set?key=foo&value=bar
    if (url.pathname === '/cache/set') {
      const key = url.searchParams.get('key');
      const value = url.searchParams.get('value');
      if (!key || !value) {
        logRequest(request, 400);
        return new Response(JSON.stringify({ error: 'Missing key or value' }), { status: 400, headers: { 'Content-Type': 'application/json' } });
      }
      cache.set(key, value);
      logRequest(request, 200);
      return new Response(JSON.stringify({ message: 'Set', key, value }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    // Example: /cache/get?key=foo
    if (url.pathname === '/cache/get') {
      const key = url.searchParams.get('key');
      if (!key) {
        logRequest(request, 400);
        return new Response(JSON.stringify({ error: 'Missing key' }), { status: 400, headers: { 'Content-Type': 'application/json' } });
      }
      const value = cache.get(key);
      logRequest(request, 200);
      return new Response(JSON.stringify({ key, value }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    // Add more cache endpoints here
    logRequest(request, 404);
    return new Response(JSON.stringify({ error: 'Not found' }), { status: 404, headers: { 'Content-Type': 'application/json' } });
  } catch (err) {
    logRequest(request, 500, err);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), { status: 500, headers: { 'Content-Type': 'application/json' } });
  }
}

function logRequest(request, status, error) {
  // In production, send to logging infra
  console.log(`[CACHE] ${request.method} ${request.url} [${status}]${error ? ' ' + error : ''}`);
}