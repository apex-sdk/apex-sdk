// Cloudflare Worker for custom API endpoint
export default {
  async fetch(request) {
    const url = new URL(request.url);
    if (url.pathname === '/api/hello') {
      return new Response(JSON.stringify({ message: 'Hello from Cloudflare Worker!' }), {
        headers: { 'Content-Type': 'application/json' }
      });
    }
    return fetch(request);
  }
};
