# Data inventory

Record of the personal data Diceri processes (LGPD Art. 37, GDPR Art. 30). Every
table or store with personal data must be listed here in the same commit that adds it
(AGENTS.md §14, rule 26).

## Durable (SQLite)

| Table.field | Personal data | Purpose | Legal basis | Retention | Deletion path |
| --- | --- | --- | --- | --- | --- |
| `accounts.id` | pseudonymous id (reveals sign-up time) | primary key | contract (providing the account) | while the account exists | account erasure (planned: delete endpoint) |
| `accounts.username` | public handle chosen by the user | show who hosts a room | contract | while the account exists | account erasure |
| `accounts.key_hash` | SHA-256 of the sign-in key (pseudonymous) | sign the owner in | contract | while the account exists | account erasure |
| `accounts.created_at` | sign-up time | operations, abuse handling | legitimate interest | while the account exists | account erasure |

Backups (`VACUUM INTO` snapshots) keep the 7 most recent days; erased rows disappear
from backups within that window.

## In memory only (lost on restart)

| Store | Personal data | Purpose | Retention |
| --- | --- | --- | --- |
| rooms | title, owner username, participants' display names and output languages, host/guest keys | run the meeting | until the API restarts |
| rate limiter | client IP address | stop scripted abuse | about 10 minutes of inactivity |

## Never stored server-side

Audio, transcripts, translations and voice embeddings ([ADR 0006](decisions/0006-voice-cloning-consent.md),
[ADR 0007](decisions/0007-transcripts-stay-in-the-browser.md)). Voice clones live at the
speech provider only for the call and are deleted when the speaker leaves.
