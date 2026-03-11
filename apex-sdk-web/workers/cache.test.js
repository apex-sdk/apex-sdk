import cacheWorker from './cache.js';
import { simulateWorkerFetch } from './testUtils.js';

describe('cache.js Worker', () => {
  it('should set cache headers for /public/*', async () => {
    const res = await simulateWorkerFetch(cacheWorker, 'https://test.com/public/file.js');
    expect(res.headers.get('Cache-Control')).toBe('public, max-age=86400');
  });

  it('should proxy non-public requests', async () => {
    const res = await simulateWorkerFetch(cacheWorker, 'https://test.com/other');
    expect(res.headers.get('Cache-Control')).not.toBe('public, max-age=86400');
  });
});
