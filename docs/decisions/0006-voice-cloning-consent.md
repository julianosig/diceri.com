---
status: accepted (stock-voice fallback superseded by 0014)
date: 2026-11-25
---

# Explicit, per-call consent for voice cloning

## Context and problem statement

Cloning an anonymous participant's voice is biometric processing. All TTS
vendors require the voice owner's consent; EU AI Act transparency duties apply
since 2026-08-02 and laws such as Illinois BIPA treat voiceprints as biometrics.

## Considered options

- Opt-in in the lobby with a read-aloud consent sentence
- Implicit cloning from the first seconds of speech
- No cloning (stock voices only)

## Decision outcome

Chosen option: "opt-in in the lobby". The speaker reads a short consent
sentence; that recording (10–15 s) is also the clone reference. Without
consent, a stock voice matched by language is used. Clones are deleted through
the vendor API when the participant leaves; reference audio is never stored by us.
Listeners are told the voice is AI-generated.

### Consequences

- Good, because it is legally defensible and transparent.
- Bad, because it adds one step before joining.

### Implementation (2026-11-25)

- Lobby: optional "Record my voice (10 s)". The user reads a short script whose
  first sentence is the spoken consent; recording stops itself at 10 s (5 s
  minimum). The sample becomes an ElevenLabs instant clone owned by the room
  (`POST /rooms/{room}/voices`, max 10 per room); a participant may only join
  with a voice recorded in the same room.
- The agent speaks that participant's translations in the clone; if the
  provider refuses it (e.g. verification required), it switches to a stock
  voice for the rest of the call instead of going silent.
- Deletion: when the participant leaves, when the session ends, and, on agent
  startup, every `diceri-*` clone (orphans from crashes or restarts).

### Confirmation

Integration test: no clone API call without a consent record; clone deletion runs on leave and on call end.
