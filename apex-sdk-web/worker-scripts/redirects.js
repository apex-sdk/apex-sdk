// Cloudflare Worker for handling redirects
export default {
  async fetch(request) {
    const url = new URL(request.url);
    // Example: Redirect /docs to /documentation
    if (url.pathname.startsWith('/docs')) {
      return Response.redirect(url.origin + '/documentation', 301);
    }

    return fetch(request);
  }
};
