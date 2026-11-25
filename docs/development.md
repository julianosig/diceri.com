# Development setup

## System packages (Debian/Ubuntu, once per machine)

The media agent links LiveKit's prebuilt libwebrtc, which needs these headers
and libraries:

```sh
sudo apt install -y libglib2.0-dev libdrm-dev libgbm-dev libx11-dev libxext-dev \
  libxfixes-dev libxdamage-dev libxrandr-dev libxcomposite-dev libva-dev
```

Browser tests need Chromium's runtime libraries:

```sh
sudo pnpm --filter @diceri/web exec playwright install-deps chromium
```

## Toolchains

- Rust (pinned by `rust-toolchain.toml`), Node 26 LTS, pnpm 12, `just`.
- **clang for libwebrtc:** libwebrtc ships a hermetic libc++ that requires the
  exact Chromium clang it was built with (newer than distro clang). `just
  toolchain` downloads it into `.toolchain/clang` (gitignored, ~70 MB, no sudo),
  and `.cargo/config.toml` points `CXX` at it. When the `livekit` crate is
  upgraded, update the revision in `scripts/install-webrtc-clang.sh` from
  `CR_CLANG_REVISION` in the artifact's `webrtc.ninja`.

## Secrets

Copy `.env.example` to `.env` and fill in provider keys. Never paste keys in
chats or commit them.

## First run

```sh
just setup
just dev
```
