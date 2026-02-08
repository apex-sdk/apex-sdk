
// Example redirect rules. In production, load from KV, JSON, or ENV.
const redirectRules = [
  // Static redirect
  { from: '/old-page', to: '/new-page', status: 301 },
  // Pattern redirect (e.g., /docs/abc -> /documentation/abc)
  { from: /^\/docs\/(.*)/, to: '/documentation/$1', status: 302 },
  // Add more rules as needed
];

function matchRedirect(url) {
  const { pathname } = new URL(url);
  for (const rule of redirectRules) {
    if (typeof rule.from === 'string' && pathname === rule.from) {
      return { to: rule.to, status: rule.status };
    }
    if (rule.from instanceof RegExp) {
      const match = pathname.match(rule.from);
      if (match) {
        // Replace $1, $2, ... in 'to' with matched groups
        let dest = rule.to;
        match.slice(1).forEach((val, idx) => {
          dest = dest.replace(`$${idx + 1}`, val);
        });
        return { to: dest, status: rule.status };
      }
    }
  }
  return null;
}

function isRedirectLoop(request, targetUrl) {
  // Prevent redirecting to self
  return new URL(request.url).pathname === new URL(targetUrl, request.url).pathname;
}

function logRedirect(request, target, status) {
  // In production, send to logging infra
  console.log(`[Redirect] ${request.url} -> ${target} [${status}]`);
}

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const match = matchRedirect(request.url);
  if (match) {
    const targetUrl = new URL(match.to, request.url).toString();
    if (isRedirectLoop(request, targetUrl)) {
      return new Response('Redirect loop detected', { status: 508 });
    }
    logRedirect(request, targetUrl, match.status);
    return Response.redirect(targetUrl, match.status);
  }
  // No redirect: pass through or return 404/other response as needed
  return new Response('No redirect rule matched', { status: 404 });
}