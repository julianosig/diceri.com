# AGENTS.md — apps/web

SvelteKit 3 single-page app. Read the root [`AGENTS.md`](../../AGENTS.md) first.

## Stack

SvelteKit 3 (`adapter-static`, SPA fallback, `ssr = false`), Svelte 5 runes,
Tailwind CSS 4 (CSS-first), TypeScript ~6, Vitest 5 (node + browser projects),
Playwright. Configuration lives in `vite.config.ts`; there is no `svelte.config.js`.

The app talks to the Rust API with same-origin requests to `/api/v1/*`. In dev,
Vite proxies `/api` to `DICERI_API_URL` (default `http://127.0.0.1:8090`).

## Layout

```
src/
  routes/                     thin pages: load data, compose features, no logic
    +layout.ts                ssr = false (media APIs are browser-only)
    layout.css                Tailwind entry + design tokens (@theme)
    open/                     the owner opens their room (the address becomes /@username)
    @[username]/              joining someone's room; the owner is sent to /open
    signin/                   sign in or sign up, then back to ?next= (/open or /@user)
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
    i18n/                     interface language: locales, `i18n` state, messages/<lang>.ts
    utils/                    pure, framework-free helpers (cn, formatters)
```

Features today:

| Feature       | Responsibility                                                            |
| ------------- | ------------------------------------------------------------------------- |
| `room`        | create/look up rooms, remember the host key of rooms this browser created |
| `lobby`       | pre-join (name, mic preview), guest knock and waiting, host join requests |
| `call`        | LiveKit session, participant tiles, control bar, audio settings, stats    |
| `translation` | agent contract, caption feed, what each listener hears                    |
| `meeting`     | orchestrates one visit: pre-join → waiting → call → left                  |
| `account`     | onboarding (username check, password, mascot), sign-in, session badge     |
| `presence`    | the live board: who is online and the public rooms                        |
| `landing`     | home-page presentation (hero animation)                                   |

Dependency direction: `meeting` → `lobby` → `call` → `translation`, `room`. Never import upwards.

## Rules

- **Copy:** every user-facing string comes from `i18n.t.<section>.<key>`
  (`#lib/i18n/index.ts`). Add the key to `messages/en.ts` first; the compiler then
  lists every language that still needs it. Use logical utilities (`text-start`, `ms-`)
  so Arabic (right to left) lays out correctly.
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
- **Navigation:** `goto(resolve('/@[username]', { username }))`; never build route
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
| User flows              | `tests/e2e/*.e2e.ts`                     | Playwright                 |

Inject `fetch` into API functions so tests pass a fake instead of mocking globals.
Browser tests need Chromium system libraries:
`pnpm exec playwright install --with-deps chromium` (requires sudo once).
