# Deployment

Production runs on one Linux host as three **systemd user services**, published
through a **Cloudflare Tunnel** (no open inbound ports, HTTPS at the edge):

```
browser ──https──▶ Cloudflare ──tunnel──▶ cloudflared ──▶ diceri-api :8100 ──▶ diceri-media-agent :8105
   └──────────── WebRTC (audio) ────────────▶ LiveKit Cloud ◀──────────────────────────┘
```

- `diceri-api` serves the web app (`DICERI_WEB_DIR`) and the API on one origin.
- Audio never crosses the tunnel: browsers and the agent talk to LiveKit Cloud directly.
- Units: [`deploy/systemd/`](../deploy/systemd/). Settings: [`deploy/production.env.example`](../deploy/production.env.example).

## One-time setup

1. `sudo loginctl enable-linger $USER` so user services run without a login session.
2. `cloudflared` in `~/.local/bin` (static binary from Cloudflare's releases).
3. A remotely managed tunnel with ingress `diceri.com → http://127.0.0.1:8100` and a
   proxied `CNAME diceri.com → <tunnel-id>.cfargotunnel.com`.
4. `~/.config/diceri/production.env` (mode 600): provider keys from `.env` plus the
   production overrides; `~/.config/diceri/tunnel.env` with `TUNNEL_TOKEN=…`.

## Deploying

`just deploy` builds, copies the release to `~/.local/share/diceri/releases/<commit>`,
points `current` at it and restarts the services. Restarting the agent ends calls in
progress, so deploy when the presence board is quiet.

Rollback: `ln -sfn ~/.local/share/diceri/releases/<older> ~/.local/share/diceri/current`
then `systemctl --user restart diceri-api diceri-agent`.

Logs: `journalctl --user -u diceri-api -f` (JSON; ids and timings only, never audio,
transcripts or keys).

## Isolation from development

Production uses its own ports (8100/8105 vs 8090/8095) and `DICERI_DEPLOYMENT=prod`,
so voice clones are named `diceri-prod-…`; dev and end-to-end agents never sweep them.
