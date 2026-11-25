# Single entry point for every task. Run `just` to list recipes.

set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Install dependencies and git hooks.
setup:
    pnpm install
    pnpm exec lefthook install
    cargo fetch

# Run API and web dev servers together.
dev:
    trap 'kill 0' EXIT; cargo run -p diceri-api & pnpm --filter @diceri/web dev & wait

# Format all code.
fmt:
    cargo fmt --all
    pnpm --filter @diceri/web format

# Everything CI runs. Must pass before every commit is pushed.
check: check-rust check-web limits

check-rust:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test --workspace --locked
    cargo deny check

check-web:
    pnpm --filter @diceri/web check
    pnpm --filter @diceri/web lint
    pnpm --filter @diceri/web exec vitest --run

# Repository size limits (files per folder, Rust file length).
limits:
    ./scripts/check-limits.sh

# Browser end-to-end tests (needs `pnpm exec playwright install --with-deps chromium`).
e2e:
    pnpm --filter @diceri/web exec playwright test

# Production builds.
build:
    cargo build --release -p diceri-api
    pnpm --filter @diceri/web build
