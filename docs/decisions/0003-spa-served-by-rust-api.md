---
status: accepted
date: 2026-11-25
---

# Client-rendered SvelteKit SPA served by the Rust API

## Context and problem statement

WebRTC, microphone and camera APIs exist only in the browser. All server logic
(rooms, tokens, translation control) is in Rust. Running a Node server only for
SSR adds a process and a class of state-leak bugs with little benefit.

## Considered options

- `adapter-static` with SPA fallback, `ssr = false`
- `adapter-node` with SSR

## Decision outcome

Chosen option: "`adapter-static` with `fallback: '200.html'`", served by the
API (or a CDN). The web app calls `/api/v1` same-origin; Vite proxies it in dev.

### Consequences

- Good, because there is no Node process in production.
- Bad, because no server-rendered link previews (OG tags) per room; revisit with
  `adapter-node` if sharing previews become a requirement.

### Confirmation

`pnpm build` produces a static `build/` directory; e2e tests run against it.
