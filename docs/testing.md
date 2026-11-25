# Testing

Four layers, each fast enough to run at its stage. `just` recipes are the single
entry point; CI runs the same commands (`.github/workflows/ci.yml`).

| Layer | Command | What it proves | Needs | Time | Runs on |
| --- | --- | --- | --- | --- | --- |
| 1. Static | `just check` | formatting, clippy, types, ESLint, size limits, dependency policy | nothing | ~10 s | pre-push, CI |
| 2. Unit | `just test` | domain rules, use cases, adapters, every HTTP route (in-process), web logic | nothing | ~10 s | pre-push, CI |
| 3. Browser | `just test-browser` | Svelte components render and behave in Chromium | Chromium + system libs | ~10 s | CI |
| 4. End-to-end | `just e2e` | the real product: API binary + production web build + LiveKit, two browsers with fake microphones | Chromium + system libs, LiveKit credentials | ~1 min | CI, before deploy |

`just verify` = layers 1 + 2 (the pre-push gate). `just ci` = all layers.

## What the end-to-end smoke covers

- `rooms.e2e.ts`: create a meeting from the landing page, land on the shareable
  room page; unknown rooms show a not-found message. No LiveKit needed.
- `call.e2e.ts`: two participants join the same room through LiveKit with fake
  microphones; each sees the other, and audio bitrate is received; the
  round-trip (server) monitor connects. **Skips** when the API returns 503
  because LiveKit is not configured.

Media tests use the browser noise filter (`Off`), because the Krisp model needs
real speech; Krisp quality is validated manually with the audio test panel.

## Rules

- Every bug fix adds a test at the lowest layer that reproduces it.
- Layers 1–2 never touch the network; fakes implement ports (`application::ports`).
- Layer 4 stays a **smoke** suite: one test per critical user journey, not edge cases.
- Never mark a failing test as skipped to get green; fix it or open an issue.

## Setup

```sh
just setup        # dependencies and git hooks
just browsers     # downloads Chromium for Playwright (once)
# Chromium system libraries, once per machine (requires sudo):
sudo pnpm --filter @diceri/web exec playwright install-deps chromium
```

End-to-end media tests read `DICERI_LIVEKIT_*` from `.env` locally and from
repository secrets in CI.
