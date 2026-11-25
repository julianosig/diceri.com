# Architecture

Status: initial target architecture (November 2026). Decisions are recorded in
[`decisions/`](decisions/); the evidence is in
[`research/2026-11-voice-translation-stack/report.md`](research/2026-11-voice-translation-stack/report.md).

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
| `apps/media-agent` | joins rooms, runs one translation pipeline per speaker and target language | planned |
| LiveKit server | SFU, TURN, track routing | planned |
| `crates/domain` | `RoomId`, `LanguageCode`, `Room` | done |
| `crates/application` | use cases + ports (`RoomRepository`, later `SpeechToText`, `Translator`, `TextToSpeech`) | rooms done |
| `crates/infra` | adapters (in-memory storage now; Postgres/Redis, Soniox, LLM, ElevenLabs later) | in-memory done |

## Translation pipeline (per speaker, per target language)

1. **Capture:** speaker publishes an Opus track with echo cancellation on.
2. **Ingest:** media-agent subscribes, receives 48 kHz PCM, resamples to 16 kHz.
3. **STT:** streaming to Soniox `stt-rt-v6`; partials drive live captions, finals
   drive translation. Endpointing decides segment boundaries.
4. **MT:** each final segment is translated by a fast LLM with the last 2–4
   segments as context and a per-call glossary. Only finals are spoken, because
   spoken audio cannot be revised.
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

## Providers behind ports

Every provider sits behind a trait in `application::ports` so it can be swapped
after the bake-off (ADR 0005) without touching use cases:
`SpeechToText`, `Translator`, `TextToSpeech`, `VoiceCloneRegistry`, `MediaGateway`.
