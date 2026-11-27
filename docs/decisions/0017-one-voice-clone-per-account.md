---
status: accepted
date: 2026-11-27
---

# One voice clone per account, reused in every call

Supersedes the "clones are deleted when the call ends" rule of
[ADR 0006](0006-voice-cloning-consent.md) for accounts with a kept recording
([ADR 0015](0015-keep-each-accounts-voice-sample.md)).

## Context and problem statement

Every pre-join re-created the provider's clone from the kept recording, so people waited
3–4 s on "Creating your voice…" at each entry and after every reload, and every entry spent
one of the provider's clone operations (the limit that blocked ElevenLabs).

## Considered options

- Keep one clone per call (re-create at every entry).
- Keep one clone per account at the provider, reused in every call.

## Decision outcome

Chosen option: "one clone per account", because entries become instant and provider
quotas are spent once per person instead of once per call.

- When recordings are kept (`DICERI_VOICE_KEY` set), recording creates the account's clone
  (named `diceri-account-<deployment>-<account id>`) and stores its provider and voice id in
  `account_voices`. Recording again replaces it and deletes the old clone at the provider.
- Pre-join reuses the stored clone; only when there is none for the current provider
  (e.g. after switching providers) is one made from the kept recording.
- The media agent no longer deletes clones when people leave (it reads the same
  `DICERI_VOICE_KEY`), and its startup sweep only matches per-call names
  (`diceri-<deployment>-`), never account clones.
- "Delete my voice" deletes the recording, the stored id and the clone at the provider;
  deleting the account removes the stored id with it (the provider clone must then be
  removed too when account deletion ships).
- Without kept recordings, clones still last only the call, as before.

### Consequences

- Good, because entering a call or reloading is instant once a person has a voice.
- Bad, because each person's clone stays at the provider while they keep a recording: it
  occupies one stored voice (Inworld On-Demand holds 100), and the provider keeps it longer.
  The terms of use say so.

### Confirmation

Unit test: after recording, reusing twice returns the same clone with one provider call;
recording again replaces it. Contract tests run the stored clone against memory and SQLite.
