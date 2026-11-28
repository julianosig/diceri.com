# Architecture

Status: as built (late November 2026). Decisions are recorded in
[`decisions/`](decisions/); the evidence is in
[`research/2026-11-voice-translation-stack/report.md`](research/2026-11-voice-translation-stack/report.md)
and [`research/2026-11-multilingual-stack/report.md`](research/2026-11-multilingual-stack/report.md).

## Overview

```
 Browser A (SvelteKit SPA)                         Browser B (SvelteKit SPA)
   mic/cam ──WebRTC──┐                         ┌──WebRTC── mic/cam
   hears B in lang A │                         │ hears A in lang B
                     ▼                         ▼
              ┌──────────────── LiveKit SFU (+TURN) ───────────────┐
              │ tracks: A.original  B.original  A→langB  B→langA   │
              └───────────────▲──────────────────┬─────────────────┘
                              │ publish           │ subscribe (PCM 48 kHz)
                     ┌────────┴──────────────────▼─────────┐
                     │ media-agent (Rust, livekit crate)    │
                     │ per speaker: VAD → STT → MT → TTS    │
                     └──┬──────────┬───────────┬────────────┘
                        │ WS       │ HTTP/WS   │ WS
                   Scribe STT   LLM (MT)   TTS (account's voice clone)

   HTTP /api/v1 ──► api (axum): accounts, rooms, voices, plans, LiveKit tokens, SPA
```

## Components

| Component | Responsibility | Status |
| --- | --- | --- |
| `apps/web` | landing, sign-in, pre-join (devices, camera, language, voice), call UI, captions, rejoin after reload | done |
| `apps/api` | accounts, room lifecycle, voices, plans and limits, join tokens, static hosting | done |
| `apps/media-agent` | joins rooms, runs one translation pipeline per speaker and target language | done (Rust, LiveKit SDK) |
| LiveKit Cloud | SFU, TURN, track routing, Krisp noise filter | done |
| `crates/media` | per-speaker pipeline: segmentation, translation fan-out, ordered speech | done |
| `crates/domain` | ids, `Room`, `Account`, `LanguageCode`, plans | done |
| `crates/application` | use cases + ports (repositories, `SpeechToText`, `Translator`, `TextToSpeech`, `VoiceCloner`, `VoiceSampleStore`) | done |
| `crates/infra` | adapters: SQLite and in-memory storage, LiveKit tokens, Scribe, Soniox, Gemini, Inworld, ElevenLabs, agent dispatcher | done |

## Translation pipeline (per speaker, per target language)

1. **Capture:** speaker publishes an Opus track with echo cancellation on.
2. **Ingest:** media-agent subscribes, receives 48 kHz PCM, resamples to 16 kHz.
3. **STT:** streaming to ElevenLabs Scribe v3 Live (VAD commits, language
   detection; Soniox selectable as fallback); the
   spoken language is **detected per phrase**, never declared (room output
   languages are hints). Finals drive translation; endpointing decides phrase
   boundaries.
4. **MT:** each final segment is translated **directly** (never via English) by
   Gemini 3.5 Flash-Lite with minimal thinking, prompted as a live interpreter
   (idioms and expressions adapted, not literal; tone and form of address kept;
   recognition errors read in context). Context is the call's last 8 turns from
   **every** participant (`Conversation`, one per call), under neutral labels
   (A, B, …) so no name or id reaches the provider. Only finals are spoken,
   because spoken audio cannot be revised.
5. **TTS:** the translation **streams** from Gemini and each complete sentence (or
   long clause, `Clauses`) is synthesized while the rest is still translating, by
   the configured provider (`DICERI_TTS_PROVIDER`: Inworld `inworld-tts-2` in production,
   ElevenLabs `eleven_v4_turbo` selectable, ADR 0016) in the speaker's own voice clone.
   Without a clone the phrase is captioned only, never spoken by a stock voice (ADR 0014).
6. **Publish:** translated audio is published as `<speaker>-<lang>` track; each
   listener subscribes only to the language they chose, with the original track
   ducked underneath.

Latency budget (end of phrase → translated audio): endpointing 200–500 ms,
STT final 100–300 ms, MT 300–700 ms, TTS first audio 150–300 ms, transport
100–200 ms. **Target ≈ 1–2 s.** Each phrase logs one `phrase latency` line per
language, from the end of the phrase: `stt_wait_ms` (last partial → final),
`first_piece_ms` and `translated_ms` (MT), `first_audio_ms` (TTS) and `speech_ms`.

Estimated provider cost: ≈ $0.04 per call-minute for both directions
(dominated by TTS). See ADR 0005.

## Multilingual routing

About 57 languages are supported any-to-any; 23 are Tier 1 (all stages strong).
Each stage has one default provider; a static, config-driven routing table
overrides it per language only where a provider has a proven gap (ADR 0005):

| Key | Stage | Example routes |
| --- | --- | --- |
| phrase's detected language | STT | yue/zu/xh → AssemblyAI (coverage) |
| listener's language | MT | Tier 2 targets → Gemini 3.6 Flash |
| listener's language | TTS | zh/ja/ko → Qwen-Audio-3.1 (only if eval confirms) |

Each participant chooses only their **output language** in the lobby. Target
routes are lookups on it; source routes use the detected language of the phrase.

## Providers behind ports

Every provider sits behind a trait in `application::ports` so it can be swapped
after the bake-off (ADR 0005) without touching use cases:
`SpeechToText`, `Translator`, `TextToSpeech`, `VoiceCloner`, `VoiceSampleStore`.

## Accounts, rooms and addresses

Everyone signs in to join (ADR 0013): each account owns one room at `/@username`.

| Address | What it does |
| --- | --- |
| `/` | landing page only (with the public beta's free spots) |
| `/open` | the owner opens their room; once the call starts the address shows `/@username` |
| `/@username` | joining someone's room; the owner is sent to `/open` |
| `/signin?next=` | sign in or create an account, then return to `/open` or `/@username` |

Reloading during a call rejoins it directly (the tab remembers it was in the call), keeping
the call timer. Sign-ups are capped by `DICERI_MAX_ACCOUNTS` (public beta, 100) and by
`DICERI_ACCOUNTS_PER_NETWORK` per network (5).

Each call holds at most its plan's people (`DICERI_<PLAN>_PARTICIPANTS`, 2 by default), one
seat per account, and the service at most `DICERI_MAX_LIVE_CALLS` calls. LiveKit webhooks
(ADR 0018) enforce this as it happens: a person beyond the call's size, a hidden listener
without its person, or anyone joining a call that ended is removed at once; the per-minute
sweep remains as a backstop.

## Voices

Joining needs the person's own voice (ADR 0014). They record about 10 seconds once:
the recording is checked for speech (loudness, then Soniox hears words), kept encrypted
with AES-256-GCM under `DICERI_VOICE_KEY` (ADR 0015), and cloned **once per account**
at the provider (ADR 0017). The clone's id is stored in `account_voices` and reused
instantly in every later call and after reloads; recording again replaces it, and
"Delete my voice" deletes the recording and the clone. Account clones are named
`diceri-account-<deployment>-<account id>`, so the media agent's startup sweep of
orphaned per-call clones (`diceri-<deployment>-`) never touches them.
