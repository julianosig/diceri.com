import { defineConfig, devices } from '@playwright/test';

/**
 * End-to-end smoke tests against the real API binary and a production web build.
 * The API loads `.env` from the repository root, so LiveKit credentials there
 * (or in CI secrets) enable the media tests; without them those tests skip.
 */
const API_PORT = 18091;
const WEB_PORT = 4180;
const repoRoot = new URL('../../', import.meta.url).pathname;

export default defineConfig({
	testDir: 'tests/e2e',
	testMatch: '**/*.e2e.ts',
	fullyParallel: true,
	forbidOnly: Boolean(process.env.CI),
	retries: process.env.CI ? 1 : 0,
	timeout: 60_000,
	reporter: process.env.CI ? [['github'], ['list']] : 'list',
	use: {
		// 127.0.0.1 is a secure context, required for getUserMedia.
		baseURL: `http://127.0.0.1:${String(WEB_PORT)}`,
		permissions: ['microphone'],
		trace: 'retain-on-failure',
		launchOptions: {
			args: ['--use-fake-ui-for-media-stream', '--use-fake-device-for-media-stream']
		}
	},
	projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
	webServer: [
		{
			command: 'cargo run -q -p diceri-api',
			cwd: repoRoot,
			url: `http://127.0.0.1:${String(API_PORT)}/healthz`,
			env: { DICERI_BIND_ADDRESS: `127.0.0.1:${String(API_PORT)}` },
			timeout: 180_000,
			reuseExistingServer: false
		},
		{
			command: `pnpm build && pnpm preview --port ${String(WEB_PORT)} --strictPort`,
			url: `http://127.0.0.1:${String(WEB_PORT)}`,
			env: { DICERI_API_URL: `http://127.0.0.1:${String(API_PORT)}` },
			timeout: 120_000,
			reuseExistingServer: false
		}
	]
});
