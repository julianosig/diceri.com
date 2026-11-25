# SvelteKit + Tailwind frontend and repository-wide engineering conventions (as of 2026-11-25)

Scope: guidance for writing AGENTS.md and scaffolding a monorepo with a SvelteKit + Tailwind web app (Node LTS, pnpm) and a Rust backend, for a Google-Meet-like app with anonymous rooms reached by shared link, WebRTC audio/video, and per-listener language and voice selection.

Method note: package versions were read directly from the npm registry (`npm view <pkg> version time`) and crates.io API on 2026-11-25. Node data came from `https://nodejs.org/dist/index.json` and the nodejs/Release `schedule.json`. These are primary sources. Items marked "Inference" are recommendations I derived; they are not quoted from a source.

---

## 1. Current versions (Nov 2026) and what changed

### Takeaway
Node 26 ("Lithium") has been **Active LTS since 2026-10-28**, so the local v26.13.1 is correct. The biggest shift is **SvelteKit 3.0 (stable 2026-10-01, now 3.4.0)**. It removes `svelte.config.js`, replaces `$lib` with `#lib`, and removes `$app/stores`. It also requires Vite 8 and **TypeScript 6**, even though npm `latest` for TypeScript is now 7.0.2. Pin TypeScript to `~6` for the Svelte toolchain.

### Cited Findings
**Runtime / package manager**
- Node release schedule: v24 "Krypton" became LTS 2025-10-28, entered maintenance 2026-10-20, and reaches end of life 2028-04-30. v26 "Lithium" started 2026-05-05, became **LTS 2026-10-28**, enters maintenance 2027-10-20, and reaches end of life 2029-04-30. v27 is in alpha (alpha 2026-10-28, planned start 2027-04-22). v25 (odd-numbered) ended 2026-06-01 — [nodejs/Release schedule.json](https://raw.githubusercontent.com/nodejs/Release/main/schedule.json)
- Latest releases: v26.13.1 (2026-11-10, lts "Lithium"), v24.22.3 (Krypton), v22.23.5 (Jod) — [nodejs.org/dist/index.json](https://nodejs.org/dist/index.json)
- pnpm latest is **12.23.0** (2026-11-23). pnpm 12.0.0 was released 2026-08-26 and 11.0.0 on 2026-04-28. The local 12.21.0 is current-major — [npm registry: pnpm](https://www.npmjs.com/package/pnpm)

**npm `latest` versions (registry, 2026-11-25)**

| Package | Latest | Notes |
|---|---|---|
| svelte | 5.59.2 | No Svelte 6 on `latest`; Svelte 5 runes is current |
| @sveltejs/kit | 3.4.0 | 3.0.0 published 2026-10-01; `version-2` tag = 2.70.4 |
| @sveltejs/vite-plugin-svelte | 7.3.2 | Kit 3 peers `^7.0.0` |
| vite | 8.3.8 | 8.0.0 released 2026-03-12; engines `^20.19.0 \|\| >=22.12.0` |
| tailwindcss / @tailwindcss/vite | 4.3.5 | v4.0.0 2025-01-21, v4.2.0 2026-02-18, v4.3.0 2026-05-08 |
| typescript | 7.0.2 (`latest`), 6.0.3 last 6.x | 7.0.2 published 2026-07-08; 7.1 beta exists |
| eslint | 10.15.0 | 10.0.0 released 2026-02-06 |
| @eslint/js | 10.0.1 | |
| typescript-eslint | 8.74.0 | |
| eslint-plugin-svelte | 3.24.1 | peer eslint `^8.57.1 \|\| ^9 \|\| ^10`, svelte `^3.37 \|\| ^4 \|\| ^5` |
| eslint-config-prettier | 10.1.8 | |
| globals | 17.14.0 | |
| prettier | 3.10.1 | |
| prettier-plugin-svelte | 4.2.0 | peer svelte `^5`, prettier `^3` |
| prettier-plugin-tailwindcss | 0.8.1 | |
| eslint-plugin-better-tailwindcss | 4.11.0 | peer tailwindcss `^3.3 \|\| ^4.1.17`, eslint up to `^10` |
| vitest / @vitest/browser | 5.1.2 | 5.0.0 released 2026-09-03 |
| vitest-browser-svelte | 3.1.0 | |
| @playwright/test | 1.65.1 | |
| svelte-check | 4.7.8 | |
| sv (CLI) | 1.7.1 | 2026-11-24 |
| @sveltejs/adapter-node | 6.0.5 | |
| @sveltejs/adapter-static | 4.0.1 | |
| @sveltejs/adapter-auto | 9.0.0 | |
| bits-ui | 2.22.0 | |
| shadcn-svelte | 1.8.2 | |
| tailwind-variants | 4.0.2 | |
| @lucide/svelte | 1.80.0 | (`lucide-svelte` 1.0.1 is the older package name) |
| runed | 0.38.0 | Runes utility library |
| mode-watcher | 1.1.0 | |
| @inlang/paraglide-js | 3.2.2 | 3.0.0 published 2026-11-03 |
| @inlang/paraglide-sveltekit | 0.16.1 | Legacy adapter package (see §2) |
| livekit-client | 2.25.0 | |
| @livekit/components-core | 0.12.17 | |
| @livekit/track-processors | 0.8.3 | |
| zod / valibot | 4.6.7 / 1.5.0 | |
| openapi-typescript / openapi-fetch | 7.13.0 / 0.17.0 | |
| @hey-api/openapi-ts | 0.99.0 | |
| @commitlint/cli / config-conventional | 21.2.4 | |
| lefthook | 2.2.3 | |
| husky | 9.1.7 | |

Source for the whole table: [npm registry](https://www.npmjs.com/), queried with `npm view` on 2026-11-25.

**Rust codegen crates (crates.io API, 2026-11-25):** ts-rs 12.0.1 (updated 2026-01-31), specta 1.0.5 (2026-11-05; specta reached 1.0 stable), utoipa 6.0.0 (2026-09-22), typeshare 1.0.5, livekit (Rust SDK) 0.12.1 — [crates.io API](https://crates.io/api/v1/crates/ts-rs)

**SvelteKit 3 requirements and breaking changes** — all from [SvelteKit 3 migration guide](https://svelte.dev/docs/kit/migrating-to-sveltekit-3):
- Requires Node ≥22.17, **TypeScript 6**, Svelte ≥5.57.1, Vite ≥8.0.12 (stable rolldown v1), and vite-plugin-svelte v7. The automated migration is `npx sv migrate sveltekit-3`. Registry peerDependencies confirm `typescript: ^6.0.0`, `vite: ^8.0.12`, and `svelte: ^5.57.1` — [npm @sveltejs/kit@3.4.0](https://www.npmjs.com/package/@sveltejs/kit)
- **`svelte.config.js` is no longer supported.** Options move into the `sveltekit()` plugin in `vite.config.*`.
- **`$lib` is no longer generated.** Use `#lib`, declared in `package.json` `imports`, and include file extensions in imports.
- `$app/stores` removed; use `$app/state` (no `$` prefix). `$app/environment` → `$app/env`. `$env/*` deprecated in favor of `$app/env/private` and `$app/env/public`.
- `$app/paths`: `base`, `assets`, and `resolveRoute` are removed; use `resolve()` and `asset()`.
- `tsconfig.json` extends `$app/tsconfig` and must define `include`/`exclude`.
- CSRF protection is always on (`csrf.checkOrigin` removed; use `csrf.trustedOrigins`). Cross-origin mutative requests without `Content-Type` are rejected.
- Any file with a `server` segment (`foo.server.ts`, `server.ts`) or inside any `server/` directory (except in `src/routes` and `static`) is server-only.
- Param matchers move to a single `src/params.ts` using `defineParams`, and can be Standard Schema validators.
- `invalidateAll` is deprecated in favor of `refreshAll`. `pushState`/`replaceState` are deprecated in favor of `goto(..., { shallow: true })`.
- `json()`/`text()` helpers are deprecated in favor of `Response.json()` / `new Response()`.
- Remote functions are still **experimental** (`experimental.remoteFunctions` + `compilerOptions.experimental.async`).
- adapter-node now bundles with rolldown and serves static assets only for GET/HEAD.
- Release date: SvelteKit 3 stable shipped 2026-10-01 — [Netlify changelog](https://www.netlify.com/changelog/2026-10-01-sveltekit-3/); [svelte.dev RC post](https://svelte.dev/blog/sveltekit-3-release-candidate)
- Project tree per current docs: `src/{lib,routes,service-worker/,app.html,error.html,hooks.client,hooks.server,instrumentation.server,params}`, `static/`, `tests/`, `vite.config`, and `tsconfig.json`. There is no `svelte.config.js`. Files in `src/lib` are imported via `#lib` — [SvelteKit docs: project structure](https://svelte.dev/docs/kit/project-structure)

**TypeScript 7 caveat**
- The svelte-check README says TypeScript 7 support needs the `--tsgo` flag, with TS 6 installed alongside: `npm i -D typescript@~6 @typescript/native@npm:typescript@7` — [svelte-check README 4.7.8](https://unpkg.com/svelte-check@4.7.8/README.md)
- The TypeScript team said TS 7 ("Corsa") does not support the existing Strada JS API. This is why Svelte tooling still uses TS 6 — [TypeScript blog, Dec 2025](https://devblogs.microsoft.com/typescript/progress-on-typescript-7-december-2025)

**sv CLI**
- `sv create` flags: `--template minimal|demo|library|addon`, `--types ts|jsdoc`, `--no-types`, `--add [add-ons...]`, `--no-add-ons`, `--install <pm>`, `--package-manager <pm>`, `--no-install`, `--no-dir-check`, `--from-playground` — [sv create docs](https://svelte.dev/docs/cli/sv-create)
- `sv add` add-ons: ai-tools, better-auth, drizzle, enhanced-img, devtools, eslint, experimental, mdsvex, paraglide, playwright, prettier, storybook, sveltekit-adapter, tailwindcss, vitest — [sv docs](https://svelte.dev/docs/cli/sv-create)

**Tailwind CSS**
- v4.3 (2026-05-08) added first-party scrollbar styling, more logical-property utilities, `zoom` and `tab-size` utilities, better `@variant` support, and new colors. v4.2 had no separate blog post. Tailwind Labs joined Shopify on 2026-09-09. No v5 is announced — [tailwindcss.com/blog](https://tailwindcss.com/blog)

### Inferences
- Recommended pin set for `apps/web/package.json`: `"engines": { "node": ">=26.11 <27" }` (or `^26`), `"packageManager": "pnpm@12.23.0"`, and `typescript: "~6.0.3"`. Do **not** take TS 7 as the main `typescript` dependency until Kit/svelte-check peer it. If faster CI type-checking is wanted, use the optional `@typescript/native` alias with `svelte-check --tsgo`.
- Add `.nvmrc` / `.node-version` containing `26`. pnpm's `packageManager` field plus Corepack (or `pnpm self-update`) pins the package manager.
- **Outdated advice to flag** (common in tutorials and in LLM training data):
  - `svelte.config.js`, `$lib/...` imports, and `$app/stores` / `$page` are all pre-Kit-3.
  - `export let` props, `$:` reactive statements, `on:click`, `<slot>`, `createEventDispatcher`, and writable stores for local state are Svelte 4-era patterns.
  - `tailwind.config.js` with `content: []`, `@tailwind base/components/utilities`, and the `postcss`/`autoprefixer` setup are Tailwind v3-era.
  - `.eslintrc.*` is pre-flat config; ESLint 10 is flat-only.
  - `@inlang/paraglide-sveltekit` with `<ParaglideJS>` is the Paraglide 1.x adapter.
- The shadcn-svelte installation page still shows `$lib` aliases and `svelte.config.js` — [shadcn-svelte install](https://shadcn-svelte.com/docs/installation/sveltekit). Expect friction with Kit 3's `#lib`. Verify that shadcn-svelte 1.8.2's CLI writes `#lib/...` aliases (or configure `components.json` manually) before adopting it.

### Gaps
- I did not verify typescript-eslint 8.74's declared support for ESLint 10 and TS 6 (peer range not checked). Run `pnpm why`, or check for install warnings.
- I could not load the "What's new in Svelte – November 2026" post (404). I did not enumerate Svelte 5.4x–5.59 feature additions (for example, async SSR status) beyond `createContext` (5.40+).
- I did not read the exact `sveltekit({...})` option names in `vite.config` from the docs. The migration guide only lists added and removed options.

---

## 2. SvelteKit project organization, state, adapters, types, components, i18n, testing

### Takeaway
Use Svelte 5 runes everywhere. Put shared reactive logic in `*.svelte.ts` classes or functions, and pass per-room or per-session state through `createContext`, not module singletons. Use `#lib` imports with explicit extensions. Keep server-only code in `server/` folders.

Because the backend is Rust and the core experience (WebRTC) is client-only, prefer **adapter-static** (prerendered marketing pages plus SPA fallback for `/r/[roomId]`). Pick adapter-node only if you need SSR, such as dynamic per-room link previews.

### Cited Findings
- Runes have made stores "much less necessary". For shared logic and state, use `$state` objects in `.svelte.js`/`.svelte.ts` files. Stores remain useful for "complex asynchronous data streams" or manual control over updates — [Svelte docs: stores](https://svelte.dev/docs/svelte/stores)
- `createContext` (Svelte 5.40+) returns a type-safe `[get, set, has]` triplet and is preferred over `setContext`/`getContext`. **Warning:** if you mutate module-level shared state (for example, an exported `$state` from a `.svelte.js` module) during SSR, it can leak to the next user. Context "is not shared between requests" — [Svelte docs: context](https://svelte.dev/docs/svelte/context)
- Server-only enforcement: any `server/` directory or `*.server.*` file is blocked from client import — [SvelteKit project structure](https://svelte.dev/docs/kit/project-structure); [migration guide](https://svelte.dev/docs/kit/migrating-to-sveltekit-3)
- Env: use `$app/env/private` and `$app/env/public`, with `defineEnvVars` from `@sveltejs/kit/env`. `$env/*` is deprecated — [migration guide](https://svelte.dev/docs/kit/migrating-to-sveltekit-3)
- Paraglide JS is the official SvelteKit i18n integration (`npx sv add paraglide`) — [paraglidejs.com/sveltekit](https://paraglidejs.com/sveltekit). Setup:
  - Add `paraglideVitePlugin({ project: './project.inlang', outdir: './src/lib/paraglide', strategy: [...] })`.
  - Put `%lang%`/`%dir%` placeholders in `app.html`.
  - Add `paraglideMiddleware` in `hooks.server.ts`. Kit 3 makes `event.request` readonly, so pass a copied event.
  - Export `reroute` from `src/hooks.ts` using `deLocalizeUrl`.
  - Messages live in `messages/{locale}.json`; call them as `m.key()`.
  - For SSG with adapter-static SPA fallback, set `paths.relative = false`.
  - Include the `cookie` strategy if a stored preference must affect the first request.
- Paraglide JS 3.0.0 was published 2026-11-03; the latest is 3.2.2 — [npm](https://www.npmjs.com/package/@inlang/paraglide-js)
- shadcn-svelte init: `npx shadcn-svelte@latest init`, then `add button`. Components go under `components/ui`, with `utils` and `hooks` aliases — [shadcn-svelte docs](https://shadcn-svelte.com/docs/installation/sveltekit)
- adapter-static with SPA fallback is documented as compatible with Paraglide when `paths.relative` is false — [paraglidejs.com/sveltekit](https://paraglidejs.com/sveltekit)

### Inferences (recommendations)
**Adapter choice.** Use `@sveltejs/adapter-static` with `fallback: '200.html'`. Prerender `/`, `/about`, and `/legal`. Mark `src/routes/r/[room]/+layout.ts` with `export const ssr = false`.

Then:
- The Rust server (axum, for example) or a CDN serves the build directory.
- Everything else (signaling, room tokens, translation control) goes through the Rust API over fetch or WebSocket.
- There is no Node process in production, and no SSR state-leak class of bugs.

Choose adapter-node instead only if you need per-room dynamic OG tags or server-side auth cookies handled by SvelteKit.

**Types from Rust.**
- *Preferred: one OpenAPI contract.* Generate it with `utoipa` 6 in Rust and run `openapi-typescript` + `openapi-fetch` on the TS side. This also covers REST validation.
- *Lighter alternative:* `ts-rs` 12 or `specta` 1.0 to export WebSocket/DataChannel message enums directly. The signaling and translation-control protocol is message-based, so a discriminated-union export (serde `#[serde(tag = "type")]`) fits well.
- Commit generated files to `packages/protocol/` (or `apps/web/src/lib/api/generated/`), and add a CI check that regenerating produces no diff.

**Component patterns.**
- Typed props: `let { variant = 'default', children, ...rest }: Props = $props();`
- Composition: snippets (`{#snippet}` / `{@render}`) instead of slots, and callback props (`onclick`) instead of `createEventDispatcher`.
- Bindable props: `$bindable()` for two-way binding.
- Variants: `tailwind-variants` (`tv()`), which shadcn-svelte also uses. Class merging: `cn()` = clsx + tailwind-merge.
- Headless accessible primitives: bits-ui 2.x. Icons: `@lucide/svelte`. Runes utilities (`watch`, `useEventListener`, `PersistedState`): runed.

**Accessibility.**
- eslint-plugin-svelte's a11y rules are compiler warnings, so treat them as errors in CI.
- Video tiles need `aria-label` with the participant name.
- Mute and camera toggles should use `aria-pressed`.
- Add live regions (`aria-live="polite"`) for join/leave and for caption or translation text.
- Run Playwright + `@axe-core/playwright` smoke tests.

**Folder tree for `apps/web`** (Kit 3 conventions):
```
apps/web/
├─ package.json            # "imports": { "#lib/*": "./src/lib/*" }, engines, scripts
├─ vite.config.ts          # sveltekit({...}), tailwindcss(), paraglideVitePlugin()
├─ tsconfig.json           # extends "$app/tsconfig"; include/exclude
├─ eslint.config.js
├─ project.inlang/  messages/{en,es,fr,...}.json
├─ static/                 # robots.txt, favicon only
├─ tests/e2e/              # Playwright specs (*.e2e.ts)
└─ src/
   ├─ app.html  app.css  app.d.ts  error.html
   ├─ hooks.server.ts  hooks.ts  params.ts
   ├─ routes/
   │  ├─ +layout.svelte  +layout.ts  +page.svelte     # landing: "New meeting" / "Join"
   │  └─ r/[room=roomId]/
   │     ├─ +layout.ts      # ssr = false
   │     ├─ +page.svelte    # thin: composes features/call
   │     └─ lobby/+page.svelte  # pre-join: device check, name, language
   └─ lib/
      ├─ components/ui/     # design-system primitives (button, dialog, select, tooltip...) — no app logic
      ├─ features/
      │  ├─ room/           # join/create, share link, participants list
      │  │  ├─ components/  RoomHeader.svelte, ShareLinkDialog.svelte
      │  │  ├─ room.svelte.ts   # RoomState class (runes) + createContext
      │  │  └─ room.test.ts
      │  ├─ call/           # video grid, tile, controls bar
      │  ├─ media/          # devices.svelte.ts, getUserMedia wrappers, audio-worklets/
      │  ├─ translation/    # language/voice pickers, captions, per-listener prefs
      │  └─ settings/
      ├─ api/               # typed client (openapi-fetch), ws client, generated/ types
      ├─ paraglide/         # generated (gitignored or committed — pick one)
      ├─ utils/             # cn(), formatters — pure, framework-free
      └─ server/            # (only if adapter-node) server-only helpers
```
Rules for this tree:
- Routes stay thin and compose features.
- Features may import `components/ui`, `api`, and `utils`, but not each other's internals. Each feature exports an `index.ts`.
- `ui` imports nothing from `features`.

**State.**
- One `RoomState` class per joined room, created in the room layout and provided via `createContext`. Fields use `$state`/`$derived`, and the class has `connect()` and `dispose()` methods.
- Module-level `$state` is allowed only for truly global, client-only preferences, such as the theme or the last-used device IDs persisted to localStorage.
- Use `$effect` only for syncing with external systems (MediaStream, LiveKit room, WebSocket), and always return cleanup.

**Load functions.** With an SPA and a Rust backend, use universal `+page.ts` loads only for cheap validation, such as the room ID format via `params.ts` matchers. Fetch room metadata from Rust.

**Testing.**
- Vitest 5 runs in two projects:
  - `unit`: node environment, for `*.test.ts`.
  - `client`: `@vitest/browser` + Playwright provider + `vitest-browser-svelte`, for `*.svelte.test.ts`. This is the sv-addon default pattern.
- Playwright 1.65 for e2e. Use Chromium flags `--use-fake-ui-for-media-stream` and `--use-fake-device-for-media-stream` to test camera and mic, plus two browser contexts to simulate two participants.

**Tailwind v4 CSS-first** (`src/app.css`):
```css
@import "tailwindcss";
@plugin "@tailwindcss/forms";            /* optional */
@custom-variant dark (&:where(.dark, .dark *));
@theme {
  --color-brand-500: oklch(0.62 0.19 255);
  --font-sans: "Inter Variable", ui-sans-serif, system-ui;
  --radius-tile: 1rem;
}
```
Register the `@tailwindcss/vite` plugin in `vite.config.ts`. You need no `tailwind.config.js`, no PostCSS config, and no autoprefixer.

### Gaps
- I did not find official Kit 3 guidance on the exact `package.json` `imports` entry that `sv create` generates for `#lib`. Verify by scaffolding.
- I did not verify that shadcn-svelte 1.8.x supports Kit 3 `#lib` (its docs are stale; see §1).
- I did not fetch the vitest-browser-svelte docs or the current sv vitest add-on output.

---

## 3. WebRTC / media in the browser with Svelte (LiveKit, getUserMedia, AudioWorklet, devices)

### Takeaway
There is **no official LiveKit Svelte SDK**. Use `livekit-client` 2.25 directly, wrapped in a runes class (`RoomState`) that subscribes to `RoomEvent`s. Attach tracks with `track.attach()` (needed for adaptiveStream) inside a Svelte attachment or `$effect`. Expose capture constraints (`echoCancellation`, `noiseSuppression`, `autoGainControl`, `voiceIsolation`) through `AudioCaptureOptions`.

### Cited Findings
- No official livekit-client + Svelte 5 runes example exists. An unofficial community "livekit-svelte" Svelte 5 SDK exists; check its maintenance before relying on it — [sveltethemes: livekit-svelte](https://sveltethemes.dev/powxenv/livekit-svelte); [livekit/client-sdk-js](https://github.com/livekit/client-sdk-js)
- Subscribing — [LiveKit docs: subscribe](https://docs.livekit.io/home/client/tracks/subscribe/):
  - Listen to `RoomEvent.TrackSubscribed` (track, publication, participant) and call `track.attach()` (or pass an element).
  - adaptiveStream picks the simulcast layer from element size and visibility, pauses hidden tracks, and **requires `track.attach()` in JS**.
  - Selective subscription: `autoSubscribe: false` + `publication.setSubscribed(true)`.
  - `publication.setEnabled(false)` stops data without renegotiation, and `setVideoQuality(VideoQuality.LOW)` suits thumbnails.
  - Audio volume is controlled with `track.setVolume(0–1)`.
- `AudioCaptureOptions` (SDK v2.25.0) — [LiveKit JS reference](https://docs.livekit.io/reference/client-sdk-js/interfaces/AudioCaptureOptions.html):
  - Constraints: `autoGainControl`, `channelCount`, `deviceId`, `echoCancellation`, `latency`, `noiseSuppression`, `sampleRate`, `sampleSize`.
  - `processor`: a TrackProcessor.
  - `restrictOwnAudio`: experimental, Chromium-only.
  - `voiceIsolation`: experimental, limited browser support. When supported, it overrides `noiseSuppression`.
- Latest packages: `livekit-client` 2.25.0, `@livekit/components-core` 0.12.17 (framework-agnostic observables used by the React/Vue component kits), and `@livekit/track-processors` 0.8.3. The Rust `livekit` crate is 0.12.1 — [npm](https://www.npmjs.com/package/livekit-client); [crates.io](https://crates.io/crates/livekit)

### Inferences (recommendations)
**Per-listener translation implies selective subscription.** Each listener subscribes to the translated-audio track for their chosen language and voice, for example one track per (speaker, language), published by a Rust or AI agent participant. Mute or skip the original-language audio as needed.

- Use `autoSubscribe: false` and call `setSubscribed()` based on the listener's language preference. Store the preference in `features/translation/prefs.svelte.ts`, persisted.
- An alternative is LiveKit track names or attributes encoding `lang` and `voice`, filtered client-side.

**Svelte integration pattern.**
```ts
// features/call/room.svelte.ts
export class RoomState {
  participants = $state<RemoteParticipant[]>([]);
  connection = $state<ConnectionState>('disconnected');
  #room = new Room({ adaptiveStream: true, dynacast: true });
  async connect(url: string, token: string) { /* bind RoomEvent handlers → reassign $state */ }
  dispose() { this.#room.disconnect(); }
}
export const [getRoom, setRoom] = createContext<RoomState>();
```
- LiveKit objects mutate internally, so reassign arrays or snapshot them on events instead of making Room objects deeply reactive. `$state.raw` is appropriate for SDK objects.
- Attach video with `{@attach (el) => { track.attach(el); return () => track.detach(el); }}` (a Svelte 5 attachment) on a `<video>` element.

**Device handling.**
- Call `getUserMedia` only after a user gesture in the lobby.
- Enumerate devices after permission is granted, because labels are empty before that.
- Listen to `navigator.mediaDevices` `devicechange`, and switch via `room.switchActiveDevice(kind, deviceId)`.
- Persist the chosen deviceIds.
- Handle `NotAllowedError`, `NotFoundError`, and `NotReadableError` with distinct UI.
- Handle the autoplay policy with `room.startAudio()` behind a "Click to enable audio" banner when `RoomEvent.AudioPlaybackStatusChanged` fires.

**Echo cancellation.**
- Keep `echoCancellation: true`, `noiseSuppression: true`, and `autoGainControl: true` as defaults for the speaker mic.
- Offer an opt-in "voice isolation" toggle where supported.
- If translated TTS audio is played locally, note that browser AEC only cancels audio rendered through the same page or output device. Test with headphones off.

**AudioWorklet.** Use it for local VU meters or client-side voice activity detection. Put worklet sources in `src/lib/features/media/worklets/*.ts`, import them with Vite's `?worker&url` / `?url`, then call `audioContext.audioWorklet.addModule(url)`. Keep the processing in the Rust backend or agent, not the browser, unless latency demands otherwise.

### Gaps
- The LiveKit page I fetched did not document `getLocalDevices`, `switchActiveDevice`, `startAudio`, or defaults for capture options. The method names above come from SDK familiarity and should be checked against the v2.25 reference.
- I did not research whether the project will use LiveKit vs. a custom Rust SFU (webrtc-rs / str0m). That is out of scope for this note.
- I did not research current browser support for `voiceIsolation` (MDN not fetched).

---

## 4. Repository conventions (AGENTS.md, commits, hooks, limits, naming, ADRs, monorepo, tooling)

### Takeaway
AGENTS.md is plain Markdown with no required schema. Nest one per package, because the closest file wins.

- Enforce Conventional Commits with commitlint run from lefthook.
- Enforce file and function size limits with ESLint `max-lines` / `max-lines-per-function` and clippy thresholds.
- Record decisions as MADR 4.0 files in `docs/decisions/NNNN-title.md`.

### Cited Findings
- AGENTS.md — [agents.md](https://agents.md/):
  - It is an open, plain-Markdown "README for agents", used by 60k+ projects and stewarded by the Agentic AI Foundation (Linux Foundation).
  - Suggested sections: project overview, build/test commands, code style, testing instructions, security considerations, commit/PR guidelines, deployment.
  - There are no required fields. In monorepos, put an AGENTS.md per package; the closest file wins, and explicit user prompts override it. OpenAI's main repo has 88 of them.
  - Guidance: list test commands explicitly, because agents run them. To keep backward compatibility with tools that look for another name (for example CLAUDE.md), rename existing docs to AGENTS.md and symlink the old name.
- Conventional Commits 1.0.0 — [conventionalcommits.org](https://www.conventionalcommits.org/en/v1.0.0/):
  - Format: `<type>[optional scope][!]: <description>`, then an optional body and footers, each separated by a blank line.
  - `feat` maps to a MINOR release and `fix` to a PATCH. `!` or a `BREAKING CHANGE:` footer maps to a MAJOR release.
  - Scope is a noun in parentheses.
  - `@commitlint/config-conventional` adds `build`, `chore`, `ci`, `docs`, `style`, `refactor`, `perf`, `test` (and `revert`).
- lefthook supports `parallel: true` jobs with `glob`, `{staged_files}`, and `stage_fixed: true` to re-stage auto-fixes, and installs via npm — [lefthook.dev](https://lefthook.dev/). Versions: lefthook 2.2.3, husky 9.1.7 — npm registry.
- ESLint size rules — [ESLint max-lines-per-function](https://eslint.org/docs/rules/max-lines-per-function); [ESLint max-lines](https://eslint.org/docs/rules/max-lines):
  - `max-lines-per-function` defaults to 50 and is not in `recommended`.
  - `max-lines` limits lines per file.
  - Angular Universal used `max-lines-per-function: 200` and disabled it for specs.
  - Biome's equivalent is also opt-in — [Biome](https://biomejs.dev/linter/rules/no-excessive-lines-per-function).
- MADR 4.0.0 (2024-09-17) — [adr.github.io/madr](https://adr.github.io/madr/):
  - Files are `docs/decisions/NNNN-title-with-dashes.md`.
  - Sections: optional front matter (status, date, decision-makers, consulted, informed); Title; Context and Problem Statement; Decision Drivers; Considered Options; Decision Outcome (Consequences, Confirmation); Pros and Cons of the Options; More Information.
  - "Bare" and "minimal" variants exist.

### Inferences (recommendations)
**Monorepo layout** (pnpm workspace + Cargo workspace side by side):
```
/
├─ AGENTS.md                 # repo-wide rules (+ CLAUDE.md -> AGENTS.md symlink)
├─ README.md  LICENSE  .editorconfig  .gitattributes  .gitignore
├─ justfile                  # single entrypoint: just dev | check | test | fmt | gen
├─ package.json  pnpm-workspace.yaml  pnpm-lock.yaml  .npmrc  .node-version
├─ Cargo.toml  Cargo.lock  rust-toolchain.toml  rustfmt.toml  clippy.toml  deny.toml
├─ lefthook.yml  commitlint.config.js  renovate.json
├─ apps/
│  ├─ web/   (+ AGENTS.md)   # SvelteKit
│  └─ api/   (+ AGENTS.md)   # Rust binary crate (axum), serves API + static web build
├─ crates/                   # Rust libs: protocol/, media/, translation/ ...
├─ packages/                 # TS libs: protocol-ts (generated types), eslint-config?
├─ docs/decisions/           # MADR ADRs
├─ docs/research/
└─ .github/workflows/        # ci.yml: lint, typecheck, test, build, codegen-diff
```
`apps/` + `crates/` + `packages/` is a common split and keeps Cargo and pnpm globs simple (`members = ["apps/api", "crates/*"]`; `packages: ["apps/web", "packages/*"]`).

**Task runner.** Use `just` with recipes that call `pnpm -r` and `cargo`. It is cross-language and avoids Makefile tab and .PHONY quirks. Keep the `package.json` scripts as the source of truth for JS.

**Numeric limits** (suggested; there is no authoritative standard — see Gaps):

| Target | Warn | Error | Notes |
|---|---|---|---|
| `.ts` file (`max-lines`, skipBlankLines+skipComments) | 250 | 400 | generated code excluded |
| `.svelte` component (whole file) | 200 | 300 | split into child components or `.svelte.ts` logic |
| function (`max-lines-per-function`) | 50 | 80 | off for `*.test.ts` and `*.e2e.ts` |
| `complexity` | — | 12 | |
| `max-depth` / `max-params` | — | 4 / 4 | use options objects |
| Rust file | ~400 | 600 (CI script) | clippy has no per-file limit |
| Rust fn (`clippy::too_many_lines`, `too-many-lines-threshold`) | — | 80 | set in `clippy.toml` |
| `clippy::cognitive_complexity` threshold | — | 15 | |
| items per folder | 10–12 | 15 | custom CI script (e.g., `find -maxdepth 1`), no standard lint |
| Line width | Prettier `printWidth: 100`, rustfmt `max_width = 100` | | |

**ESLint flat config sketch** (`apps/web/eslint.config.js`):
```js
import js from '@eslint/js';
import ts from 'typescript-eslint';
import svelte from 'eslint-plugin-svelte';
import prettier from 'eslint-config-prettier';
import globals from 'globals';

export default ts.config(
  { ignores: ['.svelte-kit/', 'build/', 'src/lib/paraglide/', '**/generated/'] },
  js.configs.recommended,
  ...ts.configs.strictTypeChecked,
  ...svelte.configs.recommended,
  prettier,
  ...svelte.configs.prettier,
  {
    languageOptions: {
      globals: { ...globals.browser },
      parserOptions: { projectService: true, extraFileExtensions: ['.svelte'], parser: ts.parser },
    },
    rules: {
      'max-lines': ['error', { max: 400, skipBlankLines: true, skipComments: true }],
      'max-lines-per-function': ['error', { max: 80, skipBlankLines: true, skipComments: true }],
      complexity: ['error', 12],
      'max-depth': ['error', 4],
      'max-params': ['error', 4],
      'no-restricted-imports': ['error', { patterns: ['$lib/*', '#lib/features/*/!(index)*'] }],
    },
  },
  { files: ['**/*.svelte'], rules: { 'max-lines': ['error', { max: 300, skipBlankLines: true, skipComments: true }] } },
  { files: ['**/*.test.ts', '**/*.e2e.ts'], rules: { 'max-lines-per-function': 'off' } },
);
```
Note: ESLint `max-lines-per-function` on a `.svelte` file measures functions inside `<script>`; `max-lines` counts the whole component. The `no-restricted-imports` pattern for feature internals is illustrative and should be tuned.

**Prettier** (`.prettierrc`): `{ "useTabs": true, "singleQuote": true, "printWidth": 100, "plugins": ["prettier-plugin-svelte", "prettier-plugin-tailwindcss"], "overrides": [{ "files": "*.svelte", "options": { "parser": "svelte" } }], "tailwindStylesheet": "./apps/web/src/app.css" }`. Tailwind v4 requires `tailwindStylesheet` instead of a JS config path. prettier-plugin-tailwindcss must be listed last.

**Commit rules for AGENTS.md:**
- Conventional Commits, enforced by commitlint (`extends: ['@commitlint/config-conventional']`).
- Scopes are an enum: `web`, `api`, `protocol`, `media`, `translation`, `infra`, `docs`, `deps`.
- Header ≤ 72 chars, imperative mood, lowercase subject, no trailing period.
- The body explains *why* and wraps at 100.
- **Atomic, micro-commits:** one logical change per commit. Each commit builds and passes `just check`. Never mix refactors with behavior changes or formatting with logic. Generated code goes in the same commit as the source change that produced it.
- Use `!` and a `BREAKING CHANGE:` footer for protocol changes between web and api.

**lefthook.yml sketch:**
```yaml
pre-commit:
  parallel: true
  jobs:
    - name: web-lint
      root: apps/web/
      glob: "*.{ts,js,svelte,css}"
      run: pnpm exec eslint --fix {staged_files} && pnpm exec prettier --write {staged_files}
      stage_fixed: true
    - name: rust-fmt
      glob: "*.rs"
      run: cargo fmt --all
      stage_fixed: true
commit-msg:
  jobs:
    - run: pnpm exec commitlint --edit {1}
pre-push:
  jobs:
    - run: just check     # svelte-check, clippy -D warnings, unit tests
```
Prefer lefthook over husky + lint-staged for polyglot repos: it is a single config, runs jobs in parallel, has `root:` per app, and runs from a binary, not Node.

**Naming.**
- Svelte components: `PascalCase.svelte`.
- Runes modules: `kebab-case.svelte.ts`. Other TS: `kebab-case.ts`.
- Tests: `*.test.ts`, `*.svelte.test.ts`, and `*.e2e.ts`.
- Routes use SvelteKit `+page`/`+layout` names.
- Rust: snake_case modules and files, CamelCase types.
- Branches: `type/short-desc`.
- CSS tokens: `--color-*`, `--radius-*` in `@theme`.

**English-only.** All code, identifiers, comments, commit messages, ADRs, and docs are in English. UI strings live only in Paraglide `messages/*.json` and are never hard-coded; ESLint can flag raw text in markup (`svelte/no-raw-text`-style rules or an i18n lint).

**.editorconfig:** `root = true`; `[*]` utf-8, lf, insert_final_newline, trim_trailing_whitespace; `[*.{ts,js,svelte,css,json}]` indent_style = tab; `[*.rs]` indent 4 spaces; `[*.{yml,yaml,md}]` 2 spaces.

**Dependency updates.** Renovate is preferable to Dependabot for a pnpm + Cargo monorepo. Group the Svelte, Tailwind, and Vite ecosystems, plus Rust crates, and automerge patch updates with a `minimumReleaseAge` of about 3 days for supply-chain safety. Dependabot is the simpler GitHub-native fallback.

**AGENTS.md section outline (root):**
1. Project overview: product in 3 lines, architecture diagram (web ↔ api ↔ SFU/LiveKit ↔ translation pipeline), links to ADRs.
2. Repo map: the monorepo tree above, plus what lives where, with a pointer to nested AGENTS.md files.
3. Toolchain and versions: Node 26 LTS, pnpm 12, Rust toolchain pin, TS ~6 (not 7), Kit 3, Svelte 5 runes, Tailwind 4.
4. Commands: `just setup | dev | check | test | fmt | gen | build`. Agents must run `just check` before finishing.
5. Code style and hard rules:
   - Runes only.
   - No `$lib`, `svelte.config.js`, `$app/stores`, `export let`, `on:` directives, or `tailwind.config.js`.
   - `#lib` imports with extensions.
   - Size limits table.
   - English-only.
6. Architecture rules: feature boundaries, server-only code, no module-level mutable state on the server, generated types never hand-edited.
7. Testing: what to test where (unit, component browser, e2e with fake media), with commands.
8. Commits and PRs: Conventional Commits, scopes, atomic commits, PR template.
9. Security and privacy: anonymous rooms, unguessable room IDs (≥128-bit), short-lived LiveKit tokens minted by api only, no secrets in `$app/env/public`, CSP, no recording without consent.
10. ADR process: when to write one, and the MADR template path.
11. Gotchas: autoplay/startAudio, permissions UX, TS 7 and svelte-check, shadcn-svelte alias mismatch with Kit 3.

Nested `apps/web/AGENTS.md` covers Svelte/Kit specifics and the component patterns from §2. `apps/api/AGENTS.md` covers Rust: clippy pedantic level, error-handling crate choice (thiserror/anyhow), tracing, and codegen commands.

### Gaps
- I found **no authoritative, standards-body recommendation** for numeric file or function limits. The ESLint default (50/function) and Angular Universal (200/function) are the only sourced figures. The table above is a practitioner judgment.
- I did not verify current commitlint `config-conventional` defaults such as the `header-max-length` value (historically 100). Check the installed v21.
- Not researched: Renovate vs Dependabot feature details as of 2026; `just` version; clippy `too_many_lines` default (historically 100). I did not open the docs for these.
- I did not fetch AGENTS.md examples from specific top repos (only the agents.md site summary).
