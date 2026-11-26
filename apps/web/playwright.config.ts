import { defineConfig, devices } from '@playwright/test';

/**
 * End-to-end smoke tests against the real API binary and a production web build.
 * The API loads `.env` from the repository root, so LiveKit credentials there
 * (or in CI secrets) enable the media tests; without them those tests skip.
 */
const API_PORT = 18091;
const AGENT_PORT = 18095;
const WEB_PORT = 4180;
const repoRoot = new URL('../../', import.meta.url).pathname;

export default defineConfig({
	testDir: 'tests/e2e',
	testMatch: '**/*.e2e.ts',
	fullyParallel: true,
	forbidOnly: Boolean(process.env.CI),
	retries: process.env.CI ? 1 : 0,
	timeout: 90_000,
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
			// Translation agent; reads provider keys from .env (or CI secrets).
			command: 'cargo run -q -p diceri-media-agent',
			cwd: repoRoot,
			url: `http://127.0.0.1:${String(AGENT_PORT)}/healthz`,
			env: {
				DICERI_AGENT_BIND_ADDRESS: `127.0.0.1:${String(AGENT_PORT)}`,
				// Own voice-clone namespace: the agent's startup sweep must not touch dev or prod.
				DICERI_DEPLOYMENT: 'e2e'
			},
			// Surfaces per-phrase latency logs in the test output.
			stdout: 'pipe',
			timeout: 600_000,
			reuseExistingServer: false
		},
		{
			command: 'cargo run -q -p diceri-api',
			cwd: repoRoot,
			url: `http://127.0.0.1:${String(API_PORT)}/healthz`,
			env: {
				DICERI_BIND_ADDRESS: `127.0.0.1:${String(API_PORT)}`,
				DICERI_AGENT_URL: `http://127.0.0.1:${String(AGENT_PORT)}`,
				// Parallel tests share one IP; the limit has its own router test.
				DICERI_WRITE_LIMIT_PER_MINUTE: '0',
				// The paid voice feature stays covered end to end while it is off in production.
				DICERI_VOICE_CLONING: 'true',
				DICERI_DEPLOYMENT: 'e2e',
				// A fresh database per run, never the dev file.
				DICERI_DATABASE_URL: 'sqlite::memory:'
			},
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
