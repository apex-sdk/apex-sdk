import authWorker from './auth.js';
import { simulateWorkerFetch } from './testUtils.js';

describe('auth.js Worker', () => {
  it('should return 401 for /admin without auth', async () => {
    const res = await simulateWorkerFetch(authWorker, 'https://test.com/admin');
    expect(res.status).toBe(401);
  });

  it('should allow /admin with correct auth', async () => {
    const res = await simulateWorkerFetch(authWorker, 'https://test.com/admin', {
      headers: { Authorization: 'Bearer YOUR_TOKEN' }
    });
    expect(res.status).not.toBe(401);
  });

  it('should proxy non-admin requests', async () => {
    const res = await simulateWorkerFetch(authWorker, 'https://test.com/other');
    expect(res.status).not.toBe(401);
  });
});
