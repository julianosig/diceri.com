---
status: accepted
date: 2026-11-25
---

# Named rooms, an opt-in live board and host-chosen entry

## Context and problem statement

To grow by word of mouth, the home page shows a live board of rooms. A board of
anonymous language codes ("pt · en") does not make anyone want to join. Rooms need a
title and a visible host, but many calls are private conversations that must never
show up publicly.

## Considered options

- Every room is public, with its title and host name.
- Rooms are public by default; hosts can opt out.
- Rooms are private by default; hosts opt in to the board.

## Decision outcome

Chosen option: "private by default, opt in", because listing a call publicly exposes
the host's name and the room link, and that should be a deliberate choice.

- Every room has a title (1–60 characters), and remembers the host's display name once
  they join.
- `listed` (default `false`) puts the room on the board with its title, host, languages,
  size and link. Unlisted rooms only add to the anonymous totals ("N online now").
- `entry` is `approval` (default: the host admits each person) or `open` (anyone who
  asks gets in at once). Open rooms still go through the same knock and credentials
  flow, so the rules live in one place (`Room::knock`).
- The board only lists rooms this deployment knows, so dev and e2e rooms that share
  the LiveKit project never appear in production.

### Consequences

- Good, because the board shows real, joinable rooms and gives people a reason to click.
- Good, because private calls look exactly as before to the outside: no title, no name.
- Bad, because a listed open room can be joined by strangers; hosts choose this
  explicitly, and the per-client write limit caps scripted joins.

### Confirmation

Domain and use case tests (`Room::knock`, `PresenceService`), router tests for
`/rooms` and `/presence`, and the end-to-end test where a visitor walks into a public
open room from the board.
