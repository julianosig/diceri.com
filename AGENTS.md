# AGENTS.md

Rules for every contributor, human or AI. The `AGENTS.md` closest to the file you
edit wins; nested files only add detail, they never contradict this one.

- Web specifics: [`apps/web/AGENTS.md`](apps/web/AGENTS.md)
- Rust specifics: [`apps/api/AGENTS.md`](apps/api/AGENTS.md) (applies to all of `crates/` too)

## 1. Product

Diceri is a Google-Meet-inspired app for **video calls with live voice translation**.
Each account (username + password, no email) owns one room named after it
(`diceri.com/@username`); everyone who joins a call signs in first
([ADR 0013](docs/decisions/0013-accounts-to-join-and-per-network-limit.md)). Each listener
picks the language they want to hear, and the other person's speech is transcribed,
translated and re-spoken **in the original speaker's own recorded voice** (with explicit
consent, see §9).

Architecture overview: [`docs/architecture.md`](docs/architecture.md).
Decisions: [`docs/decisions/`](docs/decisions/). Research backing the stack:
[`docs/research/`](docs/research/).

## 2. Language policy

**English only** for everything in the repository: code, identifiers, comments,
logs, error messages, commit messages, branch names, docs, ADRs and tests.
User-facing UI copy is the only exception, and it lives in the i18n message catalogues
(`apps/web/src/lib/i18n/messages/`, [ADR 0012](docs/decisions/0012-typed-i18n-message-catalogues.md)),
never hard-coded in components. Pages not yet moved there are migrated when touched.

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

Generated code and `docs/research/` are exempt, and `docs/decisions/` from the folder limit
(numbered ADRs are an append-only record). Tests are exempt from
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

- Rooms are private by default: only owners who opt in appear on the public live
  board ([ADR 0008](docs/decisions/0008-named-rooms-and-live-board.md)). A room's
  address (`/@username`) is guessable, so its entry policy (host approval by
  default) is what protects it ([ADR 0011](docs/decisions/0011-password-accounts-and-personal-rooms.md)).
  Ids are UUIDv7 and never grant access (§14); never expose sequential ids.
- Passwords are hashed with Argon2id; sessions are random tokens stored hashed.
- Media tokens (LiveKit JWTs) are minted only by `apps/api`, scoped to one
  room, and short-lived (5–10 min).
- **Voice cloning requires explicit opt-in** from the speaker for that call, and joining
  requires it: translations are spoken only in the speaker's own recorded voice, never a
  stock voice (captions only if the clone is refused). Provider clones are deleted when
  the call ends; the recording itself is kept encrypted with the account until it is
  deleted (ADR 0015). See [ADR 0006](docs/decisions/0006-voice-cloning-consent.md) and
  [ADR 0014](docs/decisions/0014-own-voice-required-to-join.md).
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

## 14. Database and identifiers

Backed by [the database design research](docs/research/2026-11-database-design/report.md)
and [ADR 0010](docs/decisions/0010-uuidv7-ids-and-database-conventions.md). SQLite today
(accounts are the only durable data), PostgreSQL-ready.

**Identifiers, capabilities and secrets: three different things.**

1. Every entity id is a **UUIDv7** (RFC 9562) typed in `domain` (`RoomId`, `RequestId`,
   `AccountId`, `ParticipantId`), generated in the application through the
   `IdGenerator` port, never by the database. Primary key column: `id`.
2. **Never use an id (any UUID) as a capability or secret** (RFC 9562 §8). Anything that
   grants access is a separate 128-bit CSPRNG value: the room link is a `RoomCode`
   (22 base62 chars), host/guest/account keys are `AccessKey`s.
3. Secrets are stored only as SHA-256 (they are server-generated and high-entropy);
   a user-chosen secret would need Argon2id. Never store a key in plaintext.
4. Room codes and keys are never logged, never sent to providers or the media server,
   and their `Debug` is redacted. Logs, LiveKit room names and provider resources use ids.
5. Ids travel as lowercase hyphenated text; parse case-insensitively and answer 404 to
   anything that is not a v7. Never expose account ids publicly (the handle is the
   username).

**Schema.**

6. Every SQLite table is `STRICT`; ids are `BLOB NOT NULL CHECK (length(id) = 16)`
   (PostgreSQL: `uuid`).
7. Names: lowercase snake_case, plural tables, `<referenced_singular>_id` foreign keys,
   `is_`/`has_` booleans, units in names (`_ms`, `_bytes`), instants `<participle>_at`.
8. Name every constraint and index: `check_<table>_<col>_<kind>`, `uq_<table>_<cols>`,
   `fk_<table>_<col>_<ref>`, `index_<table>_on_<cols>`.
9. `NOT NULL` unless absence has a documented meaning. Every invariant of a domain type
   gets a `CHECK` mirroring it (the domain constructor stays the primary validator).
   `TEXT` + length `CHECK`, never `varchar(n)`. Enums: `TEXT CHECK (col IN (...))`.
10. Foreign keys state `ON DELETE` explicitly and their column is indexed.
11. Instants are UTC Unix **milliseconds** in `INTEGER` (PostgreSQL `timestamptz`), set by
    the application; DB defaults are only a backstop; no triggers.
12. Hard delete by default; "ended"/"closed" is a state column. No soft delete without
    an ADR. Normalize to 3NF; JSON only for opaque payloads that are never queried.

**Migrations.**

13. Schema changes are `sqlx::migrate!` files in `crates/infra/migrations/`, named
    `<YYYYMMDDHHMMSS>_<snake_description>.sql`, applied at startup before serving.
14. Forward-only: no down migrations; never edit or delete a merged migration; fix
    forward. One logical change per migration; DDL and data backfills are separate.
15. Changes an older running release cannot survive (rename, drop, type change, new
    NOT NULL) use expand/contract across two releases. SQLite table rebuilds follow the
    official 12-step procedure in a `-- no-transaction` migration.
16. Portable SQL only: `ON CONFLICT ... DO UPDATE` and `RETURNING` are fine;
    no `AUTOINCREMENT`, `INSERT OR REPLACE`, `GLOB`.

**Data access.**

17. Ports in `application::ports` speak domain types and a `thiserror` enum; `sqlx`
    types (`Row`, `Pool`, `Transaction`) never leave `infra`. Methods are
    aggregate-scoped and intention-revealing; add one only when a use case needs it.
18. Bind every value; SQL text is `&'static str`. `AssertSqlSafe` only in `infra`, for
    allow-listed identifiers, with a comment saying why.
19. Map driver errors with `is_unique_violation()` / `is_foreign_key_violation()` /
    `is_check_violation()`, never by parsing messages. Rows become domain types through
    their constructors (a corrupt row is a storage error, not a panic).
20. Every query names the index that serves it; no queries in loops; lists use keyset
    pagination on `(created_at, id)`, never `OFFSET`. Cross-aggregate atomicity goes
    through a unit-of-work port, never a transaction threaded through `application`.
21. Read-then-write transactions use `BEGIN IMMEDIATE`; no transaction spans a network
    or provider call.

**Operation, testing, privacy.**

22. Connection options live in one function (`sqlite_accounts::options`): WAL,
    `foreign_keys`, 5 s busy timeout, `synchronous = FULL` (change only with a written
    reason), `optimize_on_close`. Bundled SQLite ≥ 3.51.3.
23. The database lives on local disk, in a `0700` directory with `0600` files, outside
    any served path. Backups are `VACUUM INTO` snapshots checked with
    `PRAGMA integrity_check` (`scripts/backup-db.sh`); never copy a live `.db` without its WAL.
24. Each repository port has one contract suite run against both its in-memory fake and
    its SQL adapter; adapter tests run the real migrations.
25. No column may hold audio, transcripts, translations or voice embeddings, with one
    exception: each account's voice recording in `voice_samples`, encrypted with
    AES-256-GCM under `DICERI_VOICE_KEY` ([ADR 0015](docs/decisions/0015-keep-each-accounts-voice-sample.md)).
    Raw IP addresses are never persisted.
26. Every table with personal data is listed in [`docs/data-inventory.md`](docs/data-inventory.md)
    with purpose, legal basis (LGPD/GDPR), retention and deletion path.
27. Never log bound parameters, usernames, keys, hashes or the database URL; log
    operation names, ids and durations.
