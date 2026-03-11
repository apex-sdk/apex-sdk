import apiWorker from './api.js';
import { simulateWorkerFetch } from './testUtils.js';
describe('api.js Worker', () => {
  it('should return hello message for /api/hello', async () => {
    const res = await simulateWorkerFetch(apiWorker, 'https://test.com/api/hello');
    expect(res.status).toBe(200);
    const json = await res.json();
    expect(json).toEqual({ message: 'Hello from Cloudflare Worker!' });
  });

  it('should proxy other requests', async () => {
    const res = await simulateWorkerFetch(apiWorker, 'https://test.com/other');
    expect(res.status).not.toBe(404); // Should not 404, should proxy
  });
});
