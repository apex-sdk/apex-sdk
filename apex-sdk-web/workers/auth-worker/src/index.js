
// Simple Auth Worker with routing, logging, and error handling
addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const url = new URL(request.url);
  try {
    // Example: /auth/login (POST)
    if (url.pathname === '/auth/login' && request.method === 'POST') {
      // Placeholder: parse credentials, validate, issue token
      logRequest(request, 200);
      return new Response(JSON.stringify({ token: 'fake-jwt-token' }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      });
    }
    // Example: /auth/logout (POST)
    if (url.pathname === '/auth/logout' && request.method === 'POST') {
      logRequest(request, 200);
      return new Response(JSON.stringify({ message: 'Logged out' }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      });
    }
    // Add more auth endpoints here
    logRequest(request, 404);
    return new Response(JSON.stringify({ error: 'Not found' }), { status: 404, headers: { 'Content-Type': 'application/json' } });
  } catch (err) {
    logRequest(request, 500, err);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), { status: 500, headers: { 'Content-Type': 'application/json' } });
  }
}

function logRequest(request, status, error) {
  // In production, send to logging infra
  console.log(`[AUTH] ${request.method} ${request.url} [${status}]${error ? ' ' + error : ''}`);
}