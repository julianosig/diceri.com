---
status: accepted
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

### Confirmation

Integration test: no clone API call without a consent record; clone deletion runs on leave and on call end.
