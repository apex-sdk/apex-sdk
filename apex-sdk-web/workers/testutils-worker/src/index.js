// TestUtils Worker entry
addEventListener('fetch', event => {
  event.respondWith(new Response('TestUtils Worker running'));
});