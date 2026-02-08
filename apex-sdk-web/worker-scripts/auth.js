// Cloudflare Worker for simple authentication
export default {
  async fetch(request) {
    const url = new URL(request.url);
    // Example: Protect /admin route
    if (url.pathname.startsWith('/admin')) {
      const authHeader = request.headers.get('Authorization');
      if (!authHeader || authHeader !== 'Bearer YOUR_TOKEN') {
        return new Response('Unauthorized', { status: 401 });
      }
    }
    return fetch(request);
  }
};
