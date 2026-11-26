# AGENTS.md

Rules for every contributor, human or AI. The `AGENTS.md` closest to the file you
edit wins; nested files only add detail, they never contradict this one.

- Web specifics: [`apps/web/AGENTS.md`](apps/web/AGENTS.md)
- Rust specifics: [`apps/api/AGENTS.md`](apps/api/AGENTS.md) (applies to all of `crates/` too)

## 1. Product

Diceri is a Google-Meet-inspired app for **link-joined meetings without accounts, with
live voice translation**. One person creates a room and shares the link; the
other joins. Each listener picks the language they want to hear, and the other
person's speech is transcribed, translated and re-spoken **in a voice that
sounds like the original speaker** (with explicit consent, see §9).

Architecture overview: [`docs/architecture.md`](docs/architecture.md).
Decisions: [`docs/decisions/`](docs/decisions/). Research backing the stack:
[`docs/research/`](docs/research/).

## 2. Language policy

**English only** for everything in the repository: code, identifiers, comments,
logs, error messages, commit messages, branch names, docs, ADRs and tests.
User-facing UI copy is the only exception, and it lives in i18n message files,
never hard-coded in components once i18n lands.

## 3. Repository map

```
apps/
  web/            SvelteKit 3 SPA (Svelte 5 runes, Tailwind 4)
  api/            axum HTTP control plane (binary + lib for tests)
crates/
  domain/         pure types and rules; no I/O, no async runtime, no frameworks
  application/    use cases + ports (traits); depends only on domain
  infra/          adapters implementing ports (storage, STT, MT, TTS, LiveKit)
packages/         shared TypeScript packages (e.g. generated API types)
docs/
  architecture.md
  decisions/      ADRs (MADR 4 format)
  research/       dated research reports; read-only history
deploy/           production units and settings templates (docs/deployment.md)
scripts/          repository tooling (bash, small, documented)
```

Planned (create only when needed, with an ADR if the shape changes):
`apps/media-agent/` (LiveKit translator agent), `crates/media/` (audio pipeline).

Dependency direction is strict: `domain ← application ← infra ← apps`.
Never import "upwards" (e.g. `domain` must not know about `axum`, `tokio` or `sqlx`).

## 4. Toolchain (pinned)

| Tool | Version | Pinned in |
| --- | --- | --- |
| Rust | 1.100, edition 2024 | `rust-toolchain.toml`, `Cargo.toml` |
| Node | 26 LTS | `.node-version`, `package.json#engines` |
| pnpm | 12 | `package.json#packageManager` |
| SvelteKit / Svelte | 3.x / 5.x (runes) | `apps/web/package.json` |
| Tailwind CSS | 4.x (CSS-first) | `apps/web/package.json` |
| TypeScript | **~6** (not 7: Svelte tooling requires 6) | `apps/web/package.json` |

Do not upgrade a major version in passing. Upgrades get their own commit
(`build(deps): ...`) and, if behavior changes, an ADR.

## 5. Commands

`just` is the single entry point. Run `just` to list recipes.

| Command | What it does |
| --- | --- |
| `just setup` | install JS/Rust dependencies and git hooks |
| `just dev` | run API (`:8090`) and web (`:5180`, proxies `/api`) |
| `just fmt` | format Rust and web code |
| `just check` | static layer: fmt, clippy, cargo-deny, svelte-check, eslint, limits |
| `just test` | unit layer: Rust unit + router tests, web logic tests (no network) |
| `just verify` | `check` + `test`; **the gate before every push** (~25 s) |
| `just test-browser` | Svelte component tests in Chromium |
| `just e2e` | end-to-end smoke: real API + web build + LiveKit, fake microphones |
| `just ci` | everything CI runs |
| `just build` | production builds |
| `just deploy` | build and restart the production services on this host |

**Before declaring any task done, run `just verify` and make it pass** (and `just e2e`
when you touched a user journey or media code). Never
disable a lint, skip a test or bypass a hook (`--no-verify`) to get green.
If a rule is wrong, change the rule in its own commit and explain why.

## 6. Size limits (enforced)

Small units are the core organizing rule. When you approach a limit, split by
responsibility; do not compress code to fit.

| Unit | Limit | Enforced by |
| --- | --- | --- |
| Rust file | 400 lines | `scripts/check-limits.sh` |
| Rust function | 80 lines, cognitive complexity 15, ≤ 6 args | clippy (`clippy.toml`) |
| TypeScript file | 400 lines (code, excluding blanks/comments) | eslint `max-lines` |
| Svelte component | 300 lines | eslint `max-lines` |
| TS function | 80 lines, complexity 12, depth 4, ≤ 4 params | eslint |
| Entries per folder | 15 | `scripts/check-limits.sh` |
| Line width | 100 chars | rustfmt, prettier |

Generated code and `docs/research/` are exempt. Tests are exempt from
function-length limits but not from file limits.

## 7. Code organization principles

1. **Feature first, then layer.** Group by feature (`rooms/`, `translation/`),
   not by technical kind (`handlers/`, `models/`) at the top level.
2. **One responsibility per file.** One use case, one adapter, one component.
3. **Reusable building blocks.** UI primitives in `apps/web/src/lib/components/ui`;
   shared Rust logic in a crate, never copy-pasted between apps.
   Before writing something new, search for an existing primitive.
4. **Public surface is explicit.** Rust: `mod.rs`/`lib.rs` re-export what is
   public, everything else stays private. Web: each feature exposes an
   `index.ts`; other code imports only from it.
5. **Ports and adapters for providers.** STT, MT, TTS, storage and media are
   traits in `application::ports`. Providers change fast (see research), so
   swapping one must touch only `infra` and the composition root.
6. **No dead code, no speculative abstraction.** Add a crate, folder or
   generic only when there is a second real use.
7. **Errors are typed.** Rust: `thiserror` in libraries, `anyhow` only in
   binaries/tests. HTTP errors are RFC 9457 problem details.
8. **No secrets in code.** Configuration comes from environment variables
   (`DICERI_*`), documented in `apps/api/AGENTS.md`.

## 8. Naming

| Thing | Convention | Example |
| --- | --- | --- |
| Rust crates | `diceri-<name>` (kebab) | `diceri-application` |
| Rust files/modules | `snake_case.rs` | `room_repository.rs` |
| Svelte components | `PascalCase.svelte` | `ShareLink.svelte` |
| Runes modules | `kebab-case.svelte.ts` | `room-state.svelte.ts` |
| Other TS files | `kebab-case.ts` | `http-client.ts` |
| Tests (TS) | `*.spec.ts`, `*.svelte.spec.ts`, `*.e2e.ts` | `room-api.spec.ts` |
| Env vars | `DICERI_SCREAMING_SNAKE` | `DICERI_BIND_ADDRESS` |
| Branches | `<type>/<short-kebab>` | `feat/lobby-language-picker` |
| ADRs | `NNNN-kebab-title.md` | `0004-livekit-media-plane.md` |

## 9. Security, privacy and consent

- Rooms are private by default: only hosts who opt in show their room's title and their
  name on the public live board ([ADR 0008](docs/decisions/0008-named-rooms-and-live-board.md)).
  Room ids are 128-bit random (22 base62 chars); never
  expose sequential or database ids.
- Media tokens (LiveKit JWTs) are minted only by `apps/api`, scoped to one
  room, and short-lived (5–10 min).
- **Voice cloning requires explicit opt-in** from the speaker for that call.
  Without consent, use a stock voice. Clones are deleted when the call ends.
  See [ADR 0006](docs/decisions/0006-voice-cloning-consent.md).
- Never log audio, transcripts or provider API keys. Log ids and timings only.
- CORS uses an explicit origin allow-list; never `Any`.

## 9a. License

The project is `AGPL-3.0-only` ([`LICENSE`](LICENSE)). Every dependency must be
AGPL-compatible: permissive licenses only (MIT, Apache-2.0, BSD, ISC, Zlib, …),
enforced by `cargo deny` for Rust and reviewed with `pnpm licenses list --prod`
for the web app. Never vendor code under an incompatible or proprietary license.
The only exception is LiveKit's Krisp noise filter, allowed by the additional
permission in [`LICENSE-EXCEPTION.md`](LICENSE-EXCEPTION.md); adding any other
non-free component needs a new, explicit exception decided by the maintainers.

## 10. Testing

- Every use case and adapter has unit tests; every HTTP route has a router-level
  test (`tower::ServiceExt::oneshot`, no network).
- Web: pure logic in `*.spec.ts` (node), components in `*.svelte.spec.ts`
  (Vitest browser mode), user journeys in `apps/web/tests/e2e/*.e2e.ts`
  (Playwright, fake media devices).
- Layers, commands and rules: [`docs/testing.md`](docs/testing.md).
- Fix the code, not the test, unless the test is wrong; say which in the commit.

## 11. Git: micro-commits

We use [Conventional Commits](https://www.conventionalcommits.org), checked by
commitlint in the `commit-msg` hook.

```
<type>(<scope>): <imperative lowercase summary, ≤ 72 chars, no period>

<optional body: why, not what; wrapped at 100>
```

- **Types:** `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `build`, `ci`,
  `chore`, `style`, `revert`.
- **Scopes:** `web`, `api`, `domain`, `application`, `infra`, `media`,
  `translation`, `rust`, `ci`, `deps`, `docs`, `repo` (list in
  `commitlint.config.js`).
- **One logical change per commit.** Each commit compiles and passes
  `just check` on its own. Typical size: a few files, well under ~200 changed
  lines (excluding lockfiles and generated code).
- **Never mix** refactors with behavior changes, formatting with logic, or
  dependency bumps with features.
- Generated code is committed together with the change that produced it.
- Breaking API/protocol changes between web and api: `feat(api)!:` plus a
  `BREAKING CHANGE:` footer.
- Do not commit or push unless asked. Never rewrite published history.

## 12. Architecture Decision Records

Write an ADR (copy [`docs/decisions/template.md`](docs/decisions/template.md))
when you add or replace a provider, a framework, a crate or app, a protocol, or
anything that changes §3, §4 or §6. Status flow: `proposed → accepted →
superseded by NNNN`. Never edit an accepted ADR's decision; supersede it.

## 13. Known pitfalls (outdated patterns to avoid)

Training data and tutorials are often older than this stack. Do **not** use:

- `svelte.config.js`, `$lib`, `$app/stores`, `$env/*` (SvelteKit 2) → config in
  `vite.config.ts`, `#lib/...` imports with file extensions, `$app/state`.
- `export let`, `on:click`, `<slot>`, `createEventDispatcher`, writable stores
  for component state (Svelte 4) → `$props`, `onclick`, snippets, callback
  props, runes classes.
- `tailwind.config.js`, PostCSS/autoprefixer setup (Tailwind 3) → `@theme` in CSS.
- TypeScript 7 as the main `typescript` dependency.
- axum 0.7 idioms (`/:param` paths, `#[async_trait]` on extractors) → `/{param}`.
- `unwrap()`/`expect()` outside tests; `println!` instead of `tracing`.
- Holding a `std::sync` lock across `.await`; unbounded channels for audio.
