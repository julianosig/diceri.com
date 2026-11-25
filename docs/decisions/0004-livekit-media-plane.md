---
status: accepted
date: 2026-11-25
---

# LiveKit as media plane with a Rust translator agent

## Context and problem statement

We need reliable 1:1 WebRTC (TURN, reconnection, client SDKs) plus server-side
access to each speaker's audio to translate it and publish translated audio
back. Research: `docs/research/2026-11-voice-translation-stack/notes/rust-backend-webrtc.md`.

## Considered options

- Self-hosted LiveKit SFU + Rust agent using the `livekit` crate
- str0m (sans-IO Rust WebRTC), building TURN, jitter buffer and codecs ourselves
- webrtc-rs
- Browser peer-to-peer with a server tap

## Decision outcome

Chosen option: "LiveKit + Rust agent". The agent joins as a hidden participant,
subscribes to each original track, and publishes one translated track per
target language; listeners subscribe only to their chosen language. The API
mints short-lived, room-scoped LiveKit tokens.

### Consequences

- Good, because TURN, Opus, jitter handling and reconnection are solved.
- Good, because it matches LiveKit's own translated-track model.
- Bad, because LiveKit's ready-made translation module is Python-only; the Rust
  agent must implement segmentation and turn handling itself.
- Bad, because the agent needs glibc (libwebrtc), so no musl/scratch images.

### Confirmation

Two-browser e2e test with fake media hears a translated track; per-stage latency is traced.
