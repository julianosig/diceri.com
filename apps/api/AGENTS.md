# AGENTS.md — Rust (apps/api and crates/)

Read the root [`AGENTS.md`](../../AGENTS.md) first. These rules apply to every
Rust crate in the workspace.

## Crates and boundaries

| Crate | Contains | May depend on |
| --- | --- | --- |
| `diceri-domain` | value objects (`RoomId`, `RoomCode`, `LanguageCode`), entities, invariants | `serde`, `thiserror`, `uuid` (parse and build ids only; ADR 0010) |
| `diceri-application` | use cases (`use_cases/`), ports as traits (`ports/`) | `domain` |
| `diceri-infra` | adapters implementing ports, one per module | `application`, `domain`, provider SDKs |
| `diceri-api` | axum router, handlers by feature, composition root | everything above |

`domain` has no async, no I/O, no clock and no randomness; inject them through
ports. Adding a crate requires an ADR.

## apps/api layout

```
src/
  main.rs           thin: config, telemetry, bind, serve with graceful shutdown
  lib.rs            public surface used by main and integration tests
  config.rs         env-based configuration (see table below)
  state.rs          AppState + composition root (wires infra into use cases)
  router.rs         build_router(): routes + middleware (trace, timeout, CORS)
  error.rs          ApiError -> RFC 9457 problem+json
  features/
    <feature>/      mod.rs (routes), handlers.rs, dto.rs
tests/              router-level integration tests (oneshot, no network)
```

New endpoint checklist: use case in `application` (with unit test) → handler +
DTO in `features/<feature>/` → route in its `mod.rs` → router test in `tests/`.
DTOs are the HTTP contract (camelCase JSON); never serialize domain types directly
in responses.

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `DICERI_BIND_ADDRESS` | `127.0.0.1:8090` | listen address |
| `DICERI_ALLOWED_ORIGINS` | `http://localhost:5180` | comma-separated CORS allow-list |
| `DICERI_LOG_FORMAT` | `pretty` | `pretty` or `json` |
| `DICERI_WEB_DIR` | unset | built SPA (`apps/web/build`) served for non-API paths |
| `DICERI_WRITE_LIMIT_PER_MINUTE` | `60` | POSTs per client IP per minute (`0` disables) |
| `DICERI_CLIENT_IP_HEADER` | unset | client IP header behind a proxy (`cf-connecting-ip` for Cloudflare) |
| `DICERI_DEPLOYMENT` | `dev` | names voice clones `diceri-<deployment>-…`; API and agent must match |
| `DICERI_DATABASE_URL` | `sqlite://diceri.db` | SQLite file for accounts (the only durable data) |
| `DICERI_NETWORK_KEY` | unset | secret for hashing sign-up networks (IPv4 address or IPv6 /64) to limit room usernames per network (unset: not limited; needs `DICERI_CLIENT_IP_HEADER` behind a proxy) |
| `DICERI_MAX_ACCOUNTS` | unset | public beta: at most this many accounts; sign-up answers 429 `limit: beta` when full, sign-in still works (unset: open) |
| `DICERI_ACCOUNTS_PER_NETWORK` | `5` | room usernames one network may claim when `DICERI_NETWORK_KEY` is set |
| `DICERI_SECURE_COOKIES` | `false` | `true` marks the session cookie `Secure` (HTTPS); on in production |
| `DICERI_LIVEKIT_URL` | unset | LiveKit project URL (`wss://...`); unset disables joining (503) |
| `DICERI_LIVEKIT_API_KEY` | unset | LiveKit API key |
| `DICERI_LIVEKIT_API_SECRET` | unset | LiveKit API secret (secret) |
| `DICERI_FREE_CALL_MINUTES` | `60` | longest call on the free plan (everyone's default) |
| `DICERI_FREE_DAILY_MINUTES` | `60` | call minutes per account per UTC day on the free plan (same as the month: no daily limit) |
| `DICERI_FREE_MONTHLY_MINUTES` | `60` | call minutes per account per UTC month on the free plan |
| `DICERI_PRO_CALL_MINUTES` / `DICERI_PRO_DAILY_MINUTES` / `DICERI_PRO_MONTHLY_MINUTES` | `60` / `300` / `300` | same for the pro plan (assigned with `diceri-api plan <user> pro`) |
| `DICERI_PRO_PRICE_CENTS` | `2900` | pro price shown on the plans, US cents a month ([pricing](../../docs/pricing.md)) |
| `DICERI_TEAM_CALL_MINUTES` / `DICERI_TEAM_DAILY_MINUTES` / `DICERI_TEAM_MONTHLY_MINUTES` | `120` / `600` / `800` | same for the team plan |
| `DICERI_TEAM_PRICE_CENTS` | `7900` | team price shown on the plans, US cents a month |
| `DICERI_MAX_LIVE_CALLS` | `15` | calls with someone in them at once, across the service |
| `DICERI_MONTHLY_CALL_MINUTES` | `1500` | call minutes per UTC month across every account (keeps within the media free tier) |
| `DICERI_VOICE_CLONING` | `false` | `true` offers recording your own voice (needs the ElevenLabs and Soniox keys) |
| `DICERI_VOICE_KEY` | unset | 32 bytes in base64 (`openssl rand -base64 32`) encrypting each account's kept voice recording (ADR 0015); unset: recordings are not kept. Losing it loses the recordings (secret) |
| `DICERI_VOICE_REQUIRED` | `false` | `true` makes joining a call need a recorded voice (ADR 0014); only with voice cloning |
| `DICERI_TTS_PROVIDER` | `elevenlabs` | who clones voices: `elevenlabs` or `inworld` (ADR 0016); the media agent must use the same |
| `DICERI_INWORLD_API_KEY` | unset | Inworld key when `DICERI_TTS_PROVIDER=inworld` (secret) |
| `DICERI_ELEVENLABS_API_KEY` | unset | voice cloning (secret); recording a voice also needs the Soniox key |
| `DICERI_SONIOX_API_KEY` | unset | checks a voice recording holds words before it is cloned (secret) |
| `RUST_LOG` | `info,tower_http=debug` | tracing filter |

Copy `.env.example` to `.env` for local development (loaded with `dotenvy`).
Document every new variable here and in `.env.example` in the same commit. Wrap
secrets in `secrecy::SecretString` so they cannot be logged.

## Rules

- Lints: workspace-level clippy `pedantic` plus `unwrap_used`, `panic`, `todo`,
  `print_stdout` denied. Every crate sets `[lints] workspace = true`. Use
  `#[allow(..., reason = "...")]` with a reason, at the narrowest scope.
- `unsafe_code` is forbidden. Exceptions (FFI in a media crate) need an ADR.
- Errors: one `thiserror` enum per module boundary; map to `ApiError` with
  `From` impls in `error.rs`. Log internal errors once, at the edge.
- Async: never hold a `std::sync` lock across `.await`; use bounded channels;
  for real-time audio prefer dropping stale frames over awaiting capacity.
  Every spawned task is tracked and cancelled on shutdown
  (`tokio_util::sync::CancellationToken` + `TaskTracker`).
- Observability: `tracing` spans per request, room and pipeline stage; record
  latency per stage (STT final, MT, TTS first byte, end-to-end). Never log
  audio, transcripts or keys.
- Dependencies: add versions only in the root `[workspace.dependencies]`;
  crates use `dep.workspace = true`. `cargo deny check` must pass.

## Tests

- Unit tests live in the same file (`#[cfg(test)] mod tests`), using fake port
  implementations; no network.
- HTTP tests build the real router with `AppState::in_memory()` and call it
  with `tower::ServiceExt::oneshot`.
- Time-dependent pipeline tests use `tokio::time::pause()`.
