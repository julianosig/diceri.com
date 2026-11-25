# Diceri

Anonymous, link-joined meetings with live voice translation: each person hears
the other in their own language, in a voice that sounds like the speaker.

- Contributor and agent rules: [`AGENTS.md`](AGENTS.md)
- Architecture: [`docs/architecture.md`](docs/architecture.md)
- Decisions: [`docs/decisions/`](docs/decisions/)

## Quick start

Requirements: Rust 1.100 (via rustup), Node 26 LTS, pnpm 12, [`just`](https://just.systems),
and the system packages in [`docs/development.md`](docs/development.md).

```sh
just setup   # dependencies + git hooks
just dev     # API on :8090, web on :5180
just check   # everything CI runs
```

## License

Diceri is free software under the [GNU Affero General Public License v3.0 only](LICENSE)
(`AGPL-3.0-only`). If you run a modified version as a network service, you must
offer its users the corresponding source code (AGPL section 13).

An [additional permission](LICENSE-EXCEPTION.md) (AGPL section 7) allows
combining Diceri with LiveKit's proprietary Krisp noise filter, which the web
app uses for noise and background-voice cancellation.
