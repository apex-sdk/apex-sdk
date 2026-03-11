
// Simple API Worker with routing, logging, and error handling
addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const url = new URL(request.url);
  try {
    // Example: /api/hello?name=World
    if (url.pathname === '/api/hello') {
      const name = url.searchParams.get('name') || 'World';
      logRequest(request, 200);
      return new Response(JSON.stringify({ message: `Hello, ${name}!` }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      });
    }
    // Add more API endpoints here
    logRequest(request, 404);
    return new Response(JSON.stringify({ error: 'Not found' }), { status: 404, headers: { 'Content-Type': 'application/json' } });
  } catch (err) {
    logRequest(request, 500, err);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), { status: 500, headers: { 'Content-Type': 'application/json' } });
  }
}

function logRequest(request, status, error) {
  // In production, send to logging infra
  console.log(`[API] ${request.method} ${request.url} [${status}]${error ? ' ' + error : ''}`);
}