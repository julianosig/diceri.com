---
status: accepted
date: 2026-11-27
---

# Keep each account's voice recording, encrypted

Amends AGENTS.md §14 rule 25 ("no column may hold audio") for one table, and extends
[ADR 0014](0014-own-voice-required-to-join.md): consent now comes from the terms of use.

## Context and problem statement

Every call needs the speaker's own voice (ADR 0014), and today the 10-second recording
is thrown away once ElevenLabs has cloned it. That ties every voice to one provider: if we
moved to another voice model we could not recreate anyone's voice, and people record
again at every call.

## Considered options

- Keep nothing; ask everyone to record again after a provider change.
- Keep the provider's clone only (no audio).
- Keep the recording with the account, encrypted, and clone it again when needed.

## Decision outcome

Chosen option: "keep the recording with the account, encrypted", because it is the only
way to recreate voices with another provider, and it lets people record once.

- One recording per account in `voice_samples`, replaced when they record again. It is
  stored only after the existing checks pass (clear human speech, then words heard).
- Encrypted with AES-256-GCM under `DICERI_VOICE_KEY` (a server secret, never in the
  database or its backups), bound to the account id as associated data. Without the key
  recordings are not kept. Losing the key loses the recordings.
- Kept while the account exists: deleted with the account (cascade) or at any time with
  "Delete my voice" in the account menu (`DELETE /accounts/me/voice`).
- Reused: pre-join clones the saved recording for each call (the provider clone is still
  deleted when the call ends); recording again replaces it.
- Consent is given through the terms of use (`/terms`), accepted when creating an
  account and stated next to the recording. The read-aloud text no longer contains a
  spoken consent sentence.
- The recording is used only to recreate that person's voice for their own translations,
  with the current or a future voice provider. It is never logged.

### Consequences

- Good, because voices survive a change of provider and people record only once.
- Bad, because the database now holds biometric data: a leak of the database together
  with the key would expose voices. The key lives only in the server's environment file.
- Bad, because consent rests on the terms of use; under LGPD and GDPR consent for
  sensitive data must be specific and prominent, so the notice names the voice explicitly.

### Confirmation

Contract tests run the store against memory and SQLite; the SQLite adapter test checks
the stored bytes are not the audio and that another account's id cannot decrypt them.
Router tests: recording saves the sample, reuse clones it, deleting removes it.
