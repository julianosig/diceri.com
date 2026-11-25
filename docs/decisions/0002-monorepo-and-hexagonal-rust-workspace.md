---
status: accepted
date: 2026-11-25
---

# Monorepo with a hexagonal Rust workspace

## Context and problem statement

The product has a Rust backend, a SvelteKit frontend and soon a media agent.
Speech providers will be swapped as the market moves, so provider code must be
isolated from business rules.

## Considered options

- One repository: pnpm workspace + Cargo workspace, layered Rust crates
- Separate repositories per app
- One Rust crate with modules only

## Decision outcome

Chosen option: "one repository with `apps/`, `crates/`, `packages/`", with Rust
crates `domain ← application ← infra ← apps`. Ports are traits in
`application`; adapters live in `infra`; binaries are composition roots.

### Consequences

- Good, because a provider swap touches only `infra` and wiring.
- Good, because crate boundaries make illegal dependencies a compile error.
- Bad, because more `Cargo.toml` files to maintain; mitigated by
  `[workspace.dependencies]` and `[workspace.lints]`.

### Confirmation

`domain` compiles without tokio/axum; `cargo deny` and clippy run on the whole workspace in CI.
