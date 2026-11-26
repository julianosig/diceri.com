# Relational schema design, integrity and migrations (SQLite now, PostgreSQL later), November 2026

Scope: naming, in-database integrity, time, migrations with sqlx 0.9, SQLite in production,
soft delete, normalization and JSON. Ids (UUIDv7) are covered elsewhere.

Local facts checked in this repository on 2026-11-25:

- `Cargo.toml` pins `sqlx = { version = "0.9.1", default-features = false, features =
  ["runtime-tokio", "sqlite-bundled"] }`. So `migrate`, `macros` (`query!`) and `json` are
  **off**.
- `Cargo.lock` resolves `libsqlite3-sys 0.40.0`, whose bundled `sqlite3.h` declares
  `SQLITE_VERSION "3.54.1"`.
- `crates/infra/src/sqlite_accounts.rs` runs `CREATE TABLE IF NOT EXISTS accounts (username TEXT
  PRIMARY KEY NOT NULL, key_hash BLOB NOT NULL UNIQUE, created_at INTEGER NOT NULL DEFAULT
  (unixepoch())) STRICT` at startup.
- The same file uses WAL, `busy_timeout` 5 s and a pool of 4 connections (1 connection for
  `:memory:`).

## Naming conventions (tables, columns, constraints, indexes)

### Takeaway
Use lowercase snake_case everywhere. Choose one rule for table names (plural is the common
choice in app backends and is acceptable to the style guides) and apply it without exception.
Name foreign keys `<entity>_id` and timestamps `<verb>_at`. Give every constraint and index an
explicit prefixed name, such as the GitLab scheme. Explicit names keep migrations deterministic
and portable to Postgres.

### Cited Findings
- Use lowercase names with underscores in Postgres. Unquoted identifiers fold to lowercase, so
  mixed case forces quoting everywhere — [PostgreSQL wiki, Don't Do This](https://wiki.postgresql.org/wiki/Don't_Do_This)
- Holywell's SQL style guide:
  - Use snake_case. Avoid `tbl`/`sp_` prefixes and Hungarian notation.
  - Avoid uncommon abbreviations. Keep names at 30 bytes or less.
  - Never end a name with an underscore or use consecutive underscores.
  - Never give a table and a column the same name.
  - Column names are singular.
  - Table names should prefer a collective noun (`staff`), with a plural as the "less ideal"
    option.
  - Link tables should get a descriptive name (`services`), not `cars_mechanics`.
  - Source: [SQL Style Guide (Simon Holywell)](https://www.sqlstyle.guide/)
- Holywell's uniform suffixes are `_id`, `_status`, `_total`, `_num`, `_name`, `_seq`, `_date`,
  `_tally`, `_size` and `_addr`. The guide also says to avoid naming a primary key just `id`
  "where possible" — [SQL Style Guide](https://www.sqlstyle.guide/). This conflicts with common
  ORM and app practice, where `id` is the primary key and `<table_singular>_id` is the foreign
  key. Treat it as an opinion, not a consensus.
- Holywell on keys and constraints:
  - Every table must have at least one key.
  - Give constraints custom names (UNIQUE, PK and FK may be left to the database).
  - Put `CHECK()` constraints in separate clauses for easier debugging.
  - Avoid EAV tables.
  - Avoid storing a value and its unit in separate columns; put the unit in the column name.
  - Source: [SQL Style Guide](https://www.sqlstyle.guide/)
- GitLab's constraint and index naming conventions:
  - CHECK constraints: `check_<table>_<column>[_and_<column>]*[_<suffix>]`, where the suffix is
    the kind of validation, e.g. `_length` or `_enum`.
  - Foreign keys: `fk_<table>_<column>_<foreign_table>`, e.g. `fk_projects_group_id_groups`.
  - Indexes: `index_<table>_on_<column>[_and_<column>]*`.
  - Prefixes are preferred over suffixes so the kind of object is visible at a glance.
  - The convention was created because, without an ORM picking names (their Go services),
    explicit names are needed.
  - Source: [GitLab docs, Constraints naming conventions](https://docs.gitlab.com/development/database/constraint_naming_convention/)
- GitLab's Container Registry uses a different prefix set (`pk_`, `fk_`, `uq_`, `ck_`). Even
  within GitLab, the exact prefixes vary; what matters is applying one scheme consistently — [GitLab container-registry MR 177](https://gitlab.com/gitlab-org/container-registry/-/merge_requests/177)
- PostgreSQL truncates identifiers at 63 characters. GitLab allows dropping the `_and_` in
  check-constraint names to stay under that limit — [GitLab docs](https://docs.gitlab.com/development/database/constraint_naming_convention/)

### Inferences
- Suggested AGENTS.md rules:
  - Tables are plural snake_case (`accounts`, `rooms`, `room_participants`). The existing
    `accounts` table already follows this.
  - Primary key: `id`, or the natural key when one exists (as `accounts.username` does today).
  - Foreign keys: `<referenced_singular>_id`.
  - Timestamps: `<past_participle>_at` (`created_at`, `expires_at`, `deleted_at`).
  - Booleans: `is_`/`has_`/`can_` (e.g. `is_active`), stored as INTEGER 0/1 with
    `CHECK (is_x IN (0, 1))` in SQLite and as `boolean` in Postgres.
  - Durations and units carry the unit in the name (`duration_ms`, `size_bytes`).
  - No reserved words (`user`, `order`, `group`), so nothing ever needs quoting.
- I found no 2024–2026 authoritative source that settles plural versus singular table names.
  Both are used widely. Consistency is the only rule backed by sources.
- Prefixes adapted from GitLab: `pk_<table>`, `fk_<table>_<column>_<ref_table>`,
  `uq_<table>_<cols>`, `check_<table>_<column>_<kind>`, `index_<table>_on_<cols>`. Naming CHECK
  and UNIQUE constraints explicitly matters for portability: Postgres error payloads report the
  constraint name, and sqlx exposes it through `DatabaseError::constraint()`. The application
  can then map a named constraint violation to a typed domain error (for example "username
  taken") instead of parsing messages.
- SQLite caveat: SQLite does not report constraint names in UNIQUE or NOT NULL errors the way
  Postgres does. It reports `table.column`. Error mapping should therefore go through one
  adapter function per backend. (Inferred from experience; I did not find a specific SQLite
  document for this in this session.)

### Gaps
- I did not fetch Supabase's or Heroku's style guides; GitLab and Holywell were used.
- I found no source quantifying the use of plural versus singular table names.

## Integrity in the database (NOT NULL, CHECK, UNIQUE, FKs, enums, lengths)

### Takeaway
Treat the database as the last line of defense:

- NOT NULL is the default for every column.
- Every invariant that can be expressed in a single row gets a CHECK constraint.
- Every uniqueness rule gets a UNIQUE constraint.
- Every reference gets a foreign key with an explicit ON DELETE and an index on the child column.
- Enums use `TEXT` plus CHECK, which is portable and cheap to change in both engines.
- Use `text` with CHECK length bounds instead of `varchar(n)`.

### Cited Findings
- In SQLite, foreign keys are "disabled by default (for backwards compatibility), so must be
  enabled separately for each database connection." If no action is specified, the default is
  NO ACTION. The available actions are NO ACTION, RESTRICT, SET NULL, SET DEFAULT and CASCADE.
  RESTRICT is checked immediately, even when the constraint is deferred — [SQLite, Foreign Key Support](https://www.sqlite.org/foreignkeys.html)
- SQLite: "In most real systems, an index should be created on the child key columns of each
  foreign key constraint." Without the index, a delete on the parent scans the whole child
  table. The parent key must be a PRIMARY KEY or UNIQUE — [SQLite, Foreign Key Support](https://www.sqlite.org/foreignkeys.html)
- SQLite: "It is not possible to enable or disable foreign key constraints in the middle of a
  multi-statement transaction ... it simply has no effect." Deferred constraints
  (`DEFERRABLE INITIALLY DEFERRED`) are checked at COMMIT — [SQLite, Foreign Key Support](https://www.sqlite.org/foreignkeys.html)
- sqlx 0.9.1 `SqliteConnectOptions::new()` sets `foreign_keys = ON` by default ("We choose to
  enable foreign key enforcement by default, though SQLite normally leaves it off"). It leaves
  `journal_mode` and `synchronous` unset unless they are requested. The default `busy_timeout`
  is 5 s — source read locally in `sqlx-sqlite-0.9.1/src/options/mod.rs`, also at [launchbadge/sqlx](https://github.com/launchbadge/sqlx)
- In STRICT tables, CHECK, NOT NULL, FOREIGN KEY, UNIQUE, DEFAULT, COLLATE, generated columns
  and indexes "work the same" as in ordinary tables. In addition, values that cannot be
  losslessly converted to the declared type raise `SQLITE_CONSTRAINT_DATATYPE` — [SQLite, STRICT Tables](https://www.sqlite.org/stricttables.html)
- Postgres:
  - Don't use `varchar(n)` by default; use `text` (or unbounded `varchar`), because arbitrary
    limits cause production errors and a CHECK constraint can enforce real requirements.
  - Don't use `char(n)`, even for fixed-length codes; use `text` plus a CHECK on length or
    format.
  - Don't use `money`; use `numeric` or integer minor units.
  - Don't use `serial`; use identity columns.
  - Use `NOT EXISTS` instead of `NOT IN`, because of NULL semantics.
  - Source: [PostgreSQL wiki, Don't Do This](https://wiki.postgresql.org/wiki/Don't_Do_This)
- Holywell: use `CHECK()` ranges for numeric values whose range is known, and at least `> 0`
  where applicable. Enforce known string formats with `LIKE`/`SIMILAR TO` — [SQL Style Guide](https://www.sqlstyle.guide/)
- Postgres `ALTER TABLE ... ADD CONSTRAINT ... NOT VALID` followed by `VALIDATE CONSTRAINT`:
  validation takes only a `SHARE UPDATE EXCLUSIVE` lock (plus `ROW SHARE` on the referenced
  table for an FK). `ADD FOREIGN KEY` takes `SHARE ROW EXCLUSIVE` on both tables. This is
  documented for PostgreSQL 19 ("current" docs) — [PostgreSQL docs, ALTER TABLE](https://www.postgresql.org/docs/current/sql-altertable.html)

### Inferences
- Rules for AGENTS.md:
  1. Every column is `NOT NULL` unless absence has a domain meaning, and that meaning is written
     in a comment.
  2. Every table is `STRICT` in SQLite.
  3. Text columns get `CHECK (length(x) BETWEEN 1 AND N)` that mirrors the domain newtype
     (e.g. `Username`). The domain type stays the primary validator and the database is the
     backstop.
  4. Enums: `TEXT NOT NULL CHECK (status IN ('pending','active','ended'))`. Native Postgres
     `ENUM` types are not portable to SQLite and are awkward to change (values can be added but
     not removed without a type swap; general knowledge, not verified in this session). Use a
     lookup table only when the set is data that users or admins can change, or when values
     carry attributes.
  5. Foreign keys always state `ON DELETE` explicitly:
     - `CASCADE` for owned children (a room's participants);
     - `RESTRICT` for references that must block deletion;
     - `SET NULL` only for nullable "weak" links.
  6. Every FK column gets an index, unless it is the leading column of the PK or of another
     index.
  7. Uniqueness that depends on business rules ("one active session per account") uses a
     partial unique index. SQLite and Postgres both support `CREATE UNIQUE INDEX ... WHERE`
     (general knowledge; not fetched).
  8. Store money and durations as integers in minor units (`_cents`, `_ms`).
- Because sqlx turns `foreign_keys` on, any migration that rebuilds a table (the SQLite 12-step
  procedure below) cannot switch FKs off inside sqlx's per-migration transaction. Such a
  migration needs `-- no-transaction` and explicit `BEGIN`/`COMMIT`, or it must be written so
  FKs stay valid throughout.

### Gaps
- I did not fetch a 2024–2026 primary source comparing Postgres native enums with CHECK
  constraints. The enum recommendation rests on portability reasoning.

## Time: storage, created_at/updated_at, who sets them

### Takeaway
Store instants only, always in UTC:

- **SQLite:** `INTEGER` Unix epoch. Milliseconds is the better default if ordering or
  sub-second precision matters. The current `unixepoch()` default gives seconds.
- **Postgres:** `timestamptz`, never `timestamp`.

The application (via a clock port) sets times that have business meaning. The database DEFAULT
is only a backstop for `created_at`. `updated_at` is set explicitly in each UPDATE, not by
triggers.

### Cited Findings
- STRICT tables allow only `INT`, `INTEGER`, `REAL`, `TEXT`, `BLOB` and `ANY`. There is no
  DATETIME type, so a timestamp has to be INTEGER, REAL or TEXT — [SQLite, STRICT Tables](https://www.sqlite.org/stricttables.html)
- Postgres:
  - Use `timestamptz`, not `timestamp`, even for UTC values; plain `timestamp` "is just a
    date and clock reading with no zone".
  - Avoid `timestamptz(0)`, because it rounds and `now()` can then store a time in the future.
  - Avoid `timetz` and `CURRENT_TIME`.
  - Source: [PostgreSQL wiki, Don't Do This](https://wiki.postgresql.org/wiki/Don't_Do_This)
- sqlx 0.9.1 documented the default `TimeZone=UTC` session behavior for Postgres (#4344). It
  also fixed panics when decoding out-of-range dates (#4528, Postgres; #4433, SQLite INTEGER to
  `PrimitiveDateTime` now errors) — [sqlx CHANGELOG 0.9.1](https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md)
- SQLite `ALTER TABLE ADD COLUMN` cannot use a default of `CURRENT_TIMESTAMP` or any
  parenthesized expression, so `DEFAULT (unixepoch())` works only in `CREATE TABLE` — [SQLite, ALTER TABLE](https://www.sqlite.org/lang_altertable.html)
- Postgres: a column added with a volatile default (e.g. `clock_timestamp()`) rewrites the
  table. A non-volatile default (including `now()`, which is stable within a transaction) is
  stored in the catalog with no rewrite — [PostgreSQL docs, ALTER TABLE](https://www.postgresql.org/docs/current/sql-altertable.html)

### Inferences
- Rules for AGENTS.md:
  - Column suffix `_at` and type `INTEGER` (Unix epoch in **milliseconds**, UTC) in SQLite;
    `timestamptz` in Postgres.
  - Never store local time or offsets in instants. Store a user's IANA time zone separately as
    TEXT if it is ever needed.
  - Dates without time (birthdays) use `_on` and a `TEXT` `YYYY-MM-DD` with a CHECK.
  - The application passes timestamps from an injectable clock, so tests are deterministic.
  - `DEFAULT` on `created_at` is a safety net only.
- Seconds versus milliseconds: UUIDv7 already carries a ms timestamp, and ms epoch maps losslessly
  to `timestamptz` (µs precision). That favors ms. The existing `accounts.created_at` uses
  seconds via `unixepoch()`. Changing it is a migration and should be decided once.
  `unixepoch('subsec')` and `(unixepoch('subsec')*1000)` exist in SQLite 3.42+ (general
  knowledge; not fetched this session).
- Why INTEGER over ISO-8601 TEXT in SQLite: it is compact, compares numerically, is unambiguous
  about zone, and maps to `i64`/`OffsetDateTime` in sqlx. TEXT is more readable in the sqlite3
  CLI, but the format has to be identical on every write for comparisons to stay correct.
- `updated_at`: avoid triggers. They are engine-specific, which hurts portability; set
  `updated_at = ?` in every UPDATE statement instead. Add `updated_at` only to tables whose rows
  are actually updated. Append-only tables don't need it.

### Gaps
- I found no 2024–2026 consensus article specifically on INTEGER ms versus TEXT for SQLite
  timestamps. The recommendation is derived from STRICT typing and portability.

## Migrations with sqlx 0.9 (versioned, forward-only, expand/contract, testing)

### Takeaway
Replace the startup `CREATE TABLE IF NOT EXISTS` string with versioned `.sql` files run by
`sqlx::migrate!()`. This needs the `migrate` feature, which is currently disabled.

- Migrations are forward-only. Never edit an applied migration: sqlx checksums each file and
  refuses to run if one has changed.
- Keep one migration per logical change and keep data migrations separate.
- Use expand/contract for any change a running old binary would break.
- Test from an empty database and on a copy of production-like data.

### Cited Findings
- sqlx 0.9.0 was released 2026-05-06 and 0.9.1 on 2026-11-17 — [sqlx CHANGELOG](https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md)
- sqlx default features are `any`, `macros`, `migrate` and `json`. `macros` pulls in `offline`.
  `migrate` is its own feature. Read locally in `sqlx-0.9.1/Cargo.toml`, also at [launchbadge/sqlx](https://github.com/launchbadge/sqlx)
- The project sets `default-features = false`, so it must add `"migrate"` (and `"macros"` for
  `migrate!`/`query!`).
- sqlx 0.9.0 added `sqlx.toml` per-crate config, enabled with the `sqlx-toml` feature. It can:
  - rename or relocate `_sqlx_migrations`;
  - ignore characters (e.g. whitespace) when hashing migrations;
  - set the preferred date/time crate;
  - and more.
- Other sqlx 0.9.0 changes:
  - The `Migrate` trait changed significantly.
  - `Migrator::set_ignore_missing` and `set_locking` return `&mut Self`.
  - Added: `no_tx` migration support for SQLite (#4015), `Migrator::with_migrations()` (#4020),
    the ability to skip migrations (#3846) and deterministic migration order (#4136).
  - `offline` became optional, so builds work without serde (#4077).
  - `SQLX_OFFLINE_DIR` from the environment takes precedence in macros (#3962).
  - Source: [sqlx CHANGELOG 0.9.0](https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md)
- sqlx 0.9.1: `.sql` migration files that don't match the expected filename format are now an
  error (#4367) — [sqlx CHANGELOG 0.9.1](https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md)
- sqlx-core 0.9.1 migration errors (read locally in `sqlx-core-0.9.1/src/migrate/error.rs`):
  - `VersionMismatch`: "migration {0} was previously applied but has been modified". This is
    the checksum guard against editing applied migrations.
  - `VersionMissing`: "previously applied but is missing in the resolved migrations".
  - `Dirty`: a partially applied migration.
  - "cannot mix reversible migrations with simple migrations".
  - Reversible migrations use `.up.sql`/`.down.sql`. A file that starts with
    `-- no-transaction` runs outside a transaction (`source.rs`).
  - Source: [launchbadge/sqlx](https://github.com/launchbadge/sqlx)
- `migrate!()` embeds the migrations at compile time. Recompiling when a migration is added
  needs a `build.rs` with `println!("cargo:rerun-if-changed=migrations")`, which
  `sqlx migrate build-script` generates (doc comment on `migrate!` in sqlx 0.9.1,
  `src/macros/mod.rs`) — [launchbadge/sqlx](https://github.com/launchbadge/sqlx)
- Expand/contract:
  - Expand adds new schema elements alongside the old ones. Contract removes the old ones after
    every application version has moved over — [Xata glossary, expand-contract](https://xata.io/glossary/expand-contract)
  - Xata's pgroll applies expand/contract automatically for Postgres — [Xata blog, pgroll](https://xata.io/blog/pgroll-expand-contract)
  - Talk slides: [PGConf EU 2024 slides](https://www.postgresql.eu/events/pgconfeu2024/sessions/session/5871/slides/556/Postgres%20schema%20migrations%20using%20expand_contract%20-%20PGConf%20EU%202024.pdf)
  - Encore (Nov 2026) recommends dual-read, then dual-write with reconciliation, then a single
    writer before contracting — [Encore, zero-downtime migration](https://encore.dev/articles/zero-downtime-migration)
- Postgres zero-downtime building blocks:
  - `ADD COLUMN` with a non-volatile default needs no rewrite.
  - A type change usually rewrites the table and its indexes, can temporarily need double the
    disk space, and rewriting forms "are not MVCC-safe".
  - `SET NOT NULL` skips the full-table scan if a valid `CHECK (col IS NOT NULL)` constraint
    already exists.
  - Add FK and CHECK constraints `NOT VALID`, then `VALIDATE` them separately.
  - Source: [PostgreSQL docs, ALTER TABLE (v19)](https://www.postgresql.org/docs/current/sql-altertable.html)
- SQLite `ALTER TABLE` supports RENAME TO, RENAME COLUMN, ADD COLUMN and DROP COLUMN, plus
  `ALTER COLUMN ... SET/DROP NOT NULL` since 3.53.0 (2026-04-09).
  - ADD COLUMN restrictions: no PK or UNIQUE; no `CURRENT_*` or parenthesized default; NOT NULL
    needs a non-NULL default; no STORED generated column.
  - DROP COLUMN fails if the column is indexed, used in a PK, UNIQUE, FK or CHECK, or referenced
    by a trigger or view.
  - Anything else (changing a type, adding CHECK, FK or UNIQUE constraints) needs the 12-step
    rebuild:
    1. `PRAGMA foreign_keys=OFF`.
    2. BEGIN.
    3. Save the indexes, triggers and views.
    4. CREATE `new_X`.
    5. `INSERT ... SELECT` into `new_X`.
    6. DROP the old table.
    7. RENAME `new_X` to X.
    8. Recreate the indexes and triggers.
    9. Recreate the views.
    10. `PRAGMA foreign_key_check`.
    11. COMMIT.
    12. Turn FKs back on.
  - Renaming the old table first is explicitly called incorrect.
  - Source: [SQLite, ALTER TABLE](https://www.sqlite.org/lang_altertable.html)

### Inferences
- Rules for AGENTS.md:
  1. Migrations live in `crates/infra/migrations/` and are named
     `<YYYYMMDDHHMMSS>_<snake_description>.sql` (the `sqlx migrate add` default). They are
     embedded with `sqlx::migrate!("./migrations")` and run at startup before serving traffic.
  2. Migrations are **forward-only**: no `.down.sql`. A rollback is a new migration. Reasons:
     down migrations are rarely tested, data-destroying downs are dangerous, and sqlx forbids
     mixing reversible and simple migrations, so choose once.
  3. **Never edit or delete an applied migration.** sqlx detects this (`VersionMismatch` or
     `VersionMissing`) and refuses to start. Fix mistakes with a new migration. Before merge,
     editing your own unmerged migration is fine.
  4. One logical change per migration. Schema DDL and bulk data backfills go in separate
     migrations. Large backfills run in batches from application code or a job, not in a
     single migration transaction.
  5. Any change that would break the previous binary (rename, drop, type change, new NOT NULL
     without a default) uses expand/contract across at least two releases:
     - expand: add the new column or table, nullable or with a default;
     - deploy code that writes both and reads the new one;
     - backfill;
     - add the constraint (`NOT VALID` then `VALIDATE` on Postgres);
     - contract: drop the old column in a later release.
  6. Write portable DDL: `TEXT`, `INTEGER`, `BLOB`, CHECK, explicit constraint names, no
     triggers and no engine-specific functions in defaults where avoidable. Separate migration
     directories per engine will be needed when Postgres arrives, since `STRICT` and `BLOB`
     have no Postgres equivalent. Keep them parallel and numbered alike.
  7. Testing:
     - `#[sqlx::test]` (or a test helper) runs all migrations on a fresh in-memory or temp
       database for each test.
     - Every migration that transforms data gets a test that seeds the pre-migration shape and
       asserts the post-migration shape. `Migrator::with_migrations()` (new in 0.9) makes
       partial application feasible.
     - CI checks that migrations apply cleanly from empty, and runs `PRAGMA integrity_check`
       and `PRAGMA foreign_key_check` afterwards.
  8. For SQLite table rebuilds, use a `-- no-transaction` migration that manages
     `PRAGMA foreign_keys`, `BEGIN`/`COMMIT` and `foreign_key_check` itself. This is needed
     because the pragma has no effect inside sqlx's default per-migration transaction.
- Moving the existing `accounts` table: the first migration can be the same
  `CREATE TABLE IF NOT EXISTS` statement. Existing databases then adopt it idempotently, and
  sqlx records it as version 1.
- Offline mode applies only if the team adopts `query!` macros. That requires committing the
  `.sqlx/` directory (`cargo sqlx prepare`) and a CI check (`cargo sqlx prepare --check`) so
  builds don't need a live database. This is general knowledge of sqlx; 0.9 made `offline`
  separable from `macros` at the core level (#4077).

### Gaps
- I did not fetch the sqlx 0.9 `sqlx.toml` reference or the `#[sqlx::test]` docs page. Exact
  config keys are not quoted here.
- I found no first-party sqlx guidance on forward-only versus reversible migrations. The rule is
  derived from the error semantics and general practice.

## SQLite in production (STRICT, PRAGMAs, single writer, backups, when to move, portability)

### Takeaway
SQLite is a sound production choice for a single-host, low-write-concurrency service like this
one. Use this configuration:

- STRICT tables.
- `journal_mode=WAL`, set once; it is persistent.
- `foreign_keys=ON` on every connection (sqlx does this by default).
- `busy_timeout` of about 5 s.
- `synchronous=NORMAL` in WAL mode, accepting that the last commits may be lost on power loss
  but not that the database can be corrupted.
- Periodic `PRAGMA optimize`.
- A single-writer discipline (`BEGIN IMMEDIATE` for read-then-write).
- Continuous backups with Litestream 0.5.x, plus `VACUUM INTO` snapshots.

Move to Postgres when there are several app hosts, heavy concurrent writes or data approaching
TB scale.

### Cited Findings
- STRICT was added in SQLite 3.37.0 (2021-11-27):
  - Every column must declare `INT`, `INTEGER`, `REAL`, `TEXT`, `BLOB` or `ANY`.
  - Values that cannot be losslessly converted raise `SQLITE_CONSTRAINT_DATATYPE`.
  - `integrity_check` and `quick_check` verify types in STRICT tables.
  - `INTEGER PRIMARY KEY` is still a rowid alias; `INT PRIMARY KEY` is not.
  - Source: [SQLite, STRICT Tables](https://www.sqlite.org/stricttables.html)
- WAL mode:
  - It is persistent across connections and reopenings.
  - "Readers do not block writers, and writers do not block readers", but there can be only
    one writer at a time.
  - It auto-checkpoints at 1000 pages (about 4 MB).
  - Long-running readers can starve checkpoints and grow the WAL without bound.
  - With `synchronous=NORMAL`, commits don't fsync and only checkpoints do. Commits can roll
    back after power loss.
  - WAL "does not work over a network filesystem".
  - Source: [SQLite, Write-Ahead Logging](https://www.sqlite.org/wal.html)
- The WAL page also describes a "WAL-reset bug": a rare data race that can corrupt the database
  when two connections write or checkpoint at the same instant. It affects 3.7.0 through
  3.51.2 and is fixed in 3.51.3+, with backports to 3.44.6 and 3.50.7 — [SQLite, Write-Ahead Logging](https://www.sqlite.org/wal.html).
  The project's bundled SQLite is 3.54.1 (from `libsqlite3-sys 0.40.0`), so it has the fix.
- Recent SQLite releases: 3.54.0 (2026-10-09), 3.54.1 (2026-10-16), and 3.53.6/3.50.8/3.44.7
  (all 2026-11-13) — [SQLite release chronology](https://www.sqlite.org/chronology.html). The
  page's "last updated" date is inconsistent with these entries.
- Community baseline (blogs; not SQLite docs): `journal_mode=WAL`, `synchronous=NORMAL`,
  `foreign_keys=ON`, `busy_timeout=5000`, a larger `cache_size`, and `PRAGMA optimize` before
  long-lived connections close — [OneUptime, SQLite production setup (2026-02-02)](https://oneuptime.com/blog/post/2026-02-02-sqlite-production-setup/view);
  [DEV, SQLite as your server database](https://dev.to/software_mvp-factory/sqlite-as-your-server-database-wal-mode-pragma-tuning-and-why-litestream-changes-everything-for-51n9);
  [fractaledmind, Rails SQLite fine-tuning](https://dev.to/fractaledmind/enhancing-your-rails-app-with-sqlite-fine-tuning-your-database-52b7)
- sqlx 0.9.1 `SqliteConnectOptions` defaults:
  - `foreign_keys=ON` and `busy_timeout` 5 s.
  - `journal_mode` is not set unless requested, because switching into or out of WAL needs an
    exclusive lock that can't wait on `busy_timeout`.
  - `synchronous` is not set (SQLite's default is FULL).
  - It exposes `optimize_on_close(enabled, analysis_limit)` and `analysis_limit()`.
  - Source: local `sqlx-sqlite-0.9.1/src/options/mod.rs`, also at [launchbadge/sqlx](https://github.com/launchbadge/sqlx)
- When to use client/server instead:
  - SQLite "works great as the database engine for most low to medium traffic websites"
    (fewer than 100K hits/day is fine, and 10x that has been demonstrated).
  - Choose client/server when data sits across a network from the app, when there are many
    concurrent writers ("SQLite only supports one writer at a time per database file"), when
    data nears TB scale, or when the site "requires multiple servers".
  - Source: [SQLite, Appropriate Uses](https://www.sqlite.org/whentouse.html)
- `VACUUM INTO 'file'` makes "a consistent snapshot" of a live database. The output is minimal,
  with no deleted-content traces. The backup API uses less CPU and can be incremental. The
  output can be corrupt if interrupted, and is durable after completion if `synchronous` is
  NORMAL or FULL. The target file must not already exist — [SQLite, VACUUM](https://www.sqlite.org/lang_vacuum.html)
- Litestream:
  - Streams SQLite changes to object storage or files as a separate process, with no code
    changes — [litestream.io](https://litestream.io/)
  - Latest release is v0.5.18 (Nov 2026). The 0.5 line uses the LTX format and adds
    post-restore integrity validation (v0.5.10), compaction, and experimental VFS loadable
    extensions for read-only access to replicas.
  - Windows is not officially supported.
  - Source: [Litestream releases](https://github.com/benbjohnson/litestream/releases)

### Inferences
- Rules for AGENTS.md:
  1. Every `CREATE TABLE` ends with `STRICT`.
  2. All connection settings live in one place, `SqliteConnectOptions` in infra:
     - WAL;
     - `synchronous(Normal)`, which is currently missing: the code sets WAL but leaves
       synchronous at FULL. This is safe but slower; it is a deliberate choice to make;
     - `busy_timeout` 5 s;
     - `foreign_keys(true)`, explicit even though it is the default;
     - `optimize_on_close(true, ...)`, or a periodic `PRAGMA optimize`.
  3. Never put the database on a network filesystem.
  4. Write transactions that read first must use `BEGIN IMMEDIATE`. A deferred transaction that
     upgrades to a write can get `SQLITE_BUSY` immediately, without `busy_timeout` helping.
     This is a known SQLite behavior that I did not verify against a source in this session.
     Alternatively, route writes through a dedicated 1-connection writer pool and use a
     separate read pool.
  5. Keep transactions short. Never hold one across network or provider calls, since a
     long-held transaction blocks checkpoints and the single writer.
  6. Backups:
     - Litestream (0.5.x) continuous replication to object storage in production.
     - A `VACUUM INTO` snapshot before risky migrations.
     - A regular test restore with `PRAGMA integrity_check`.
     - Never copy the `.db` file alone while it is in use; the `-wal` holds recent commits.
  7. Triggers to move to Postgres: a second app instance or host (WAL can't be shared across
     hosts), sustained write contention visible as busy timeouts, a need for
     LISTEN/NOTIFY-style features or rich types, or data size heading towards hundreds of GB.
- Portability checklist:
  - Use only types that map 1:1: INTEGER→bigint, TEXT→text, BLOB→bytea, INTEGER 0/1→boolean,
    INTEGER epoch ms→timestamptz (needs conversion in the adapter).
  - Avoid `AUTOINCREMENT`, `INSERT OR REPLACE` (use `INSERT ... ON CONFLICT ... DO UPDATE`,
    which both engines support), `datetime('now')` defaults in queries, and `GLOB`.
  - Use `RETURNING`, which both engines support (SQLite since 3.35; general knowledge).
  - Keep SQL in the infra adapter behind repository ports, so the Postgres adapter is a new
    file, not a rewrite of callers.

### Gaps
- I did not fetch the official SQLite PRAGMA page in this session. The recommendations on
  `cache_size`, `mmap_size` and `temp_store` come from blogs and are omitted for lack of primary
  sourcing.
- I did not fetch the Litestream license or v0.5 docs. Point-in-time restore details are not
  confirmed here.
- `BEGIN CONCURRENT` and `wal2` are not in mainline SQLite docs as of this check. They were not
  covered by the WAL page.

## Soft delete vs hard delete; normalization; JSON columns

### Takeaway
Default to hard deletes with FK `ON DELETE` actions and an explicit retention or deletion story.
For this project, privacy rules (anonymous rooms, clones deleted at call end) favor real
deletion. Normalize to 3NF by default and denormalize only with a measured reason. JSON columns
are fine for opaque, read-whole payloads that are never filtered or joined on. They are not
fine for anything with invariants.

### Cited Findings
- Brandur Leach, "Soft Deletion Probably Isn't Worth It" (2022), on `deleted_at` soft deletion:
  - In more than ten years he never saw anyone actually use it to undelete.
  - Deletion usually has non-data side effects anyway.
  - Foreign keys stop protecting integrity.
  - "soft deletion logic bleeds out into all parts of your code" through forgotten
    `deleted_at IS NULL` predicates.
  - He proposes a separate `deleted_record` table: insert the row as JSON on delete, then
    hard-delete.
  - Sources: [brandur.org, soft deletion](https://brandur.org/soft-deletion); [brandur.org, deleted record insert](https://brandur.org/fragments/deleted-record-insert); [Simon Willison link (2022-07-19)](https://simonwillison.net/2022/Jul/19/soft-deletion-probably-isnt-worth-it)
- thoughtbot reaches similar conclusions — [thoughtbot, The hard truth about soft deletion](https://thoughtbot.com/blog/the-hard-truth-about-soft-deletion)
- Holywell advises against EAV tables, against splitting one table across many tables (for
  archiving or regions, which forces UNION queries), and against object-oriented design in
  relational schemas — [SQL Style Guide](https://www.sqlstyle.guide/)
- The STRICT `ANY` type preserves values exactly. It is the only escape hatch from typing in
  STRICT tables — [SQLite, STRICT Tables](https://www.sqlite.org/stricttables.html)
- The sqlx `json` feature is a default feature. It is currently disabled in this project
  because default features are off (local `sqlx-0.9.1/Cargo.toml`).

### Inferences
- Rules for AGENTS.md:
  1. Hard delete by default.
  2. If an audit trail or undo is a real requirement, either insert into a `deleted_records`
     table (`table_name`, `record_id`, `data` JSON, `deleted_at`) in the same transaction, or
     use an explicit state column (`status IN ('active','ended')`) when "ended" is a real
     domain state. Example: a room that ended is a state, not a deletion.
  3. Personal data has a retention rule written next to its table.
  4. Normalize to 3NF.
  5. No EAV.
  6. No arrays or CSV in TEXT columns.
  7. Many-to-many relationships use a link table with a composite PK and FKs on both sides.
  8. JSON is allowed only for opaque blobs that the app reads and writes whole (for example,
     provider config snapshots or debug metadata). The column is `TEXT` with
     `CHECK (json_valid(col))` in SQLite and `jsonb` in Postgres. Never put a field you query,
     join, sort or enforce uniqueness on inside JSON; promote it to a column.
- If soft delete is ever adopted for one table, every unique constraint must become a partial
  index `WHERE deleted_at IS NULL`, and every query must filter. This is the bug class Brandur
  describes.

### Gaps
- I did not fetch a SQLite JSON/JSONB primary source. JSONB arrived in 3.45.0 (2024-01-15,
  listed in the chronology), but its features were not confirmed from the release notes in
  this session.
- I found no 2024–2026 primary-source survey of soft-delete usage. The evidence is
  practitioner essays.
