
import { Hono } from 'hono'
import satori from 'satori'
import { initWasm, Resvg } from '@resvg/resvg-wasm'
// @ts-ignore
import wasm from '@resvg/resvg-wasm/index_bg.wasm'

// Sentry reporting for Workers (fetch-based, minimal)
const SENTRY_DSN = 'https://examplePublicKey@o0.ingest.sentry.io/0';
async function reportErrorToSentry(error: any, context: any = {}) {
    if (!SENTRY_DSN) return;
    // Minimal Sentry envelope for error reporting
    const body = JSON.stringify({
        error: error?.message || String(error),
        context,
        timestamp: Date.now(),
    });
    try {
        await fetch(SENTRY_DSN, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body,
        });
    } catch (e) {}
}


const app = new Hono()

// Sentry.init({ dsn: 'https://examplePublicKey@o0.ingest.sentry.io/0' })

// Simple in-memory rate limiter (per IP, per minute)
const rateLimitMap = new Map<string, { count: number, last: number }>()
const RATE_LIMIT = 30 // requests per minute

let wasmInitialized = false

const initializeWasm = async () => {
    if (wasmInitialized) return
    try {
        await initWasm(wasm)
        wasmInitialized = true
    } catch (err) {
        console.error('Error initializing WASM:', err)
    }
}


app.get('/', async (c) => {
    // Rate limiting (per IP)
    const ip = c.req.header('CF-Connecting-IP') || c.req.header('x-forwarded-for') || 'unknown';
    const now = Date.now();
    const rl = rateLimitMap.get(ip) || { count: 0, last: now };
    if (now - rl.last > 60_000) {
        rl.count = 0;
        rl.last = now;
    }
    rl.count++;
    rateLimitMap.set(ip, rl);
    if (rl.count > RATE_LIMIT) {
        return c.text('Rate limit exceeded', 429);
    }

    // Query params for dynamic content
    const { title = 'Apex SDK', subtitle = '', showCrate = 'true', showGithub = 'true' } = c.req.query();

    try {
        await initializeWasm();
    } catch (e) {
        await reportErrorToSentry(e, { phase: 'wasm' });
        return c.text('Wasm init failed: ' + e, 500);
    }

    let fontData: ArrayBuffer | null = null;
    try {
        const fontRes = await fetch('https://raw.githubusercontent.com/googlefonts/roboto/main/src/hinted/Roboto-Bold.ttf');
        if (!fontRes.ok) throw new Error('Font fetch failed: ' + fontRes.status);
        fontData = await fontRes.arrayBuffer();
    } catch (e) {
        // Use embedded base64 fallback
        try {
            const robotoBase64 = '';
            if (!robotoBase64) throw new Error('No embedded font');
            const binary = atob(robotoBase64);
            const len = binary.length;
            const bytes = new Uint8Array(len);
            for (let i = 0; i < len; i++) bytes[i] = binary.charCodeAt(i);
            fontData = bytes.buffer;
        } catch (err) {
            await reportErrorToSentry(e, { font: 'remote' });
            await reportErrorToSentry(err, { font: 'embedded' });
            return c.text('Font load failed: ' + e, 500);
        }
    }

    let crateVersion = '?';
    let githubTag = '?';

    try {
        const [cratesData, githubData] = await Promise.all([
            fetch('https://crates.io/api/v1/crates/apex-sdk').then((r) => r.json() as any),
            fetch('https://api.github.com/repos/apex-sdk/apex-sdk/releases/latest', {
                headers: { 'User-Agent': 'apex-sdk-og-worker' },
            }).then((r) => r.json() as any),
        ]);
        crateVersion = cratesData.crate?.max_version || '?';
        githubTag = githubData.tag_name || '?';
    } catch (e) {
        await reportErrorToSentry(e, { phase: 'data-fetch' });
        // Continue with placeholders
    }

    try {
        const svg = await satori(
            <div
                style={{
                    display: 'flex',
                    height: '100%',
                    width: '100%',
                    alignItems: 'center',
                    justifyContent: 'center',
                    flexDirection: 'column',
                    backgroundImage: 'linear-gradient(135deg, #020024 0%, #090979 35%, #00d4ff 100%)',
                    color: 'white',
                    fontFamily: 'Roboto',
                }}
            >
                <div
                    style={{
                        display: 'flex',
                        fontSize: 80,
                        fontWeight: 'bold',
                        marginBottom: 40,
                        background: 'linear-gradient(to right, #ffffff, #a5b4fc)',
                        backgroundClip: 'text',
                        color: 'transparent',
                    }}
                >
                    {title}
                </div>
                {subtitle && (
                    <div style={{ fontSize: 36, marginBottom: 30, color: '#e0e7ef', fontWeight: 500 }}>{subtitle}</div>
                )}
                <div style={{ display: 'flex', gap: '60px' }}>
                    {showCrate !== 'false' && (
                        <div
                            style={{
                                display: 'flex',
                                flexDirection: 'column',
                                alignItems: 'center',
                                padding: '20px 40px',
                                backgroundColor: 'rgba(255, 255, 255, 0.1)',
                                borderRadius: '20px',
                                border: '1px solid rgba(255, 255, 255, 0.2)',
                            }}
                        >
                            <div style={{ fontSize: 24, paddingBottom: 10, color: '#e2e8f0', display: 'flex' }}>Latest Crate</div>
                            <div style={{ fontSize: 50, fontWeight: 'bold', display: 'flex' }}>v{crateVersion}</div>
                        </div>
                    )}
                    {showGithub !== 'false' && (
                        <div
                            style={{
                                display: 'flex',
                                flexDirection: 'column',
                                alignItems: 'center',
                                padding: '20px 40px',
                                backgroundColor: 'rgba(255, 255, 255, 0.1)',
                                borderRadius: '20px',
                                border: '1px solid rgba(255, 255, 255, 0.2)',
                            }}
                        >
                            <div style={{ fontSize: 24, paddingBottom: 10, color: '#e2e8f0', display: 'flex' }}>GitHub Release</div>
                            <div style={{ fontSize: 50, fontWeight: 'bold', display: 'flex' }}>{githubTag}</div>
                        </div>
                    )}
                </div>
            </div>,
            {
                width: 1200,
                height: 630,
                fonts: [
                    {
                        name: 'Roboto',
                        data: fontData!,
                        weight: 700,
                        style: 'normal',
                    },
                ],
            }
        );

        // Try png rendering, if fails return svg
        try {
            const resvg = new Resvg(svg, {
                fitTo: { mode: 'width', value: 1200 },
            });
            const pngData = resvg.render();
            const pngBuffer = pngData.asPng();

            c.header('Content-Type', 'image/png');
            c.header('Cache-Control', 'public, max-age=3600');
            return c.body(pngBuffer as any);
        } catch (e) {
            await reportErrorToSentry(e, { phase: 'resvg' });
            c.header('Content-Type', 'image/svg+xml');
            return c.body(svg);
        }
    } catch (e) {
        await reportErrorToSentry(e, { phase: 'satori' });
        return c.text('Satori failed: ' + e, 500);
    }
});

export default app
