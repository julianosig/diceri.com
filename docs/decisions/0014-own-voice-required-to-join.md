---
status: accepted
date: 2026-11-27
---

# Everyone speaks in their own recorded voice

Supersedes the stock-voice fallback of [ADR 0006](0006-voice-cloning-consent.md)
("without consent, use a stock voice"). Consent stays explicit and per call.

## Context and problem statement

Without a recording, translations were spoken in a stock voice picked per speaker, so a
woman could be heard in a man's voice. That breaks the product's promise (your voice, in
their language) and is confusing in a call.

## Considered options

- Keep the stock voice, matched to the speaker's apparent gender.
- Keep recording optional.
- Require a short recording (with consent) to join, and never use a stock voice.

## Decision outcome

Chosen option: "require a recording to join, never a stock voice", because a voice that
is not the speaker's misrepresents them, and guessing gender is unreliable and intrusive.

- With `DICERI_VOICE_REQUIRED=true` (and voice cloning configured), joining a call
  (host or guest) needs a voice recorded in that room: the API answers 400 otherwise and
  pre-join keeps "Join now" / "Ask to join" disabled until the voice is ready.
- The recording flow is unchanged: about 10 seconds reading a sentence that states the
  consent, checked for clear human speech before cloning, deleted when the call ends.
- The media agent speaks only in the speaker's clone. Without one, or once the provider
  refuses it (e.g. a voice that needs verification), that speaker's translations are
  captions only. Stock voices (`DICERI_TTS_VOICES`) are gone.

### Consequences

- Good, because every translated voice is the speaker's own, with their consent.
- Bad, because joining takes about 15 seconds more, and someone who will not record
  cannot join.
- Bad, because each participant creates a provider voice clone per call (time, quota of
  voice slots); clones are deleted when the call ends, as before.
- End-to-end tests keep recording on and the requirement off (each test would otherwise
  clone voices at the provider); router tests cover the requirement.

### Confirmation

Router test: with the requirement on, joining without a voice answers 400 and with one
recorded in the room succeeds; capabilities report `voiceRequired`. Media tests: a refused
clone leaves the speaker captions-only and no stock voice is ever used.
