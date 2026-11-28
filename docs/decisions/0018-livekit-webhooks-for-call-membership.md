---
status: accepted
date: 2026-11-28
---

# LiveKit webhooks enforce who may stay in a call

## Context and problem statement

Calls are kept to their size (seats), to the service's capacity, and to the people actually
in them (hidden listeners) by checks made when credentials are issued and by a sweep once a
minute. Some joins never pass through those checks: a token issued just before the host
ended the call, LiveKit's own reconnect with a token it refreshed itself, or a listener whose
person has just left. Twenty-five review rounds kept finding edge cases in the polling,
because the API only learns who connected up to a minute later.

## Considered options

- Keep polling LiveKit's presence API once a minute and tighten its edge cases.
- Receive LiveKit webhooks (`participant_joined`, `participant_left`, `room_finished`) and act
  on each connection as it happens, keeping the minute's sweep as a backstop.

## Decision outcome

Chosen option: "receive LiveKit webhooks", because every media connection, including
reconnects and late tokens, then reaches the API the moment it happens.

- `POST /api/v1/livekit/webhook` verifies LiveKit's signature (a JWT signed with the
  project's API key, carrying the SHA-256 of the body) and ignores rooms this API does not
  know: development and end-to-end runs share the LiveKit project.
- `participant_joined`: a call that has ended is closed again; a person beyond the call's size
  (other than its owner) is removed; a hidden listener whose person is not in the call is
  removed; the call counts against the service's capacity at once.
- `participant_left`: the person's hidden listener (`monitor-<seat>`) is removed at once.
- `room_finished`: the call stops counting against the capacity.
- Webhook delivery is not guaranteed (LiveKit retries, then gives up), so the minute's sweep
  stays as a backstop, and every action is idempotent (removing someone already gone is fine).
- The webhook URL and signing key are set in the LiveKit Cloud project
  (Settings → Webhooks): `https://diceri.com/api/v1/livekit/webhook`, signed with the key in
  `DICERI_LIVEKIT_API_KEY`.

### Consequences

- Good, because late tokens, refreshed-token reconnects and lingering listeners are handled
  within the webhook's delivery time instead of up to a minute later.
- Good, because capacity sees a call the moment someone connects.
- Bad, because a new public endpoint must be kept safe: it only acts on signed events about
  rooms it knows, and only to remove people or close rooms, never to let anyone in.
- Bad, because it relies on the LiveKit Cloud configuration staying in place; without it
  the minute's sweep still enforces everything, only later.

### Confirmation

Unit tests for each event against the room's rules; router tests for the signature check
(unsigned, wrong key, wrong body hash) and for unknown rooms; a signed test event from the
LiveKit Cloud dashboard after deployment.
