---
status: accepted (guest model superseded by 0013)
date: 2026-11-26
---

# Password accounts and one personal room per username

Supersedes the account model of [ADR 0009](0009-username-only-accounts.md) (a shown-once
key instead of a password) and the room titles of
[ADR 0008](0008-named-rooms-and-live-board.md).

## Context and problem statement

A one-time sign-in code was easy to lose, and free-form room titles made the board
noisy. People want a handle that *is* their place: `@ana` should be Ana's room, every
time, with her mascot.

## Considered options

- Keep key-only accounts and titled rooms.
- Username + password, still with titled, one-off rooms.
- Username + password, one personal room per account, named by the username.

## Decision outcome

Chosen option: "username + password, one personal room per account".

- Onboarding is one step: username, password (typed twice in the UI), mascot. No email.
- Passwords are user-chosen, so they are hashed with **Argon2id** at the OWASP minimum
  (19 MiB, t=2, p=1) on the blocking pool (AGENTS.md §14 rule 3). Length 8–128, no
  composition rules (NIST SP 800-63B).
- Sign-in creates a **session**: a 128-bit random token in an `HttpOnly; SameSite=Lax`
  cookie, stored as SHA-256 in `sessions` (expires after 30 days, deleted on sign-out,
  cascades with the account, expired rows purged on sign-in).
- Each account has one room, named and reached by its username (`/@ana`). `POST /rooms`
  opens it, or reopens it with new options while it is live; `GET /users/{name}/room`
  finds it. The mascot comes from the account. Titles are removed.
- The room's code still keys every lobby path, but `/@ana` is guessable, so the entry
  policy is what protects a room: approval is the default and "anyone can join" says
  so explicitly in the UI.

### Consequences

- Good, because people get a stable, shareable identity and room in one step.
- Good, because a stolen database reveals neither passwords nor live sessions.
- Bad, because a forgotten password has no recovery (no email); a reset flow needs a
  later decision.
- Bad, because a guessable room address invites uninvited knocks; approval by default,
  the write limit and Cloudflare rules keep it manageable.

### Confirmation

Argon2id adapter tests, an `AccountRepository` contract suite (sessions included) run
against the in-memory and SQLite adapters, schema tests for the new constraints and
cascade, router tests for sign-up validation, sign-in and `/users/{name}/room`, and the
end-to-end onboarding and `/@username` journeys.
