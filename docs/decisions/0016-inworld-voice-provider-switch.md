---
status: proposed
date: 2026-11-27
---

# Inworld as a switchable voice provider

Extends [ADR 0005](0005-speech-translation-provider-stack.md) (ElevenLabs for cloning and
speech); does not yet replace it.

## Context and problem statement

ElevenLabs counts every voice clone toward a monthly cap (95 on Creator). Diceri clones each
participant per call, so the cap ran out and, with a recorded voice required to join (ADR
0014), every call was blocked. Research ([provider comparison](../research/2026-11-voice-cloning-tts/report.md),
[Inworld limits](../research/2026-11-inworld-custom-voices/report.md)) found Inworld TTS-2
close to ElevenLabs on cloned-voice quality, faster and at least 2x cheaper, and counting
voices held at once (freed on delete) instead of voices created.

## Considered options

- Stay on ElevenLabs and buy a larger plan.
- Switch to Inworld outright.
- Add Inworld behind a switch, evaluate it, then decide.

## Decision outcome

Chosen option: "add Inworld behind a switch", because it unblocks calls now without
committing before the blind evaluation the research asks for.

- `DICERI_TTS_PROVIDER` (`elevenlabs` default, or `inworld`) picks the cloner in the API and
  the synthesizer and cloner in the media agent; both must agree. `DICERI_INWORLD_API_KEY`
  holds the key.
- Inworld clones are created from the kept recording (ADR 0015) with the same
  create-on-join, delete-on-leave flow; the startup sweep deletes leftovers by name prefix.
- Translations are spoken with `inworld-tts-2`, 24 kHz PCM, the language set on every
  request.
- The terms of use list Inworld as a provider.

### Consequences

- Good, because calls work again, cost per call-minute roughly halves, and the provider can
  be switched back with one setting.
- Bad, because Inworld keeps cloning samples outside its zero-data-retention, hosts in the
  US, and its DPA has no LGPD clause; the questions in the Inworld research must be answered
  before this becomes the default for everyone.
- Bad, because the On-Demand and Creator plans clone 2–3 voices per minute and guarantee
  5–10 concurrent generations: enough for testing, not for a public launch.

### Confirmation

Adapter unit tests (stream parsing, error mapping); a live probe from the production host
(clone, stream pt-BR and ja-JP, delete); the blind evaluation in the provider research
decides whether this ADR is accepted.
