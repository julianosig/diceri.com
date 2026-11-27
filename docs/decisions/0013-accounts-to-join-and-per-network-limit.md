---
status: accepted
date: 2026-11-27
---

# Every caller has an account, and each network claims only a few

Supersedes the guest model of [ADR 0011](0011-password-accounts-and-personal-rooms.md)
("guests join without an account").

## Context and problem statement

Calls cost real money per minute (speech recognition, translation, synthesis; see
`docs/pricing.md`) and the free capacity is shared. Anonymous guests could join any open
room with nothing to tie usage or abuse to, and nothing stopped one person from claiming
many usernames to multiply the free allowance.

## Considered options

- Keep anonymous guests; limit only hosts.
- Require an account to join, with no limit on accounts.
- Require an account to join, and cap the accounts each network can claim.

## Decision outcome

Chosen option: "require an account to join, and cap the accounts each network can claim",
because it ties every caller to an account and makes mass sign-up costly.

- Asking to join a room (`POST /rooms/{room}/requests`, which also admits into open
  rooms) needs a signed-in session. A signed-out visitor sees the same sign-in / sign-up
  flow as the home page, titled "Join the meeting", before pre-join.
- A network (an IPv4 address, or an IPv6 /64) may claim `DICERI_ACCOUNTS_PER_NETWORK`
  usernames (default 3). Only an HMAC-SHA256 of the network under a server secret
  (`DICERI_NETWORK_KEY`) is stored, never the address (AGENTS.md §14 rule 25), and it is
  deleted with the account. Without the key (development, tests) there is no cap.

### Consequences

- Good, because every minute of a call belongs to an account, and abuse costs real effort.
- Bad, because joining takes a sign-up first, which loses some casual guests.
- Bad, because many people behind one address (mobile carrier NAT, offices, campuses)
  share the cap; raise `DICERI_ACCOUNTS_PER_NETWORK` if that bites.
- Accounts created before the cap have no recorded network and do not count toward it.

### Confirmation

Router tests: asking to join without a session answers 401; a network past its cap
answers 429 with `limit: network`. Contract tests run the cap against both account
stores. End-to-end: a visitor signs up from the room page, then joins.
