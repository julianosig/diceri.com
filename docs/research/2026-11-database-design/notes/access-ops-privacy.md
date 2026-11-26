# Database access, operations, security and privacy for a hexagonal Rust backend (Nov 2026)

Scope: rules for an AGENTS.md covering repository/port design, sqlx 0.9, persistence testing,
performance, security, GDPR/LGPD privacy, and operations. Versions checked: sqlx 0.9.0 (released
2026-05-06) and 0.9.1 (docs.rs, 2026-11-18). The repo's workspace pins `sqlx = "0.9.1"` with
`runtime-tokio` + `sqlite-bundled`. Current adapter (`crates/infra/src/sqlite_accounts.rs`) uses runtime
`sqlx::query(...)`, WAL, an inline `SCHEMA` string executed at startup (no migrations folder), and
maps `is_unique_violation()` to a domain error. Several sources found were blogs or forums; they
are flagged as practitioner opinion where used.

## 1. Repository / port design in hexagonal Rust

### Takeaway
No canonical Rust standard exists. Practitioner consensus is: one aggregate-oriented trait per
aggregate in `application::ports`, using domain types only. Adapters map rows to domain types and
validate on read, and they translate driver errors (unique violation → domain error). Transactions
that span repositories are the hard part. The common options are a transaction closure, a
repository holding a `Transaction`, or a unit-of-work port implemented in infra.

### Cited Findings
- Common Rust pattern: one trait per repository, exposing a transaction through a closure. The
  drawback is that only that repository's methods are reachable inside the closure, which gets
  awkward as repositories multiply — [users.rust-lang.org thread (2025/26)](https://users.rust-lang.org/t/structuring-a-rust-backend-feature-based-architecture-that-scales/137452)
- One clean-architecture sample (Rocket/axum + sqlx) uses a trait per use case and per
  repository, which makes mocking easy (mockall). The author names transaction handling as the
  painful part — [DEV, edo1z](https://dev.to/edo1z/building-a-clean-architecture-with-rusts-rocket-and-sqlx-a-deep-dive-45kn)
- Passing an sqlx `Transaction` into repository structs works, but critics note that the service
  layer then knows adapter details. This is the leak to avoid — [rust-clean-architecture-with-db-transactions](https://github-redirect.dependabot.com/frederikhors/rust-clean-architecture-with-db-transactions)
- Cross-language guidance: keep transaction management in the adapter layer behind a unit-of-work
  port. A plain repository port is enough when an operation touches one table/aggregate — [DEV (Go)](https://dev.to/2nguyenlong000k/handling-transactions-without-breaking-service-adapter-decoupling-in-hexagonal-architecture-with-golang-5ce1); [DEV "stop passing sql.Tx"](https://dev.to/gabrielanhaia/stop-passing-sqltx-through-your-go-service-layer-lp7)
- A worked example of hexagonal Rust with axum/sqlx, with domain-typed repository traits and
  adapter-side error mapping — [Master hexagonal architecture in Rust (howtocodeit, mirrored)](https://l-lin.github.io/programming-languages/rust/master-hexagonal-architecture-in-Rust)
- sqlx 0.9.1 `DatabaseError` exposes `is_unique_violation()`, `is_foreign_key_violation()`,
  `is_check_violation()`, `kind() -> ErrorKind`, `code()` (SQLSTATE) and `constraint()`/`table()`.
  The last two are populated only by the Postgres driver — [docs.rs sqlx 0.9.1 DatabaseError](https://docs.rs/sqlx/latest/sqlx/error/trait.DatabaseError.html)

### Inferences
- Rule: port traits live in `application::ports`, take and return domain types and a domain/port
  error enum (thiserror), and never mention `sqlx`, `Row`, `Pool`, `Transaction` or SQL types.
- Rule: methods are intention-revealing and aggregate-scoped (`register(username, key_hash)`,
  `find_by_key_hash`, `delete_expired(before)`), not generic `insert/update/get_all`. Add a method
  only when a use case needs it (AGENTS §7.6).
- Rule: map errors with `is_unique_violation()`/`is_foreign_key_violation()` rather than matching
  SQLSTATE strings, because those helpers work on SQLite and Postgres. Don't depend on
  `constraint()` (Postgres-only) unless the Postgres adapter needs to tell two unique keys apart.
- Rule: parse rows into domain types through the domain constructors (e.g. `Username::parse`), so
  corrupted or legacy rows surface as a typed `Corrupt`/`Storage` error instead of panicking. Never
  `unwrap()`.
- Rule: if a use case must change two aggregates atomically, first ask whether they are really one
  aggregate. Otherwise add a unit-of-work port whose sqlx implementation owns the `Transaction`.
  Don't thread `Transaction` through application code.

### Gaps
- I found no authoritative (official sqlx or Rust project) guidance on unit of work. Rust sources
  are blogs and forum posts.
- I couldn't verify the `ErrorKind` variant list for 0.9.1; the docs page only links to the enum.

## 2. sqlx 0.9 specifics

### Takeaway
sqlx 0.9 adds `sqlx.toml`. It makes the `query*()` functions accept only `&'static str` (or an
explicit `AssertSqlSafe(..)` wrapper), which is a structural guard against string-built SQL.
Compile-time macros need either a live `DATABASE_URL` or committed `.sqlx` offline data, checked in
CI with `cargo sqlx prepare --check`. SQLite connect options set safe defaults: foreign keys ON,
5 s busy timeout, synchronous FULL, and a 100-statement prepared-statement cache. WAL is not
enabled by default.

### Cited Findings
- 0.9.0 replaces the `&str` parameter of the `query*()` family with `SqlSafeStr`. By default only
  `&'static str` is accepted, and other strings must be wrapped in `AssertSqlSafe(..)`. Direct
  `Executor` calls should become `sqlx::raw_sql(...)` — [SQLx 0.9.0 release discussion](https://github.com/launchbadge/sqlx/discussions/4271)
- 0.9.0 adds a `sqlx.toml` config file for multi-database setups. It can rename the `DATABASE_URL`
  variable and set global or per-column type overrides — [release discussion](https://github.com/launchbadge/sqlx/discussions/4271); [docs.rs `query!`](https://docs.rs/sqlx/0.9.0/sqlx/macro.query.html)
- SQLite changes in 0.9: `libsqlite3-sys` is specified as a version range, and there are new opt-in
  features `sqlite-deserialize`, `sqlite-load-extension` and `sqlite-unlock-notify`. The repo is
  moving to the `transact-rs` org. `Cargo.lock` is no longer tracked, so `cargo install --locked
  sqlx-cli` no longer works — [release discussion](https://github.com/launchbadge/sqlx/discussions/4271)
- Offline mode: run `cargo sqlx prepare` against a live DB, then "Check the generated `.sqlx`
  directory into version control". Compile without `DATABASE_URL` ("you must omit `DATABASE_URL` or
  else it will still try to connect"). In CI, run `cargo sqlx prepare --check` — [docs.rs sqlx 0.9.0
  `query!`](https://docs.rs/sqlx/0.9.0/sqlx/macro.query.html). The 0.9 macro docs page did not mention
  `SQLX_OFFLINE`. A docs.rs crate page says offline mode is "now always enabled", but I couldn't
  confirm that this is 0.9-specific — [docs.rs crate page](https://docs.rs/crate/sqlx/latest)
- SQLite nullability inference in macros reads `NOT NULL` constraints and `EXPLAIN` output and is
  "fragile". A `LEFT JOIN` can produce `NULL` for a `NOT NULL` column, and mistakes surface as
  runtime `UnexpectedNull`. Overrides: `"col!"`, `"col?"`, `"col: T"` — [docs.rs `query!`](https://docs.rs/sqlx/0.9.0/sqlx/macro.query.html)
- `SqliteConnectOptions` defaults (0.9.0): "SQLx does not set a journal mode by default". Foreign
  keys are enabled. "The default busy timeout is 5 seconds". "The default synchronous settings is
  FULL". "The default cache capacity is 100 statements". `create_if_missing` and
  `optimize_on_close` are off, and so are `in_memory`/`shared_cache` — [docs.rs SqliteConnectOptions](https://docs.rs/sqlx/0.9.0/sqlx/sqlite/struct.SqliteConnectOptions.html)
- `PoolOptions::test_before_acquire` "Defaults to `true`". Other defaults are documented only as
  the "default sane configuration" in the source of `PoolOptions::new()` — [docs.rs PoolOptions 0.9.1](https://docs.rs/sqlx/latest/sqlx/pool/struct.PoolOptions.html)
- WAL: readers don't block writers, but there is exactly one writer at a time. WAL does not work
  on network filesystems. The `-wal` file is part of the persistent state. With
  `synchronous=NORMAL`, transactions are not durable across power loss (checkpoints are the only
  syncs). Versions 3.7.0–3.51.2 have a low-probability WAL-reset corruption bug when two
  connections write or checkpoint at the same instant. It is fixed in 3.51.3+ (backports 3.44.6,
  3.50.7) — [sqlite.org/wal.html](https://www.sqlite.org/wal.html)

### Inferences
- Rule: prefer `query!`/`query_as!` (compile-time checked) for static SQL once a `migrations/`
  folder exists. Commit `.sqlx/`, and add `cargo sqlx prepare --check` to `just check`/CI. Until
  then, the runtime `sqlx::query` with `&'static str` + `.bind()` is acceptable because 0.9's
  `SqlSafeStr` already blocks dynamic strings.
- Rule: `AssertSqlSafe` is banned outside infra, and inside infra each use needs a comment that
  justifies it (e.g. an allow-listed identifier). Treat it as a review red flag, like `unsafe`.
- Rule: set pragmas explicitly in `SqliteConnectOptions` (journal_mode WAL, foreign_keys ON,
  busy_timeout, synchronous NORMAL-or-FULL as a conscious choice) rather than relying on defaults.
  Keep the DB on local disk.
- Rule: because SQLite has one writer, use a write pool of 1 connection, or keep the pool small
  and write transactions short. Use `BEGIN IMMEDIATE` for read-then-write transactions to avoid
  `SQLITE_BUSY` upgrades (standard SQLite practice; not confirmed in sources here).
- Rule: check that the bundled SQLite (via `libsqlite3-sys`) is 3.51.3 or later, or a patched
  backport, because of the WAL-reset bug.
- `FromRow` derive: use it only on private infra row structs (`AccountRow`), then convert to domain
  via `TryFrom`. Never derive `FromRow` on domain types, because that would make `domain` depend
  on sqlx.

### Gaps
- I couldn't read the full 0.9 CHANGELOG (pool, migrate, `#[sqlx::test]` changes) or the exact
  `PoolOptions::new()` defaults.
- I didn't confirm whether `SQLX_OFFLINE=true` is still honored in 0.9.

## 3. Testing strategy for persistence

### Takeaway
Use three layers. Fast in-memory fakes cover use-case tests. One shared contract-test suite runs
against both the fake and each real adapter. Real-adapter tests run on SQLite (per-test DB via
`#[sqlx::test]` or `sqlite::memory:`) and, if Postgres arrives, on a real Postgres
(testcontainers or `#[sqlx::test]` with a superuser URL). Migrations get tested because every
test DB is built by running them.

### Cited Findings
- `#[sqlx::test]` creates a fresh database per test. For SQLite it needs no `DATABASE_URL` and
  uses files under `target/sqlx/test-dbs/<module path>.sqlite`. Postgres/MySQL need a superuser
  `DATABASE_URL`. It auto-applies `./migrations` (or `migrations = "path"`, `migrator =
  "crate::PATH"`, or `migrations = false`). It supports SQL fixtures applied in order, deletes the
  DBs of passing tests, and keeps failing ones for debugging. It requires the `migrate` feature —
  [docs.rs `#[sqlx::test]` 0.9.0](https://docs.rs/sqlx/0.9.0/sqlx/attr.test.html)
- Trait-per-repository designs make mocking easy, for example with mockall — [DEV, edo1z](https://dev.to/edo1z/building-a-clean-architecture-with-rusts-rocket-and-sqlx-a-deep-dive-45kn)

### Inferences
- Rule: write a generic `fn contract_<port>(repo: impl Port)` suite (or a macro) in a test-support
  module, and run it for both `Memory*` and `Sqlite*` adapters. This stops fakes drifting from real
  behavior (uniqueness, ordering, not-found semantics). The repo already has `memory_accounts.rs`
  and `sqlite_accounts.rs`, so the pair exists.
- Rule: move the inline `SCHEMA` to versioned `migrations/` (`sqlx::migrate!`) before the second
  table. Test that migrations apply from empty and that the adapter works on the migrated schema.
  Never edit an applied migration.
- The workspace currently doesn't enable the `migrate`/`macros` features, so `#[sqlx::test]` would
  require enabling them.
- For Postgres later: use testcontainers-rs or `#[sqlx::test]` against a CI Postgres service, and
  keep these tests out of `just test` (no network) and in `just ci`/e2e, consistent with AGENTS §5.

### Gaps
- I didn't fetch testcontainers-rs docs. Its current version and API are unverified here.

## 4. Performance

### Takeaway
Index every query predicate and sort, verify with `EXPLAIN QUERY PLAN` (SQLite) or `EXPLAIN
ANALYZE` (Postgres), and avoid N+1 by fetching per aggregate or batching with `IN`/joins. Use
keyset (seek) pagination, not OFFSET. Keep pools small.

### Cited Findings
- OFFSET makes the DB read and discard the skipped rows, so cost grows with the page number, and
  inserts between fetches cause duplicates. Keyset pagination uses `WHERE key < :last ORDER BY key
  DESC FETCH FIRST n`, needs a deterministic (unique) order, and can't jump to arbitrary pages —
  [use-the-index-luke.com/no-offset](https://use-the-index-luke.com/no-offset)
- Pool sizing starting point: `connections = (core_count * 2) + effective_spindle_count`. "You want
  a small pool, saturated with threads waiting for connections." — [HikariCP wiki, About Pool Sizing](https://github.com/brettwooldridge/HikariCP/wiki/About-Pool-Sizing) (older, undated wiki, attributes formula to PostgreSQL project)
- SQLite allows a single writer at a time even in WAL — [sqlite.org/wal.html](https://www.sqlite.org/wal.html)
- The sqlx SQLite statement cache defaults to 100 prepared statements per connection — [docs.rs SqliteConnectOptions](https://docs.rs/sqlx/0.9.0/sqlx/sqlite/struct.SqliteConnectOptions.html)

### Inferences
- Rule: every new query in a PR comes with its index (or a note that the PK/unique index covers
  it). For hot queries, record the `EXPLAIN QUERY PLAN` output in the PR, and look for `SEARCH ...
  USING INDEX`, not `SCAN`.
- Rule: no queries inside loops over rows. Add a batch port method instead.
- Rule: list endpoints use keyset pagination with an opaque cursor `(created_at, id)`.
- Lookup by `key_hash` (the current auth path) must hit a UNIQUE index, which the schema's unique
  constraint provides.

### Gaps
- I didn't fetch SQLite `EXPLAIN QUERY PLAN` or Postgres `pg_stat_statements` docs directly.

## 5. Security

### Takeaway
Bind every value and allow-list every identifier. Run the app with a least-privilege DB role
(Postgres) or locked-down file permissions (SQLite). Hash human passwords with Argon2id, but
high-entropy random tokens can use a fast hash (SHA-256, ideally HMAC-SHA-256 with a server-side
pepper). Never log bound parameters that contain secrets or PII.

### Cited Findings
- Primary defenses: parameterized queries, allow-listing structural parts (table/column names,
  ASC/DESC can't be parameters), and no manual escaping. Hardening: least privilege ("DO NOT ASSIGN
  DBA OR ADMIN TYPE ACCESS TO YOUR APPLICATION ACCOUNTS"), a separate DB user per application,
  views to restrict columns, and not running the DBMS as root — [OWASP SQL Injection Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/SQL_Injection_Prevention_Cheat_Sheet.html)
- Passwords: Argon2id with a minimum of m=19 MiB, t=2, p=1 (alternatives m=47104/t=1, m=12288/t=3,
  m=9216/t=4, m=7168/t=5). Use scrypt N=2^17 r=8 p=1 if Argon2id is unavailable, bcrypt cost ≥10
  (72-byte limit) only for legacy systems, and PBKDF2-HMAC-SHA256 with 600,000 iterations for FIPS.
  Fast hashes like SHA-256 are unsuitable for passwords. A pepper is stored outside the DB (vault
  or HSM) and is defense in depth only. The cheat sheet does not address high-entropy API tokens —
  [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- Practitioner consensus: random API keys/tokens aren't vulnerable to dictionary attacks, so
  SHA-256 is sufficient, while bcrypt/Argon2 only add latency (one report: about 300 ms bcrypt-12
  vs about 1 µs SHA-256). Many platforms use HMAC-SHA-256 with a server secret, so a DB dump alone
  can't verify candidates — [ADR 0008 api-key hashing (WFP)](https://docs.sso.earthobservation.vam.wfp.org/architecture/decisions/adrs/0008-api-key-hashing); [DEV: migrating API key auth from bcrypt to HMAC-SHA256](https://dev.to/vessel_api/the-946-millisecond-tax-migrating-api-key-auth-from-bcrypt-to-hmac-sha256-4c74) (practitioner sources; no OWASP/NIST statement found)
- sqlx 0.9 accepts only `&'static str` SQL by default (`AssertSqlSafe` opt-out) — [release discussion](https://github.com/launchbadge/sqlx/discussions/4271)

### Inferences
- The current design (a username-only account and a secret key stored as SHA-256) is sound **only
  if the key is server-generated with ≥128 bits of CSPRNG entropy and is never user-chosen**. If
  users can ever pick or type a memorable secret, switch to Argon2id (OWASP params). Optional
  hardening: HMAC-SHA-256 with a `DICERI_*` pepper from the environment, which means a leaked DB
  file can't be used even for offline checks.
- Look up by hash and compare in the index. That is not a timing oracle for the secret because the
  attacker doesn't control the hash preimage relation. Still, use constant-time compare if a
  comparison is ever done in Rust (standard practice; unsourced here).
- Rule (SQLite): the DB file, `-wal` and `-shm` live in a dedicated directory with mode 0700 owned
  by the service user, and the file is mode 0600. Never place it under a web-served path. Backups
  inherit the same permissions or are encrypted. (Permissions guidance is general Unix practice; no
  SQLite doc fetched.)
- Rule (Postgres): separate roles for migration/owner (DDL) and the app (DML only on the needed
  tables, no SUPERUSER/CREATEDB). The connection string comes from `DICERI_DATABASE_URL` and is
  never logged. Require TLS to the DB.
- Encryption at rest: for SQLite, rely on full-disk/volume encryption. SQLCipher-style page
  encryption is an extra native dependency and needs license review under AGENTS §9a. (Not
  researched in depth.)
- Rule: no `tracing` field may contain a bound parameter value, username, key or hash. Log query
  names/ids and durations only. Don't enable sqlx statement logging with parameters in production.

### Gaps
- I didn't verify sqlx 0.9's default statement-logging behavior (`log_statements`,
  `log_slow_statements` on `ConnectOptions`) or whether it ever logs bind values.
- There is no authoritative standard statement on fast-hashing high-entropy tokens; this rests on
  practitioner sources plus OWASP's entropy reasoning.

## 6. Privacy and compliance (GDPR, LGPD)

### Takeaway
Usernames and IP addresses are personal data under both regimes (IPs as online identifiers).
Hashed keys linked to a username are pseudonymous, not anonymous. Both laws require data
minimization, storage limitation with deletion when the purpose ends, honoring erasure requests,
security measures and records of processing. Voice audio and transcripts must not be persisted.
Voice used for cloning is likely biometric/sensitive data.

### Cited Findings
- GDPR Art. 5(1)(c) minimization: "adequate, relevant and limited to what is necessary". 5(1)(e)
  storage limitation: "kept in a form which permits identification ... for no longer than is
  necessary". 5(1)(f) security. 5(2) accountability: the controller must "be able to demonstrate
  compliance" — [GDPR Art. 5](https://gdpr-info.eu/art-5-gdpr/)
- GDPR Recital 30: "Natural persons may be associated with online identifiers provided by their
  devices, applications, tools and protocols", such as IP addresses and cookie IDs, which can be
  used to profile and identify — [Recital 30](https://gdpr-info.eu/recitals/no-30/)
- CJEU C-582/14 *Breyer* (19 Oct 2016): a dynamic IP address is personal data for a website
  operator that has legal means to identify the visitor with third-party (ISP) help. It is not
  personal data where identification is prohibited by law or practically impossible — [CJEU press release](https://curia.europa.eu/jcms/jcms/p1_236867/cs); [Inside Privacy summary](https://www.insideprivacy.com/international/cjeu-confirms-dynamic-ip-addresses-to-be-personal-data/) (2016, pre-GDPR, still the leading case)
- LGPD (Lei 13.709/2018): Art. 5 I defines personal data as "informação relacionada a pessoa
  natural identificada ou identificável". Art. 5 II lists sensitive data, including biometric
  data. Art. 6 III (necessidade) means "limitação do tratamento ao mínimo necessário". Art. 6 VII
  covers security. Under Art. 12, anonymized data is not personal data unless reversible. Art. 15
  ends processing when the purpose is achieved. Art. 16: "Os dados pessoais serão eliminados após o
  término de seu tratamento". Art. 18 VI gives the right to deletion of consent-based data. Art. 37
  requires a record of processing operations. Art. 46 requires security measures. Art. 48 requires
  incident notification to the ANPD and data subjects — [Planalto, Lei 13.709](https://www.planalto.gov.br/ccivil_03/_ato2015-2018/2018/lei/l13709.htm)
- Under ANPD Resolução CD/ANPD nº 15/2024, incident communication to the ANPD and to data subjects
  is due within 3 business days of learning that personal data were affected, with a complement
  within 20 business days. A secondary source says the incident record is kept at least 5 years
  and that small agents get doubled deadlines — [ANPD gov.br](https://www.gov.br/anpd/pt-br/assuntos/noticias/aberta-consulta-publica-sobre-norma-de-comunicacao-de-incidente-de-seguranca-com-dados-pessoais/aircomunicacaodeincidentes.pdf); [Resolução 15/2024 (DOU)](https://sintse.tse.jus.br/documentos/2024/Abr/26/diario-oficial-da-uniao-secao-1/autoridade-nacional-de-protecao-de-dados-resolucao-no-15-de-24-de-abril-de-2024-aprova-o-regulamento); [TI Inside (secondary)](https://tiinside.com.br/12/08/2025/empresas-terao-ate-3-dias-para-comunicar-falhas-de-seguranca-com-dados-pessoais/)
- `VACUUM` removes "all traces of deleted content" and is an alternative to `PRAGMA
  secure_delete=ON`. `VACUUM INTO` backups purge deleted content and leave "no forensic traces" —
  [sqlite.org VACUUM](https://www.sqlite.org/lang_vacuum.html)

### Inferences
- Treat as personal data: username, account id, key hash (a pseudonymous identifier), room
  membership/timestamps tied to an account, IP addresses, and user-agent strings. Treat as
  sensitive/biometric (LGPD Art. 5 II; under GDPR, Art. 9 biometric when used to identify, which
  wasn't fetched here): voice samples and voice-clone embeddings. Clones are already
  delete-at-end-of-call per ADR 0006.
- Rule: no table or column may hold audio, transcripts, translations, or voice embeddings. Add a
  schema review check in PRs, and document every column's purpose and retention in schema docs.
- Rule: every table holding personal data has a documented retention period and a deletion path
  (a TTL column + a scheduled `delete_expired` job, e.g. rooms closed > N days). Erasure of an
  account deletes or anonymizes all rows referencing it (`ON DELETE CASCADE` or an explicit use
  case), and a contract test proves no row survives.
- Rule: deleted data lingers in SQLite free pages, the WAL, and backups. Enable `PRAGMA
  secure_delete` or run periodic `VACUUM`, and give backups a bounded retention so erased data ages
  out (document the window in the privacy notice).
- Rule: don't store raw IPs in the DB. If abuse controls need them, keep them in memory/rate
  limiter only, or store a truncated or keyed hash with a short TTL. Audit logs record action, id
  and timestamp, never content or secrets, and have their own retention.
- Records of processing (LGPD Art. 37, GDPR Art. 30): maintain a short `docs/` data inventory
  (table → fields → purpose → legal basis → retention).

### Gaps
- I didn't fetch GDPR Arts. 9, 17, 25, 30, 32 and 33 text directly. Art. 33's 72-hour breach
  notification is well known but wasn't verified in this session.
- I didn't verify the primary text of Resolução 15/2024 for the 5-year record and small-agent
  doubling.
- No authoritative source checked on whether a SHA-256 hash of a random key counts as pseudonymous
  data. The inference rests on GDPR's identifiability test.

## 7. Operations

### Takeaway
SQLite: take consistent online backups (`VACUUM INTO` or the backup API, never a raw `cp` of a
live DB without its `-wal`), keep them off-host and encrypted, and run scheduled restore drills.
Postgres later: base backups + WAL archiving for point-in-time recovery, plus slow-query
monitoring. Keep schema documentation and a data inventory in the repo.

### Cited Findings
- `VACUUM INTO 'file'` produces "a consistent snapshot of the original database", is "an
  alternative to the backup API", and yields a minimal file. It isn't blocked by another
  connection's write lock. The target must not exist. An interrupted run may produce a corrupt
  output. The backup API uses less CPU and can be incremental — [sqlite.org VACUUM](https://www.sqlite.org/lang_vacuum.html)
- The `-wal` file is part of the persistent state. Separating the DB from its WAL can lose
  committed transactions or corrupt the DB. A long-running reader can block checkpoints, causing
  unbounded WAL growth (fix with manual `RESTART`/`TRUNCATE` checkpoints). The default
  auto-checkpoint threshold is 1000 pages (about 4 MB) — [sqlite.org/wal.html](https://www.sqlite.org/wal.html)

### Inferences
- Rule: run a backup job with `VACUUM INTO` to a temp name, then `PRAGMA integrity_check` on the
  copy. Atomically rename it, then encrypt and ship it off-host. Set the retention to match the
  privacy erasure window.
- Rule: hold a quarterly restore drill (restore the latest backup into a scratch environment, run
  migrations and smoke tests) and record the RPO/RTO observed.
- For continuous PITR on SQLite, Litestream-style WAL shipping exists (not researched here). On
  Postgres, use `pg_basebackup` + WAL archiving or a managed provider's PITR, plus
  `pg_stat_statements` and `log_min_duration_statement` for slow queries (standard Postgres
  practice; docs not fetched).
- Rule: emit `tracing` spans per repository call with operation name and duration only, and alert
  on p95 latency and `SQLITE_BUSY`/pool-acquire timeouts.

### Gaps
- I didn't research Litestream's current status or license (AGPL compatibility) or Postgres
  PITR/`pg_stat_statements` docs in this session.
- I didn't verify sqlx 0.9 slow-statement logging configuration.
