# AGENTS.md — apps/web

SvelteKit 3 single-page app. Read the root [`AGENTS.md`](../../AGENTS.md) first.

## Stack

SvelteKit 3 (`adapter-static`, SPA fallback, `ssr = false`), Svelte 5 runes,
Tailwind CSS 4 (CSS-first), TypeScript ~6, Vitest 5 (node + browser projects),
Playwright. Configuration lives in `vite.config.ts`; there is no `svelte.config.js`.

The app talks to the Rust API with same-origin requests to `/api/v1/*`. In dev,
Vite proxies `/api` to `DICERI_API_URL` (default `http://127.0.0.1:8080`).

## Layout

```
src/
  routes/                     thin pages: load data, compose features, no logic
    +layout.ts                ssr = false (media APIs are browser-only)
    layout.css                Tailwind entry + design tokens (@theme)
    r/[room]/                 meeting page for a shared link
  lib/
    components/ui/            design-system primitives (Button, ...); no app logic,
                              never import from features/
    features/<feature>/       one folder per product feature
      components/             feature components
      <feature>-api.ts        calls to the Rust API
      <feature>-state.svelte.ts   runes class for feature state (when needed)
      types.ts
      index.ts                the ONLY import entry point for other code
    api/                      http client, websocket client, generated types
    utils/                    pure, framework-free helpers (cn, formatters)
```

Planned features: `room`, `lobby` (device check, language, consent), `call`
(video grid, controls), `media` (devices, LiveKit wrapper), `translation`
(language picker, captions).

## Rules

- **Imports:** `#lib/...` with the file extension (`#lib/utils/cn.ts`). Import a
  feature only via its `index.ts`. Relative imports only inside the same feature.
- **Components:** typed `$props()`; snippets instead of slots; callback props
  (`onclick`, `onchange`) instead of events; `$bindable()` only when two-way
  binding is the clear API. Pass `class` through and merge with `cn()`.
- **Variants:** use `tailwind-variants` (`tv()`) in a sibling
  `<name>-variants.ts`; no ad-hoc conditional class strings for variants.
- **Styling:** Tailwind utilities with tokens from `layout.css` (`bg-surface`,
  `text-ink-muted`, `bg-brand-500`). Add a token instead of a raw color.
  No `<style>` blocks unless Tailwind cannot express it.
- **State:** per-room/per-call state is a class with `$state`/`$derived` fields
  in a `.svelte.ts` file, provided via context from the route layout.
  Module-level `$state` only for truly global client preferences.
- **Effects:** `$effect` only to sync with external systems (MediaStream,
  LiveKit, WebSocket), always returning cleanup. Derive, don't sync.
- **Navigation:** `goto(resolve('/r/[room]', { room }))`; never build route
  strings by hand.
- **Accessibility:** every interactive element is keyboard reachable and
  labelled; toggles use `aria-pressed`; live updates (captions, join/leave,
  copy feedback) use `aria-live="polite"`. Treat Svelte a11y warnings as errors.
- **Media:** request microphone/camera only after a user gesture in the lobby;
  keep `echoCancellation`, `noiseSuppression` and `autoGainControl` on; handle
  autoplay restrictions with an explicit "join audio" action.

## Tests

| Kind                    | File                                     | Runs in                    |
| ----------------------- | ---------------------------------------- | -------------------------- |
| Pure logic, API clients | `*.spec.ts` next to the source           | Vitest `server` (node)     |
| Components              | `*.svelte.spec.ts` next to the component | Vitest `client` (Chromium) |
| User flows              | `src/routes/**/*.e2e.ts`                 | Playwright                 |

Inject `fetch` into API functions so tests pass a fake instead of mocking globals.
Browser tests need Chromium system libraries:
`pnpm exec playwright install --with-deps chromium` (requires sudo once).
