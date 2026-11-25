# Single entry point for every task. Run `just` to list recipes.

set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Install dependencies and git hooks.
setup: toolchain
    pnpm install
    pnpm exec lefthook install
    cargo fetch

# Local clang required by libwebrtc (LiveKit SDK); see docs/development.md.
toolchain:
    ./scripts/install-webrtc-clang.sh

# Run API and web dev servers together.
dev:
    trap 'kill 0' EXIT; cargo run -p diceri-api & pnpm --filter @diceri/web dev & wait

# Format all code.
fmt:
    cargo fmt --all
    pnpm --filter @diceri/web format

# ── Test pipeline ─────────────────────────────────────────────────────────
# Layer 1 `check`   static analysis, seconds          (pre-commit / pre-push)
# Layer 2 `test`    unit + router tests, no network   (pre-push)
# Layer 3 `test-browser`  component tests in Chromium (CI)
# Layer 4 `e2e`     real API + web build + LiveKit    (CI, before deploy)
# See docs/testing.md.

# Fast gate before every push: static checks + unit tests.
verify: check test

# Everything, as CI runs it.
ci: verify test-browser e2e

# Layer 1: formatting, lints, types, size limits, dependency policy.
check: limits
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo deny check
    pnpm --filter @diceri/web check
    pnpm --filter @diceri/web lint

# Layer 2: Rust unit + HTTP router tests, web logic tests. No network.
test:
    cargo test --workspace --locked
    pnpm --filter @diceri/web test

# Layer 3: Svelte component tests in headless Chromium.
test-browser:
    pnpm --filter @diceri/web test:browser

# Media tests skip unless DICERI_LIVEKIT_* is set in .env or the environment.
# Layer 4: end-to-end smoke (real API, production web build, fake microphone).
e2e:
    pnpm --filter @diceri/web test:e2e

# Live provider tests (real Soniox, Gemini, ElevenLabs; costs cents; needs .env keys).
test-live:
    cargo test -p diceri-infra --test live_providers -- --ignored --nocapture --test-threads=1

# Repository size limits (files per folder, Rust file length).
limits:
    ./scripts/check-limits.sh

# One-time: download Chromium for Playwright (system libraries need sudo, see docs/testing.md).
browsers:
    pnpm --filter @diceri/web exec playwright install chromium

# Production builds.
build:
    cargo build --release -p diceri-api
    pnpm --filter @diceri/web build
