---
status: proposed
date: 2026-11-25
---

# Cascaded speech translation: Soniox STT, LLM MT, ElevenLabs v4 Turbo TTS

## Context and problem statement

Each listener must hear the other participant in their chosen language, in a
voice like the speaker's, with the lowest possible latency and sustainable
cost. Research (Nov 2026):
[`report.md`](../research/2026-11-voice-translation-stack/report.md).

## Considered options

- Cascade: streaming STT → LLM translation → voice-cloning streaming TTS
- Managed end-to-end: Gemini 3.5 Live Translate (preview), OpenAI
  gpt-realtime-translate, Palabra, Azure Live Interpreter
- Open-weight end-to-end: Meta Seamless (non-commercial license), Kyutai
  Hibiki-Zero (into English only)

## Decision outcome

Chosen option (pending bake-off): **cascade** with
- STT: **Soniox `stt-rt-v6`** (~$0.002/participant-min, translation and
  language ID included); challenger **AssemblyAI STT-3.7-Realtime**.
- MT: a Flash/mini-class LLM with rolling context and glossary; translate only
  final segments.
- TTS: **ElevenLabs `eleven_v4_turbo`** (~$0.034/output-min, 90+ languages,
  instant clone from ~10 s); fallbacks **Inworld Realtime TTS-2** and
  **Cartesia Sonic 3.6**.
- Gemini 3.5 Live Translate is A/B tested as the managed alternative.

### Consequences

- Good, because it gives true voice cloning, captions, glossary control and any-to-any pairs.
- Good, because target latency ≈ 1–2 s vs ≈ 3 s measured for managed end-to-end.
- Bad, because three vendors to integrate and monitor; mitigated by ports.
- Bad, because no independent pt-BR/ES streaming or cross-lingual cloning benchmark exists yet.

### Confirmation

Before `accepted`: a 1–2 day bake-off on real pt-BR/EN/ES call audio measuring
WER, finalization latency, speaker similarity (cloned PT voice speaking EN and
vice versa), end-to-end latency and blind listener ratings.
