# Production Rust backend (axum + tokio) for a real-time translated video-call app — Nov 2026

Scope: crate/toolchain versions, project layout and conventions, media (WebRTC) architecture for a 1:1 call with a server-side STT -> translation -> TTS pipeline, security for anonymous link-joined rooms, deployment/CI. Researched 2026-11-25. Versions were pulled live from the crates.io API (`/api/v1/crates/<name>`) on that date.

## 1. Current versions (toolchain, edition, crates)

### Takeaway
Use Rust 1.100 (stable since 2026-11-12) with **edition 2024**. There is no 2027 edition yet. axum is still on the **0.8.x** line (0.8.9). There is no axum 0.9 on crates.io. Several companion crates had major bumps in 2026 that older tutorials miss: tower-http 0.7, sqlx 0.9, utoipa 6, reqwest 0.13, jsonwebtoken 12, ts-rs 12. specta 2 is still a release candidate.

### Cited Findings
- Rust 1.100.0 was released 2026-11-12. It stabilizes the never type `!`, changes never-type fallback for pre-2024 editions to match 2024, stabilizes core parts of the Allocator API, and lets Cargo report unused dependencies — [Phoronix](https://phoronix.com/news/Rust-1.100-Released); [Linuxiac](https://linuxiac.com/?p=223025); [noise.getoto.net mirror of release post](https://noise.getoto.net/2026/11/12/rust-1-100-0-released/)
- Secondary coverage says Cargo added a "min-publish-age" style setting, a waiting period before newly published dependency versions are picked. This is useful supply-chain hardening, but unverified against the official post — [SmartScope](https://smartscope.blog/en/blog/rust-1-100-0-never-type-stabilized-cargo-2026/)
- Rust 2024 edition shipped with Rust 1.85 (Feb 2025). A 2027 edition is only under discussion. The internals thread says it should ship ~early 2027 *only if* there are enough breaking changes, and otherwise be skipped. No official 2027 edition announcement was found — [Rust internals](https://internals.rust-lang.org/t/we-have-only-two-years-to-prepare-the-rust-2027-edition/22494); [Edition guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
- axum 0.8.0 (2025-01-01) changed path syntax from `/:id` to `/{id}`, and the old syntax panics. `ws::Message` now uses `Bytes`. Query/Form errors use serde_path_to_error. `#[async_trait]` is no longer needed for custom extractors — [tokio.rs blog](https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0)
- tower-http 0.7.0 (mid-2026) made follow-redirect header filtering cumulative (a breaking change) and added a CSRF protection middleware modeled on Go 1.25 cross-origin protection — [watchbird Renovate PR](https://codeberg.org/AriosPhoenix/watchbird/pulls/143); [graph-node bump PR](https://ohotnik.duckdns.org/graphprotocol/graph-node/pull/6633). The full changelog was not retrieved.
- Versions on crates.io as of 2026-11-25 (max stable, last update), from the crates.io API — [crates.io](https://crates.io/):

| Crate | Version | Notes |
|---|---|---|
| axum | 0.8.9 (2026-04-14), MSRV 1.80 | still 0.8 line |
| axum-extra | 0.12.6 | |
| tokio | 1.54.1 (2026-10-21) | |
| tokio-util | 0.7.20 | CancellationToken, TaskTracker |
| tower | 0.5.3 | |
| tower-http | 0.7.1 (2026-08-31) | 0.6 -> 0.7 bump in 2026 |
| tower_governor | 0.8.0 (2025-08-14) | not updated in 15 months; check that it is compatible with tower-http 0.7 / axum 0.8.9 |
| serde / serde_json | 1.0.229 / 1.0.154 | |
| tracing / tracing-subscriber | 0.1.44 / 0.3.23 | |
| opentelemetry / opentelemetry-otlp | 0.33.1 | |
| tracing-opentelemetry | 0.34.0 | must match the opentelemetry 0.33 line |
| sqlx | 0.9.1 (2026-11-18), MSRV 1.94 | 0.9.0 released 2026-05-21; 0.8 is outdated |
| thiserror / anyhow | 2.0.21 / 1.0.105 | |
| config | 0.15.27 | |
| figment | 0.10.19 (last release 2024-05) | stale |
| utoipa / utoipa-axum | 6.0.0 (2026-09-22) / 0.3.0 | utoipa 6 is a new major |
| aide | 0.15.1 | |
| ts-rs | 12.0.1 | |
| specta | 1.0.5 stable; 2.0.0-rc.28 (2026-11-05) | v2 is still RC |
| reqwest | 0.13.5 | |
| tokio-tungstenite | 0.30.0 | |
| jsonwebtoken | 12.0.0 (2026-11-13) | |
| uuid | 1.29.3 | |
| nanoid | 0.5.0 | |
| secrecy | 0.10.3 | |
| dashmap | 6.2.1 | |
| str0m | 0.26.0 (2026-11-22), edition 2024, MSRV 1.89 | very active, ~weekly releases |
| webrtc (webrtc-rs) | 0.21.1 (2026-10/11), edition 2024 | 1.0 release candidate line |
| livekit / livekit-api | 0.12.1 (2026-11-24) / 0.11.0 | very fast release cadence, pre-1.0 |
| opus | 0.4.0 (2026-08-23) | `audiopus` last released 2021, stale |
| rubato | 5.1.0 | resampling (48 kHz <-> 16/24 kHz) |
| webrtc-audio-processing | 2.1.0 | AEC/NS/AGC bindings |
| axum-test | 21.1.0 | |
| cargo-nextest | 0.9.152 | |
| cargo-deny | 0.22.1 | |

- webrtc-rs history: v0.17 (Jan 2026) froze the old tokio-coupled API. v0.20 rebuilt the async `webrtc` crate on the sans-I/O `rtc` crate. v0.21 (2026-09-19) is announced as a 1.0 release candidate and adds congestion control, a pacer, FlexFEC, a jitter buffer and RFC 8888 feedback interceptors — [webrtc.rs blog index](https://webrtc.rs/blog/index.html); [v0.17 feature-freeze post](https://webrtc.rs/blog/2026/01/31/webrtc-v0.17.0-feature-freeze-sansio-shift.html); [docs.rs webrtc 0.20.0](https://docs.rs/crate/webrtc/0.20.0)

### Inferences
- Pin `rust-version = "1.94"` or higher (sqlx 0.9 requires 1.94), and use a `rust-toolchain.toml` with `channel = "1.100"` so local, CI and Docker builds match.
- Tutorials written against axum 0.7, tower-http 0.6, sqlx 0.8, utoipa 5 or webrtc-rs 0.17 or earlier are outdated. In particular, webrtc-rs <=0.17 examples (tokio-coupled `RTCPeerConnection`) do not match the current API.
- Choose ts-rs 12 over specta for TS type generation unless you want specta 2 RC. utoipa 6 is the mainstream OpenAPI choice. aide 0.15 works but has a smaller ecosystem.
- Prefer `config` (or plain env + serde via `envy`-style) over `figment`, which has had no release since 2024.

### Gaps
- The official blog.rust-lang.org 1.100 post was not fetched directly. Facts come from news coverage and a mirror.
- Full tower-http 0.7 and utoipa 6 changelogs were not retrieved. tower_governor compatibility with tower-http 0.7 was not verified. tower_governor depends on axum/tower rather than tower-http, so it is likely fine, but this was not confirmed.

## 2. Project layout, conventions, lints, errors, state, shutdown, observability, testing

### Takeaway
Use a Cargo workspace with a thin `api` binary crate on top of `domain`, `app`, `infra` and `media` library crates (hexagonal: ports as traits in `domain`/`app`, adapters in `infra`). Put shared lint and dependency settings in `[workspace.lints]` and `[workspace.dependencies]`. Use a single `AppError: IntoResponse` per HTTP crate, an `Arc` state struct, `CancellationToken` + `TaskTracker` for shutdown, tracing + OTLP, and Router-level tests via `tower::ServiceExt::oneshot`.

### Cited Findings
- cargo-chef automatically picks up every crate in a workspace and produces fewer intermediate Docker layers for workspaces. This supports the multi-crate layout for build caching — [cargo-chef README](https://github.com/LukeMathWalker/cargo-chef)
- axum 0.8 custom extractors no longer need `#[async_trait]` (native async fn in traits), and `Option<T>` extraction now requires the `OptionalFromRequestParts` trait — [axum 0.8 announcement](https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0)
- The LiveKit Rust SDK requires Tokio and processes subscribed audio in spawned tokio tasks with `NativeAudioStream` — [livekit/rust-sdks README](https://github.com/livekit/rust-sdks)
- str0m is fully sync and sans-I/O. With async code, it recommends `select!` over the socket, the timeout, and a channel for app-originated data — [str0m README](https://github.com/algesten/str0m)

### Recommended workspace tree (synthesis; not from a single source)
```
diceri/
├─ Cargo.toml                 # [workspace], [workspace.package], [workspace.dependencies], [workspace.lints]
├─ rust-toolchain.toml        # channel = "1.100", components = ["rustfmt","clippy"]
├─ rustfmt.toml  clippy.toml  deny.toml  .config/nextest.toml
├─ crates/
│  ├─ domain/      # pure types: RoomId, ParticipantId, Language, Room, errors; NO tokio/axum/sqlx
│  ├─ app/         # use cases + ports (traits): RoomRepo, SttProvider, Translator, TtsProvider, MediaGateway, Clock
│  ├─ infra/       # adapters: postgres (sqlx), redis, stt/{deepgram,..}, mt/{..}, tts/{..}, livekit token issuing
│  ├─ media/       # audio pipeline: per-speaker task graph, opus decode/encode, resample, VAD, jitter/backpressure
│  ├─ api/         # axum Router, handlers by feature (rooms/, signaling/, health/), AppError, extractors, OpenAPI
│  └─ server/      # bin: config load, tracing init, wiring (composition root), graceful shutdown
├─ xtask/          # optional: codegen (ts-rs export), openapi dump
└─ tests/ or crates/api/tests/   # integration tests (Router + oneshot; testcontainers for PG)
```
Within `api`, use feature folders (`rooms/{mod.rs, handlers.rs, dto.rs, routes.rs}`), not layer folders.

### Workspace Cargo.toml lint snippet (recommended)
```toml
[workspace.package]
edition = "2024"
rust-version = "1.94"

[workspace.lints.rust]
unsafe_code = "forbid"            # override to "deny"+allow in media crate if FFI needed
missing_debug_implementations = "warn"
unreachable_pub = "warn"
rust_2024_compatibility = { level = "warn", priority = -1 }

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
cargo    = { level = "warn", priority = -1 }
# restriction picks
unwrap_used = "deny"
expect_used = "warn"
panic = "deny"
todo = "deny"
dbg_macro = "deny"
print_stdout = "deny"
print_stderr = "deny"
indexing_slicing = "warn"
as_conversions = "warn"            # audio code: prefer From/TryFrom
await_holding_lock = "deny"
large_futures = "warn"
too_many_lines = "warn"
cognitive_complexity = "warn"
# pedantic noise
module_name_repetitions = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
multiple_crate_versions = "allow"  # from clippy::cargo; deny.toml handles dupes
```
Each member crate sets `[lints] workspace = true`.

`clippy.toml`:
```toml
too-many-lines-threshold = 80
cognitive-complexity-threshold = 20
too-many-arguments-threshold = 6
type-complexity-threshold = 250
allow-unwrap-in-tests = true
allow-expect-in-tests = true
```
`rustfmt.toml` (stable options only): `edition = "2024"`, `max_width = 100`, `use_field_init_shorthand = true`, `use_try_shorthand = true`. Nightly-only options (`imports_granularity = "Crate"`, `group_imports = "StdExternalCrate"`) need `cargo +nightly fmt` and should be optional.

Size guideline (convention, not sourced): <=400 lines per file, a hard review flag at 600; <=60–80 lines per function (enforced via `too_many_lines`); one feature per module.

### Patterns (convention guidance)
- **Errors:** use `thiserror` enums in `domain`/`app`/`infra`, with `anyhow` only in `server` main and tests. In `api`, use `enum AppError { NotFound, Validation(..), RateLimited, Unauthorized, Internal(#[from] anyhow::Error) }` with `impl IntoResponse`. It maps to a status code and an RFC 9457 problem+json body, logs internals at `error!`, and never leaks them to the client.
- **State:** `#[derive(Clone)] struct AppState { inner: Arc<Inner> }`, with ports stored as `Arc<dyn Trait>` (or generics in hot paths) and handlers using `State<AppState>`. Use `FromRef` for sub-states.
- **Shutdown:** `axum::serve(listener, app).with_graceful_shutdown(signal)`, where `signal` waits on SIGTERM/ctrl-c and then cancels a root `tokio_util::sync::CancellationToken`. Spawn media and room tasks via `TaskTracker`, pass child tokens to them, then `tracker.close(); tracker.wait()` with a timeout.
- **WebSocket signaling:** `WebSocketUpgrade` -> `socket.split()`, a writer task fed by a bounded `mpsc`, and a reader loop inside `tokio::select!` with the cancel token and a ping/idle timeout. Cap message size, and use typed JSON messages (serde-tagged enum shared with the frontend via ts-rs).
- **Backpressure:** use bounded `mpsc` everywhere. For real-time audio, prefer *drop-oldest* (or `try_send` with frame drop and a metric) over awaiting, because a late audio frame is worthless. Use `broadcast` only for fan-out of control events, and `watch` for config/language changes.
- **Observability:** `tracing-subscriber` with JSON fmt + `EnvFilter`, and `tracing-opentelemetry` 0.34 + `opentelemetry-otlp` 0.33 (kept on the matching line). Add `tower-http` `TraceLayer` + `RequestIdLayer`/`PropagateRequestIdLayer`, and spans per room/participant/pipeline stage. Emit latency histograms per stage (STT first-partial, MT, TTS first-byte, end-to-end).
- **Testing:** unit tests in-crate. Run Router tests with `app.oneshot(Request)` (`tower::ServiceExt`) or `axum-test` 21, and DB tests with `#[sqlx::test]` or testcontainers. Use fake port implementations (in-memory STT/TTS) for pipeline tests with `tokio::time::pause()`. Run everything with `cargo nextest run`, and run doctests separately (`cargo test --doc`) because nextest does not run doctests.

### Inferences
- The domain/app split keeps provider swaps (Deepgram -> another STT) to the adapter layer, which matters because the STT/MT/TTS market changes quickly.
- Keep `media` free of axum. It should depend only on `app` ports so it can run in a separate worker process later.

### Gaps
- I did not fetch exemplary repos (zero2prod, realworld-axum, Lemmy) during this session. The layout above is synthesized convention, not copied from them.
- Exact clippy lint names should be verified with `cargo clippy -- -W help` on 1.100. New lints (or renamed ones) may exist.

## 3. Media architecture: WebRTC option comparison and recommendation

### Takeaway
**Recommend LiveKit (self-hosted OSS server, or LiveKit Cloud) as the SFU, plus a Rust "translator agent" worker using the `livekit` Rust SDK (0.12.x)** that subscribes to each speaker's audio track, runs STT -> MT -> TTS, and publishes a translated audio track per listener language. The axum backend handles rooms, links, tokens (via `livekit-api`) and business logic, and does not terminate media itself. str0m is the best pure-Rust option if you must own the media plane, but you would have to build TURN, jitter buffering, simulcast policy and client SDK glue yourself.

### Cited Findings
- **LiveKit Rust SDK:** crates `livekit`, `livekit-api` (server APIs + `AccessToken` JWT with `VideoGrants{room_join, room}`), `livekit-ffi` and `livekit-protocol`. It wraps Google libwebrtc (C++) and requires Tokio. It supports Linux/macOS/Windows/iOS/Android. It can subscribe to remote audio via `NativeAudioStream`, and its examples include `basic_room`, `play_from_disk` and `save_to_disk` — [livekit/rust-sdks](https://github.com/livekit/rust-sdks); [docs.rs libwebrtc](https://docs.rs/crate/libwebrtc/0.3.26/source/Cargo.toml)
- **LiveKit server** has an embedded TURN server with integrated auth, and clients fall back to TCP/TURN automatically. Production config goes in a YAML file via `--config`/`LIVEKIT_CONFIG` — [LiveKit self-hosting deployment](https://docs.livekit.io/transport/self-hosting/deployment.md); [config-sample.yaml](https://www.github.com/livekit/livekit/blob/master/config-sample.yaml)
- **LiveKit Agents translation (Python):** `livekit.agents.beta.translation` provides `TranslationModel` / `TranslationSession` / `TranslationRoomIO`. It links a source audio track, publishes the translated audio on a new track named `{source_track_sid}-{language}`, and publishes input/output transcripts with `lk.transcription_language`. Providers are `GeminiTranslationModel` and OpenAI `RealtimeTranslationModel`. It is track-based, with `set_track`/`unset_track` at runtime — [LiveKit API ref](https://docs.livekit.io/reference/python/livekit/agents/beta/translation/index.html)
- The LiveKit cascaded recipe "Pipeline Translator" uses Deepgram STT -> OpenAI LLM ("respond only with the translation") -> ElevenLabs `eleven_multilingual_v2` TTS with `preemptive_generation=True` — [LiveKit recipe](https://docs.livekit.io/reference/recipes/pipeline_translator.md)
- OpenAI positions `gpt-realtime-translate` for human-to-human multilingual use (not agents) — [OpenAI cookbook](https://developers.openai.com/cookbook/examples/voice_solutions/realtime_translation_guide)
- **Pipecat** chains transport -> STT -> LLM -> TTS as swappable frame processors. It offers `SmallWebRTCTransport` (P2P to the bot, no media server), Daily (managed), or WebSocket transports — [AWS blog on Pipecat](https://aws.amazon.com/blogs/machine-learning/deploy-voice-agents-with-pipecat-and-amazon-bedrock-agentcore-runtime-part-1); [dev.to](https://dev.to/mohamedamine_benhima/what-makes-pipecat-different-from-other-voice-agent-frameworks-2n0)
- **Latency budget:** target ~500 ms perceived latency for conversational speech-to-speech translation, with 2–3 s acceptable for lectures. In one cascade, TTS made up 62% of the real-time factor, and streaming TTS cut added latency from 4,200 ms to 475 ms (at ~2 pp WER cost). VAD endpointing is a major delay source, so send audio to STT in parallel with VAD — [Deepgram](https://deepgram.com/learn/real-time-speech-to-speech-translation)
- **str0m:** sans-I/O, sync, no threads and no internal clock. It supports ICE, SDP, data channels, TWCC, BWE, simulcast, NACK, and frame or RTP mode. It does **not** include a PeerConnection API, adaptive jitter buffer, encode/decode, TURN or NIC enumeration. It is tested mostly as a server SFU, used in production by Lookback as an SFU, and its chat SFU example is "not for production" — [str0m README](https://github.com/algesten/str0m). Latest is 0.26.0 (2026-11-22) — crates.io API.
- **webrtc-rs:** a Pion port. It was rebuilt on the sans-I/O `rtc` crate in 0.20, and 0.21 is a 1.0 RC with jitter buffer, pacer, congestion control and FlexFEC interceptors — [webrtc.rs blog](https://webrtc.rs/blog/index.html)

### Option comparison (synthesis)
| Option | Fit for 1:1 + server audio tap | Pros | Cons |
|---|---|---|---|
| **LiveKit SFU + Rust agent (recommended)** | Excellent | Battle-tested SFU, embedded TURN, JS/mobile client SDKs, libwebrtc-grade Opus/jitter/NetEQ in the agent via the Rust SDK, translated-track model matches LiveKit's own translation design, Cloud or self-host | Extra Go service to run; Rust SDK is pre-1.0 with fast churn; libwebrtc build adds C++ toolchain/binary size |
| str0m embedded in axum process | Good but DIY | Pure Rust, sans-I/O (deterministic tests), single binary | You build signaling, TURN (coturn), jitter buffer, Opus decode/encode, BWE policy, reconnection, client glue |
| webrtc-rs 0.21 | Medium | Pure Rust, Pion-like API, now has jitter buffer/FEC | Just reached 1.0-RC after a rewrite; less production evidence |
| mediasoup (Node/C++ worker, Rust crate exists) | Good | Mature SFU | Rust binding is secondary; still need custom audio tap via PlainTransport/RTP |
| Browser P2P + server tap | Poor | Lowest cost for raw call | Server cannot see P2P media without a separate upstream connection; doubles uplink; hard to keep translated track in sync |

### Audio handling recommendations
- **Echo cancellation:** keep browser `getUserMedia({audio:{echoCancellation:true,noiseSuppression:true,autoGainControl:true}})`. Translated audio arrives as a normal remote WebRTC track, so the browser AEC reference includes it. Server-side AEC is not needed for the STT tap. Optionally apply noise suppression server-side before STT (`webrtc-audio-processing` 2.1).
- **Opus:** decode 48 kHz Opus to PCM (the LiveKit SDK does this for you), then resample with `rubato` to the provider rate (often 16 kHz for STT, 24 kHz for TTS input/output). Publish TTS PCM back via the SDK's audio source, which encodes Opus. Enable Opus DTX/FEC on the client.
- **Pipeline per speaker:** one tokio task graph per (speaker track, target language). Use a bounded channel with drop-oldest between stages, streaming STT (partials) -> sentence/clause segmentation -> streaming MT -> streaming TTS, and a cancellation token for barge-in and for language changes. Duck or mute the original voice (or play it at low volume) on the listener side.

### Inferences
- For a 1:1 call, the critical latency path is provider latency (STT endpointing + TTS first byte), not SFU forwarding, so WebRTC-stack performance differences are secondary to the operational maturity of TURN, reconnection and client SDKs. This favors LiveKit.
- LiveKit's own translation module and recipes are Python. Writing the agent in Rust means reimplementing that orchestration (segmentation, turn events) with the Rust SDK. This is feasible but not turnkey. An alternative is a Python LiveKit Agents worker for media while Rust stays the control plane. Note this trade-off for the team.

### Gaps
- No published latency benchmarks comparing LiveKit vs str0m vs webrtc-rs for 1:1 calls were found.
- `NativeAudioSource`/publishing and APM features of the Rust SDK were not confirmed from the README fetch. Check docs.rs for `livekit::webrtc::audio_source`.
- The claim about browser AEC covering translated audio played via WebRTC is an inference. Chrome AEC does not reliably cancel audio played via Web Audio/`<audio>` from non-WebRTC sources, so play translated audio as a WebRTC track.

## 4. Security for anonymous link-joined rooms

### Takeaway
Use 128-bit+ random room IDs (not sequential and not UUIDv7), short-lived scoped JWTs per join (LiveKit `AccessToken` with a `room_join` grant for that room only, TTL of minutes), rate limiting at the edge and in tower, TURN with time-limited credentials, strict CORS, and secrets via env/secret manager wrapped in `secrecy`.

### Cited Findings
- LiveKit tokens are built with `AccessToken::with_api_key(key, secret).with_identity(..).with_grants(VideoGrants{room_join:true, room:..}).to_jwt()`, with the key/secret read from env — [livekit/rust-sdks](https://github.com/livekit/rust-sdks)
- LiveKit's embedded TURN authenticates through LiveKit itself, so no separate long-lived TURN secrets reach clients — [LiveKit deployment docs](https://docs.livekit.io/transport/self-hosting/deployment.md)
- tower-http 0.7 adds CSRF (cross-origin) protection middleware — [graph-node PR](https://ohotnik.duckdns.org/graphprotocol/graph-node/pull/6633)
- tower_governor latest is 0.8.0 (Aug 2025) — crates.io API.

### Inferences (recommendations)
- Room ID: 16–22 chars from `nanoid` (default alphabet with 21 chars is ~126 bits) or base62 of 128 random bits. Never expose a DB PK.
- Join flow: `POST /rooms` -> creator gets the link plus a host secret. `POST /rooms/{id}/join` (rate-limited per IP + per room) returns a LiveKit JWT with a TTL of 5–10 min, a random identity, `can_publish` for audio/video only, and a room max of 2 participants enforced server-side (LiveKit `max_participants` on room create).
- Rate limiting: tower_governor (per IP, behind `SmartIpKeyExtractor` only if the proxy is trusted) plus Cloudflare/nginx limits. Cap WebSocket connections per IP.
- CORS: `tower_http::cors::CorsLayer` with an explicit origin list (never `Any` with credentials).
- If you are not using LiveKit TURN, use coturn with `use-auth-secret` (TURN REST API ephemeral credentials) or a managed TURN service. Expose TURN on 443/TCP+TLS for restrictive networks.
- Secrets: env injected from a secret manager. Wrap them in `secrecy::SecretString` so they are never logged. Use `cargo-deny` + `cargo-audit` in CI.
- Rooms expire (TTL in Redis/PG) and are destroyed when the last participant leaves.

### Gaps
- tower_governor compatibility with axum 0.8.9 / tower-http 0.7 was not verified.

## 5. Deployment and CI

### Takeaway
Use a multi-stage Docker build with cargo-chef (planner/builder/runtime) and the same Rust version in all stages. Use a `debian:trixie-slim` runtime, or `gcr.io/distroless/cc-debian12/13` for glibc binaries. Note that the LiveKit SDK's libwebrtc requires glibc, so do not use musl or scratch for the agent. CI on GitHub Actions should run fmt --check, clippy `-D warnings`, nextest, doctests, cargo-deny and cargo-audit.

### Cited Findings
- The official cargo-chef Dockerfile: `FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef` -> `planner` (`cargo chef prepare --recipe-path recipe.json`) -> `builder` (`cargo chef cook --release --recipe-path recipe.json`, then `cargo build --release --bin app`) -> `FROM debian:trixie-slim AS runtime`. Use the same Rust version across stages or caching breaks. For Alpine, build static musl — [cargo-chef README](https://github.com/LukeMathWalker/cargo-chef)
- cargo-nextest 0.9.152 and cargo-deny 0.22.1 are the current versions — crates.io API.

### Recommended CI (synthesis)
```yaml
jobs:
  check:
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master   # reads rust-toolchain.toml
        with: { toolchain: "1.100", components: "rustfmt, clippy" }
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all --check
      - run: cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
      - uses: taiki-e/install-action@v2
        with: { tool: "cargo-nextest,cargo-deny,cargo-audit" }
      - run: cargo nextest run --workspace --locked --profile ci
      - run: cargo test --doc --workspace
      - run: cargo deny check          # advisories, bans, licenses, sources
```
Also pin the toolchain, use `--locked`, run an `sqlx prepare --check` step (offline query metadata), add Dependabot/Renovate, and optionally use Cargo's new min-publish-age setting.

`deny.toml` essentials: `[advisories] yanked = "deny"`; `[licenses] allow = ["MIT","Apache-2.0","BSD-3-Clause","ISC","Unicode-3.0","Zlib"]` (libwebrtc/BoringSSL may need `OpenSSL`/`BSD-*` exceptions); `[bans] multiple-versions = "warn"`, `wildcards = "deny"`; `[sources] unknown-registry = "deny"`, `unknown-git = "deny"`.

### Inferences
- Ship two images: `api` (axum control plane, can be distroless) and `media-agent` (LiveKit Rust SDK, glibc and larger). Plus LiveKit server (with embedded TURN on UDP 443/3478 and TCP 443) and optionally Redis for multi-node LiveKit.
- The action versions (`checkout@v4`, `rust-cache@v2`) are as last known. Check for newer majors.

### Gaps
- Distroless image tags for Debian 13 were not verified this session.
- GitHub Actions versions as of Nov 2026 were not verified.
