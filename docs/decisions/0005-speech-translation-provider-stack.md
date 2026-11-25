---
status: proposed
date: 2026-11-25
---

# Multilingual cascade: one default provider per stage, routed only on proven cliffs

## Context and problem statement

Each listener must hear the other participant in their chosen language, in a
voice like the speaker's, across as many languages and any-to-any pairs as
possible, with low latency and sustainable cost. Research (Nov 2026):

- [`2026-11-voice-translation-stack/report.md`](../research/2026-11-voice-translation-stack/report.md) — stack and architecture
- [`2026-11-multilingual-stack/report.md`](../research/2026-11-multilingual-stack/report.md) — per-language quality, tiers, routing

## Considered options

- Cascade with a single provider per stage
- Cascade with per-language routing per stage
- Managed end-to-end: Gemini 3.5 Live Translate (preview), OpenAI
  gpt-realtime-translate (13 output languages), Palabra, Azure Live Interpreter
- Open-weight end-to-end: Meta Seamless (non-commercial), Kyutai Hibiki-Zero (into English only)

## Decision outcome

Chosen option (pending evaluation): **cascade, one default provider per stage,
plus a static routing table** keyed by the speaker's declared language (STT)
and the listener's chosen language (MT, TTS).

| Stage | Default | Fallback | Route now | Route only if eval confirms |
| --- | --- | --- | --- | --- |
| STT | Soniox `stt-rt-v6` with language hints | ElevenLabs Scribe v3 Live | AssemblyAI 3.7 for yue, zu, xh | AssemblyAI 3.7 for sv, fr, de, nl |
| MT | Gemini 3.5 Flash-Lite, thinking minimal, direct X→Y (never pivot via English) | gpt-6-luna / gpt-5.6-luna | Gemini 3.5 Flash for Tier 2 targets (Baltic, Balkan, kk, be, fa, Arabic dialects, zh-Hant) | Flash for any pair with a clear quality gain |
| Captions | Soniox built-in two-way translation | LLM MT text | — | — |
| TTS | ElevenLabs `eleven_v4_turbo` (instant clone) | Inworld TTS-2 | — | Qwen-Audio-3.1-TTS for zh, ja, ko targets |

Supported languages: ~58 any-to-any (Soniox ∩ ElevenLabs v4). Tier 1 (all
stages strong): en, es, pt, fr, de, it, nl, ru, tr, fi, ar, hi, zh, ja, ko, vi,
pl, cs, uk, ro, el, id, th. The lobby asks each participant to declare their
language; it is passed to STT as a hint and keys every route.

### Consequences

- Good, because true voice cloning, captions and glossary control across ~58 languages.
- Good, because target latency ≈ 1–2 s vs ≈ 3 s measured for managed end-to-end.
- Good, because cost ≈ $0.04 per two-party call-minute; routing barely changes it.
- Bad, because up to four vendors to integrate and monitor; mitigated by ports
  (`SpeechToText`, `Translator`, `TextToSpeech`) and a config-driven routing table.
- Bad, because per-language STT data comes from one vendor-run benchmark, and
  no benchmark measures ElevenLabs v4 cross-lingual speaker similarity.

### Confirmation

Before `accepted`, run the three evaluation matrices from the multilingual
report on 14 languages (en, pt-BR, es, fr, de, sv, ru, ar, hi, zh, ja, ko, vi, th),
**cross-lingual TTS first**:

- STT: enable a route only if the challenger beats Soniox by > 2 WER points
  with no worse finalization latency.
- MT: use Flash for a pair only if quality clearly improves within ~600 ms.
- TTS: enable the Qwen CJK route only if speaker similarity improves by
  ≥ 0.05 cosine, MOS is no worse and time-to-first-audio stays in budget.
