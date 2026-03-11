// Cloudflare Worker for caching static assets
export default {
  async fetch(request) {
    const url = new URL(request.url);
    // Example: Cache assets under /public for 1 day
    if (url.pathname.startsWith('/public/')) {
      return new Response(await fetch(request), {
        headers: { 'Cache-Control': 'public, max-age=86400' }
      });
    }
    return fetch(request);
  }
};
