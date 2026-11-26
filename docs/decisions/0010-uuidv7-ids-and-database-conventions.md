---
status: accepted
date: 2026-11-26
---

# UUIDv7 ids, room link codes and database conventions

## Context and problem statement

Ids were 128-bit random base62 strings, and the room id doubled as the shareable link.
We want standard, time-ordered ids (good index locality, ready for PostgreSQL) and a
written set of database rules before more durable tables appear. Research:
[`docs/research/2026-11-database-design/report.md`](../research/2026-11-database-design/report.md).

The catch: RFC 9562 says UUIDs "MUST NOT be used as security capabilities", a v7 id
reveals its creation time, and a room link *is* a capability (anyone with the link of
an open room gets in, see ADR 0008).

## Considered options

- Keep random base62 ids everywhere.
- Make every id, including the room link, a UUIDv7.
- UUIDv7 for every id; keep the room link as a separate 128-bit random code.

## Decision outcome

Chosen option: "UUIDv7 ids plus a separate room code".

- `domain` has typed ids (`RoomId`, `RequestId`, `AccountId`, `ParticipantId`) wrapping a
  v7 `Uuid`; parsing rejects other versions. `domain` may now depend on `uuid` (pure:
  parsing and building from given parts only).
- The application gets ids from an `IdGenerator` port (clock + randomness); `infra`
  implements it with the system clock and the OS CSPRNG (74 fresh random bits per id).
- The old random room id becomes `RoomCode`: the `/r/<code>` link and the key for every
  room API path. It is never logged (redacted `Debug`), never sent to LiveKit or
  providers: media rooms and voice clones are named by `RoomId`.
- The accounts table gets a v7 `id` primary key, `username` as a named `UNIQUE`
  constraint with a `CHECK` mirroring `Username`, `created_at` in milliseconds, and moves
  to versioned `sqlx::migrate!` migrations with explicit SQLite connection settings.
- The general rules are in AGENTS.md §14.

### Consequences

- Good, because ids are standard, sortable by time and map 1:1 to PostgreSQL `uuid`.
- Good, because links keep 128 bits of entropy and stop appearing in logs and in the
  media server.
- Bad, because rooms carry two identifiers; the rule "paths take the code, everything
  else the id" must be followed.
- Bad, because the first migration replaces the startup `CREATE TABLE`; existing
  databases (only test accounts existed) were recreated rather than migrated.

### Confirmation

Domain tests (v7 parsing, ordering, redacted `RoomCode`), an `AccountRepository`
contract suite run against both adapters, adapter tests that read the stored bytes and
check constraint enforcement, router tests asserting a v7 `id` and a 22-char `code`, and
the end-to-end suite.
