---
status: accepted
date: 2026-11-25
---

# Username-only accounts stored in SQLite

## Context and problem statement

Rooms on the live board should show who owns them with a stable handle (`@ana`),
not a display name anyone can retype. Sign-up must stay as light as the rest of
the product: no email, no password, no third-party login.

## Considered options

- Email or OAuth sign-in.
- Username plus password.
- Username only: claiming a free name returns a secret key that is the session
  and the way to sign in on another device.

## Decision outcome

Chosen option: "username only", because it takes one field and one click.

- `Username`: 3–20 of `a-z`, `0-9`, `_`, starting with a letter, case-insensitive,
  a few reserved names (`admin`, `diceri`, …).
- The key is 128 random bits (`AccessKey`), returned once on sign-up and kept in an
  `HttpOnly; SameSite=Lax` cookie (`Secure` in production) scoped to `/api`.
  Storage keeps only its SHA-256; a slow hash is unnecessary for random keys.
- Creating a room requires an account; the room's `owner` is shown as its host.
  Joining a room still needs no account.
- Accounts are the first durable data, so they live in SQLite (`sqlx`, no compile-time
  macros) behind the `AccountRepository` port. Rooms stay in memory.

### Consequences

- Good, because sign-up is instant and nothing personal is stored.
- Good, because a leaked database does not let anyone sign in.
- Bad, because losing the browser's cookie and the shown code loses the account; the
  UI shows the code once and asks the user to save it.
- Bad, because usernames can be squatted; the per-client write limit and a Cloudflare
  rate-limit rule on sign-up keep it slow.

### Confirmation

Domain tests for `Username`, adapter tests on an in-memory SQLite (unique names, keys
stored hashed), router tests for `/accounts` and for `POST /rooms` requiring a session.
