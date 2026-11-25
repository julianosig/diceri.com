# Architecture

Status: initial target architecture (November 2026). Decisions are recorded in
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
                   Soniox STT   LLM (MT)   ElevenLabs TTS (cloned voice)

   HTTP /api/v1 ──► api (axum): rooms, consent, LiveKit tokens, serves SPA build
```

## Components

| Component | Responsibility | Status |
| --- | --- | --- |
| `apps/web` | landing, lobby (devices, language, consent), call UI, captions | rooms done |
| `apps/api` | room lifecycle, join tokens, config for clients, static hosting | rooms done |
| `apps/media-agent` | joins rooms, runs one translation pipeline per speaker and target language | done (Rust, LiveKit SDK) |
| LiveKit Cloud | SFU, TURN, track routing, Krisp noise filter | done |
| `crates/media` | per-speaker pipeline: segmentation, translation fan-out, ordered speech | done |
| `crates/domain` | `RoomId`, `LanguageCode`, `Room` | done |
| `crates/application` | use cases + ports (`RoomRepository`, later `SpeechToText`, `Translator`, `TextToSpeech`) | rooms done |
| `crates/infra` | adapters: in-memory storage, LiveKit tokens, Soniox, Gemini, ElevenLabs, agent dispatcher | done |

## Translation pipeline (per speaker, per target language)

1. **Capture:** speaker publishes an Opus track with echo cancellation on.
2. **Ingest:** media-agent subscribes, receives 48 kHz PCM, resamples to 16 kHz.
3. **STT:** streaming to Soniox `stt-rt-v6` with language identification; the
   spoken language is **detected per phrase**, never declared (room output
   languages are hints). Finals drive translation; endpointing decides phrase
   boundaries.
4. **MT:** each final segment is translated **directly** (never via English) by
   Gemini 3.5 Flash-Lite with minimal thinking, the last 2–4 segments as context
   and a per-call glossary. Only finals are spoken, because spoken audio cannot
   be revised.
5. **TTS:** ElevenLabs `eleven_v4_turbo` with an instant voice clone created
   from the consented lobby sample (stock voice without consent).
6. **Publish:** translated audio is published as `<speaker>-<lang>` track; each
   listener subscribes only to the language they chose, with the original track
   ducked underneath.

Latency budget (end of phrase → translated audio): endpointing 200–500 ms,
STT final 100–300 ms, MT 300–700 ms, TTS first audio 150–300 ms, transport
100–200 ms. **Target ≈ 1–2 s.** Every stage is measured with tracing spans.

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
`SpeechToText`, `Translator`, `TextToSpeech`, `VoiceCloneRegistry`, `MediaGateway`.
