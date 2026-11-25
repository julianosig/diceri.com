---
status: accepted
date: 2026-11-25
---

# Call transcripts are kept in each participant's browser

## Context and problem statement

Participants want a transcript of the call (what was said, in the original
language and every translation) when it ends: the host when they end it for
everyone, and anyone else up to the moment they leave. Rooms are anonymous and
`AGENTS.md` forbids storing or logging transcripts on servers.

## Considered options

- Record in each browser from the captions the agent already broadcasts
- Store transcripts server-side per room and serve them at the end

## Decision outcome

Chosen option: "record in each browser". The agent already sends every caption
(originals and translations) to every participant; each browser folds them into
a transcript (`TranscriptLog`) from the moment it joined. When the participant
leaves, or the host ends the meeting for everyone (`POST /rooms/{room}/end`,
which deletes the LiveKit room and refuses new joins with 410 Gone), the end
screen shows the transcript with copy and download (.txt).

### Consequences

- Good, because no conversation content is stored or logged on our servers.
- Good, because it needs no new backend storage, retention policy or access control.
- Bad, because someone who joins late only gets what was said after they joined,
  and a page reload loses the transcript collected so far.
- Bad, because translations exist only for the languages present in the room.

### Confirmation

`translation.e2e.ts` ends a two-language call as host and checks both end
screens show originals and translations.
