import redirectsWorker from './redirects.js';
import { simulateWorkerFetch } from './testUtils.js';

describe('redirects.js Worker', () => {
  it('should redirect /docs to /documentation', async () => {
    const res = await simulateWorkerFetch(redirectsWorker, 'https://test.com/docs');
    expect(res.status).toBe(301);
    expect(res.headers.get('Location')).toBe('https://test.com/documentation');
  });

  it('should proxy non-docs requests', async () => {
    const res = await simulateWorkerFetch(redirectsWorker, 'https://test.com/other');
    expect(res.status).not.toBe(301);
  });
});
