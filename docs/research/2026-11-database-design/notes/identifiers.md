# Identifier and key design with UUIDv7 (as of November 2026)

Context: Rust backend (axum, tokio, sqlx 0.9.1, SQLite now, Postgres maybe later). Today ids are
128-bit random values in 22-char base62; room ids appear in shareable URLs (`/r/<id>`), and anyone
holding the link can reach the room. Secrets (host keys, account keys) are separate 128-bit random
bearer keys stored as SHA-256. The team has decided to use UUIDv7 for id fields.

Versions checked: RFC 9562 (May 2024), PostgreSQL 18 (released 2025-09-25), `uuid` crate 1.29.3,
`sqlx` / `sqlx-sqlite` 0.9.1 (latest on docs.rs on 2026-11-25).

## RFC 9562 UUIDv7: layout, monotonicity, what it leaks, guessability, ids vs secrets

### Takeaway
UUIDv7 is a 48-bit Unix-millisecond timestamp, then 74 bits that are random or partly a counter.
It reveals when the id was created, and the RFC says outright that UUIDs "MUST NOT be used as
security capabilities". A v7 id is fine as an identifier, but it must not be the thing that grants
access to a room.

### Cited Findings
- Layout (§5.7): `unix_ts_ms` bits 0-47 ("48-bit big-endian unsigned number of the Unix Epoch
  timestamp in milliseconds"), `ver` bits 48-51 = `0b0111`, `rand_a` bits 52-63 (12 bits, random or
  used for monotonicity), `var` bits 64-65 = `0b10`, `rand_b` bits 66-127 (62 bits, random or an
  optional counter). That gives 74 bits that are not timestamp, version or variant. UUIDv4 has 122
  random bits. — [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html)
- Monotonicity (§6.2) offers three methods.
  - Method 1: a fixed-length dedicated counter, which "MUST be positioned immediately after the
    embedded timestamp". It should be at least 12 bits and at most 42.
  - Method 2: "monotonic random", where the random part is a randomly seeded counter.
    "incrementing the counter by 1 SHOULD NOT be used by implementations that favor
    unguessability".
  - Method 3: replace up to 12 leftmost random bits with sub-millisecond clock precision.
  - Rollovers "MUST be handled by the application". Implementations "SHOULD check if the currently
    generated UUID is greater than the previously generated UUID".

  — [RFC 9562 §6.2](https://www.rfc-editor.org/rfc/rfc9562.html)
- §6.1: "Implementations MAY alter the actual timestamp". The RFC "makes no requirement or guarantee
  about how close the clock value needs to be to the actual time". — [RFC 9562 §6.1](https://www.rfc-editor.org/rfc/rfc9562.html)
- §6.9: implementations "SHOULD utilize a cryptographically secure pseudorandom number generator
  (CSPRNG)" and must reseed after a fork. — [RFC 9562 §6.9](https://www.rfc-editor.org/rfc/rfc9562.html)
- §8 Security: implementations should not assume UUIDs are hard to guess. UUIDs "MUST NOT be used as
  security capabilities". "Timestamps embedded in the UUID do pose a very small attack surface". If
  UUIDs are used in a security operation, UUIDv4 is recommended. — [RFC 9562 §8](https://www.rfc-editor.org/rfc/rfc9562.html)
- §6.12: "treat UUIDs as opaquely as possible" and avoid parsing them unless necessary. §6.11: v6 and
  v7 "sort as opaque raw bytes without the need for parsing or introspection". — [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html)
- Practitioner view: v7 ids "can often be decoded to reveal an approximate creation time". Ordering
  and rate information can sometimes be inferred from sequences of ids. Suggested mitigations: use v4
  for external ids, or keep v7 internal and expose a separate opaque token. —
  [Clockworks security notes](https://dexcompiler.github.io/Clockworks/concepts/security.html)
- Counterpoint (informal forum comment): if creation time is already exposed through API timestamp
  fields, leaking it through the id adds nothing. — [HN mirror](https://web-tau-bice-50.vercel.app/item/45593358)

### Inferences
- **Room links are capabilities.** In Diceri, knowing `/r/<id>` is enough to reach a room (the join
  request is a knock, but the id itself is what is being protected). §8 rules out using a UUIDv7
  as that capability. With the `uuid` crate's `now_v7()`, about 32 bits per id are freshly random
  (74 bits minus the 42-bit counter, see the next section). The counter is seeded randomly each
  millisecond, but later ids in the same millisecond are derived from it. That is far below today's
  128 random bits.
- **Recommendation:** use UUIDv7 as the internal primary key and foreign key for rooms. Keep the
  shareable room slug as a separate 128-bit CSPRNG value with a `UNIQUE` index: the existing
  22-char base62, or a v4. Treat it the same way as today's bearer keys, except that it does not need
  hashing if it is only a "knock" handle. Hash it like the host keys if possession alone grants entry.
  Use v7 directly in URLs only for resources whose access is already checked by another credential
  (for example a join-request id behind the host key).
- Creation-time leakage matters little for ephemeral rooms. It is still a privacy signal: it tells
  an outsider when an account or room was made. That is another reason not to expose account v7 ids.
- Do not rely on `with_adjust_by_millis`-style offsets (§6.1) for secrecy. They hide the time but
  do not add entropy.

### Gaps
- No authoritative quantitative analysis of v7 guessability per library was found. The roughly 32
  random-bit figure is derived from the `uuid` crate docs, not stated by them.

## Why UUIDv7 for primary keys: index locality, benchmarks, comparison with ULID, Snowflake, bigint

### Takeaway
UUIDv7 keys are inserted in time order, so new entries land at the right edge of the B-tree. That
is how it fixes the locality problem of v4 keys: the published reports show much smaller indexes
and much less WAL. The cost compared with bigint is 16 bytes instead of 8, which the sources call
modest. ULID has the same layout idea, but v7 is the IETF standard and has native Postgres 18
support.

### Cited Findings
- Buildkite (Gordon Chan, 2023-09-25, older than 2024 but widely cited) moved all new tables to
  UUIDv7 primary keys. Random v4 values "will not be clustered … inserts will be performed at random
  locations". Time-ordered ids gave "a 50% reduction in the WAL … rate of the primary database" and a
  similar drop in write I/O. They chose v7 over integers because integers "need coordination" when
  sharding, and called the 128-bit vs 64-bit overhead marginal. Setting version 7 broke some
  customers that validated UUIDs. — [Buildkite](https://buildkite.engineering/goodbye-integers-hello-uuids/)
- PostgreSQL 18 (released 2025-09-25) adds `uuidv7()` ("temporally sortable") and a `uuidv4()` alias
  (Andrey Borodin). — [PG 18 release notes](https://www.postgresql.org/docs/release/18.0/)
- `uuidv7([shift interval])`: "UNIX timestamp with millisecond precision + sub-millisecond timestamp
  + random". The optional shift moves the timestamp. `uuid_extract_timestamp()` and
  `uuid_extract_version()` are also available. — [PG 18 docs, UUID functions](https://www.postgresql.org/docs/18/functions-uuid.html)
- The PG commit (Masahiko Sawada, 2024-12-11) uses a 12-bit sub-millisecond fraction in `rand_a` so
  that values are monotonic "within the same backend", even if the clock goes backwards. Monotonicity
  is not guaranteed across backends. — [brandur.org, 2024-12-31](https://brandur.org/fragments/uuid-v7-monotonicity);
  [depesz](https://www.depesz.com/tag/uuidv7/)
- Cybertec identifies lost temporal locality as the main downside of random UUID keys and recommends
  a sequential variant such as v7. — [Cybertec](https://www.cybertec-postgresql.com/en/?p=47041)
- Benchmarks (community blogs, uneven methodology, so treat them as directional):
  - v7 inserts were about 10x faster than v4 at 100M rows on PG 18, and the v7 index was about a
    quarter smaller. — [dev.to/regyl](https://dev.to/regyl/postgresql-uuidv4-vs-uuidv7-1pdf)
  - v4 used up to 35% more space at 100M rows. — [TigerData](https://www.tigerdata.com/blog/how-sequential-uuidv7-boosts-ingestion-performance)
  - Leaf density was about 58% for v4 and 91% for v7. — [dev.to/ahmet_gedik](https://dev.to/ahmet_gedik778845/how-uuid-v7-primary-keys-cut-b-tree-bloat-in-our-video-store-585n)
  - Results were nearly equal at small sizes.
- Storage cost: a UUID takes twice the space of a bigint (Laurenz Albe, Cybertec, 2021). —
  [pgsql-general](https://postgrespro.com/list/id/69a5b7b83689ab456dd4b3cd7eb08e9573652039.camel@cybertec.at)
- ULID: 48-bit ms timestamp plus 80 random bits, written as 26 chars of Crockford base32. Within one
  millisecond the random part is incremented by 1, and generation fails on overflow. It is
  "128-bit compatibility with UUID", but it has no version or variant bits, so it is not an
  RFC UUID. — [ULID spec](https://github.com/ulid/spec)
- RFC 9562 §6.13: store UUIDs as their 128-bit binary value where feasible (the text form is 288
  bits). Avoid name-based v3 and v5 as keys, and prefer time-based surrogate keys. —
  [RFC 9562 §6.13](https://www.rfc-editor.org/rfc/rfc9562.html)

### Inferences
- At Diceri's scale (rooms, join requests, accounts on SQLite), the gain from v7 over random ids is
  real but small. The stronger reasons to choose v7 are standardization, a native PG 18 type and
  function, and time order that is useful for listing and pagination.
- Snowflake-style 64-bit ids need worker-id coordination, and bigint identity needs the database to
  assign the id. Neither is needed for a single API instance. v7 lets ids be generated in the app
  before insert, which suits the ports/adapters design.
- Prefer v7 over ULID: both have the same ordering properties, but only v7 is the standard, with
  native Postgres support and `uuid`/`sqlx` integration.

### Gaps
- No vendor-neutral, reproducible PG 18 benchmark with full methodology was found.
- No SQLite-specific v4 vs v7 benchmark was found.
- Could not retrieve Cybertec's "Unexpected downsides of UUID keys" article directly.
- No MySQL source was checked; it is not relevant to the stack.

## Storage: SQLite (BLOB vs TEXT, STRICT, ordering), Postgres `uuid`, sqlx 0.9 encoding

### Takeaway
In SQLite, store ids as 16-byte `BLOB` in `STRICT` tables. sqlx 0.9.1 encodes `uuid::Uuid` as a
blob and decodes it only from blob bytes. BLOBs compare with `memcmp`, so v7 ids sort by time. In
Postgres, use the native `uuid` type, which is the same `Uuid` in Rust.

### Cited Findings
- `sqlx-sqlite` 0.9.1 `types/uuid.rs` for `Uuid`:
  - `type_info` = Blob; `compatible` accepts Blob or Text.
  - Encode pushes `self.as_bytes().to_vec()` as a Blob.
  - Decode is `Uuid::from_slice(value.blob_borrowed())`, with no text parsing.
  - `uuid::fmt::Hyphenated` and `uuid::fmt::Simple` encode and decode as TEXT, using `parse_str` on
    decode.

  — [sqlx-sqlite 0.9.1 source](https://docs.rs/crate/sqlx-sqlite/latest/source/src/types/uuid.rs)
- The sqlx 0.9.1 SQLite type table lists `uuid::Uuid` as "BLOB, TEXT" and the `Hyphenated` and
  `Simple` wrappers as TEXT. All require the `uuid` Cargo feature. — [sqlx 0.9.1 SQLite types](https://docs.rs/sqlx/latest/sqlx/sqlite/types/index.html)
- sqlx 0.9.1 Postgres maps `uuid::Uuid` to `UUID` and "Requires the `uuid` Cargo feature flag". —
  [sqlx 0.9.1 Postgres types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)
- SQLite: "When two BLOB values are compared, the result is determined using memcmp()". Collations
  apply only to TEXT. The cross-class order is NULL < INTEGER/REAL < TEXT < BLOB. —
  [SQLite datatypes](https://www.sqlite.org/datatype3.html)
- STRICT tables allow only `INT, INTEGER, REAL, TEXT, BLOB, ANY`. — [SQLite STRICT](https://www.sqlite.org/stricttables.html)
- On a rowid table, a non-INTEGER primary key "is really just a UNIQUE index": a separate B-tree,
  with the key stored twice. `WITHOUT ROWID` stores the table in one B-tree keyed on the PK. It
  helps "tables that have non-integer or composite … PRIMARY KEYs" when rows are smaller than about
  1/20 of a page (about 200 bytes at 4 KiB). — [SQLite WITHOUT ROWID](https://www.sqlite.org/withoutrowid.html)
- Postgres `uuid` accepts upper case, braces and missing hyphens on input. "Output is always in the
  standard form" (lowercase, hyphenated). — [PG 18 uuid type](https://www.postgresql.org/docs/18/datatype-uuid.html)

### Inferences
- Schema pattern for SQLite:
  `id BLOB PRIMARY KEY NOT NULL CHECK (length(id) = 16)` in a `STRICT` table. The `CHECK` stops a
  stray TEXT value: `STRICT` would reject TEXT in a BLOB column anyway, but `length` also catches
  wrong-size blobs. Consider `WITHOUT ROWID` for small tables keyed only by id or by a composite
  key, such as join requests or membership. Measure before adopting it, as SQLite advises.
- Never mix TEXT and BLOB representations in one column. A TEXT id decoded as `Uuid` will fail
  (inference from the decode code), and TEXT always sorts before BLOB.
- For manual inspection in the `sqlite3` CLI, use `hex(id)`, or the `uuid` extension's
  `uuid_str()`.
- The Postgres migration path is clean: `BLOB(16)` maps to `uuid` and the Rust type stays `Uuid`.
  The only change needed is the migration DDL.

### Gaps
- The sqlx 0.9 changelog could not be fetched to confirm whether the 0.9 release changed uuid
  encoding. The 0.9.1 source shows the blob behavior described above.

## Rust: `uuid` crate, `now_v7`, ContextV7, serde, app vs DB generation, testability

### Takeaway
Use `uuid` 1.x with features `v7` and `serde`, and generate ids in the application with
`Uuid::now_v7()`. Ids it generates within one process are ordered. For tests, hide id creation
behind a small `IdGenerator` port, or build ids with `Uuid::new_v7(Timestamp::from_unix(...))`.

### Cited Findings
- `uuid` 1.29.3, `Uuid::now_v7()` (features `std` and `v7`): "All UUIDs generated through this method
  by the same process are guaranteed to be ordered by their creation". It calls
  `new_v7(Timestamp::now(shared_context_v7()))`. — [docs.rs uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html);
  [v7.rs source](https://docs.rs/uuid/latest/src/uuid/v7.rs.html)
- `new_v7(ts)`: "The 74 most significant bits of the counter value are retained" and the rest is
  random. The docs advise: "Avoid using a counter wider than 74 bits". — [docs.rs uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)
- `ContextV7` provides:
  - "An unsynchronized, reseeding counter that produces 42-bit values". It is reseeded every 1 ms
    with a random 41-bit value. "The only guarantee is monotonicity".
  - On counter wrap, the timestamp is incremented.
  - `with_additional_precision()` uses 12 counter bits for sub-ms time. `with_adjust_by_millis()`
    shifts timestamps "to obfuscate their actual generation time".
  - It is `!Sync` and "needs to be wrapped in a mutex to share".

  — [docs.rs ContextV7](https://docs.rs/uuid/latest/uuid/timestamp/context/struct.ContextV7.html)
- `get_version()` returns `Option<Version>`. `get_timestamp()` returns `Some` for v1, v6 and v7,
  "truncated to millisecond precision for version 7". — [docs.rs uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)
- Feature flags: `v4`, `v7`, `serde`, `fast-rng`, `rng-getrandom`. There is a `uuid::serde` module
  with "Adapters for alternative serde formats". — [docs.rs uuid crate root](https://docs.rs/uuid/latest/uuid/index.html)
- `Display` is lowercase hyphenated. `parse_str` and `try_parse` accept simple, hyphenated, URN and
  braced forms. — [docs.rs uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)
- RFC §6.2: when several generators exist (for example several processes, or Postgres backends),
  monotonicity is per generator only. — [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html);
  [brandur.org](https://brandur.org/fragments/uuid-v7-monotonicity)

### Inferences
- **Generate in the app, not the database.**
  - SQLite has no native v7 function.
  - App-side generation works the same on SQLite and Postgres.
  - It lets use cases know the id before insert, which avoids `RETURNING` round trips.
  - It keeps id creation inside `application` behind a port.
  - On Postgres, `DEFAULT uuidv7()` can be added as a safety net, but the app should still supply
    the value.
- **Domain type.** Wrap the value in newtypes (`RoomId(Uuid)`, `JoinRequestId(Uuid)`) in `domain`.
  `uuid` with no default features is a pure, I/O-free dependency, which is acceptable for the domain
  crate. Implement `sqlx::Type`, `Encode` and `Decode` (or use `#[sqlx(transparent)]`) in `infra`
  only, so `domain` does not depend on sqlx.
- **Testability.** Define `trait IdGenerator { fn new_id(&self) -> Uuid; }` (or add it to the
  existing clock port). Production calls `Uuid::now_v7()`. Tests use a sequence generator that
  builds ids from a fake clock with `Uuid::new_v7(Timestamp::from_unix(&ctx, secs, nanos))`, or
  with `Uuid::from_u128`. This keeps tests deterministic and ordering assertions stable.
- Do not create a fresh `ContextV7` per call: monotonicity comes from a shared context. Use
  `now_v7()`, or one context behind a mutex.
- Serde: the default output is a hyphenated string in JSON, which is what the generated OpenAPI or
  TypeScript types expect. Add `format: uuid` to the schema.

### Gaps
- The `shared_context_v7()` source (a mutex-wrapped static or not) and the counter increment step
  used by `now_v7` were not confirmed from the docs.
- The exact serde behavior for non-human-readable formats was not verified on the fetched page.

## External representation: canonical vs compact, parsing strictness, case

### Takeaway
Expose the canonical lowercase hyphenated form in JSON and in URLs that are not shareable. Accept
any case when parsing, as RFC 9562 requires, and reject any version other than 7 at API boundaries
that accept ids we minted. Keep shareable room links on a separate short random slug.

### Cited Findings
- RFC 9562 §4: hex "may be all uppercase, all lowercase, or mixed case". The text form is
  hex-and-dash. — [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html)
- Postgres always outputs the lowercase standard form and accepts upper case, braces and missing
  hyphens on input. — [PG 18 uuid type](https://www.postgresql.org/docs/18/datatype-uuid.html)
- `uuid` `parse_str` accepts simple, hyphenated, URN and braced forms. `Display` is lowercase
  hyphenated. — [docs.rs uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)
- Buildkite learned that downstream systems validate UUID versions, and changing the version broke
  customers. — [Buildkite](https://buildkite.engineering/goodbye-integers-hello-uuids/)
- ULID's Crockford base32 is 26 chars, compared with 36 for a hyphenated UUID. —
  [ULID spec](https://github.com/ulid/spec)

### Inferences
- **Path and JSON ids:** canonical 36-char lowercase. Parse with `Uuid::try_parse`, then require
  `get_version() == Some(Version::SortRand)` (v7) and treat anything else as 404. A 404 is better
  than a 400 here, so the response does not reveal how ids are shaped. Avoid accepting braced or URN
  forms in URLs. The simplest strict rule is to accept exactly 36 characters in hyphenated form, in
  any case.
- **Room share link:** keep the current 22-char base62 128-bit random slug. It is shorter than 36
  characters, readable, and is a real capability. A compact encoding of a v7 id (22-char base62 or
  26-char base32) would hide the familiar UUID shape, but it still leaks the time and still has low
  entropy, so it only adds encoding complexity.

### Gaps
- No authoritative source was found that recommends rejecting non-v7 versions. That is a design
  choice; RFC §6.12 favors treating ids as opaque.

## Natural vs surrogate keys; exposing internal ids

### Takeaway
Use a surrogate v7 `id` as the primary key for accounts and keep `username` as a `UNIQUE` column.
Then renames and case normalization never touch foreign keys. Expose ids only where they are a
handle, never as a credential.

### Cited Findings
- RFC 9562 §6.13 advises against using name-derived identifiers as keys because "changes to the
  input name can invalidate the structure", and recommends time-based surrogate keys. —
  [RFC 9562 §6.13](https://www.rfc-editor.org/rfc/rfc9562.html)
- In a SQLite rowid table, a non-INTEGER PK such as `username TEXT` is a separate unique index that
  stores the key twice. Foreign keys copy the text into every child row. —
  [SQLite WITHOUT ROWID](https://www.sqlite.org/withoutrowid.html)
- Buildkite previously kept both an integer key for internal use and a UUID for external use, and
  merged them into a single v7 key. — [Buildkite](https://buildkite.engineering/goodbye-integers-hello-uuids/)

### Inferences
- Accounts: `id BLOB(16) PK` (v7), `username TEXT NOT NULL UNIQUE` (with `COLLATE NOCASE`, or a
  normalized lowercase column, depending on the product rule). Foreign keys reference `id`. Users
  can log in by username, while the API returns and references `id`. Renaming then becomes a
  single-row update.
- Exposure policy:
  - Join-request ids: v7 is fine, because they are already gated by the host key.
  - Account ids: expose them only to the account owner, or not at all, because they reveal the
    sign-up time.
  - Room: expose the share slug, not the v7 PK, unless the share slug is the PK by design.
- Bearer secrets (host and account keys) stay as they are: 128-bit CSPRNG values, stored as
  SHA-256, never UUIDs, in line with RFC §8.

### Gaps
- No 2024-2026 primary source dedicated to natural vs surrogate keys was retrieved. The
  recommendation rests on the RFC §6.13 guidance and general practice.
