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
5. A LiveKit Cloud webhook (Settings → Webhooks, [ADR 0018](decisions/0018-livekit-webhooks-for-call-membership.md)):
   URL `https://diceri.com/api/v1/livekit/webhook`, signed with the key in
   `DICERI_LIVEKIT_API_KEY`. Without it calls are still kept to their rules by the minute
   sweep, only later. "Send a test event" should log nothing at warn level.

## Deploying

`just deploy` builds, copies the release to `~/.local/share/diceri/releases/<commit>`,
points `current` at it and restarts the services. Restarting the agent ends calls in
progress, so deploy when the presence board is quiet.

Rollback: `ln -sfn ~/.local/share/diceri/releases/<older> ~/.local/share/diceri/current`
then `systemctl --user restart diceri-api diceri-agent`.

Logs: `journalctl --user -u diceri-api -f` (JSON; ids and timings only, never audio,
transcripts or keys).

## Database and backups

Accounts live in SQLite at `DICERI_DATABASE_URL` (production:
`~/.local/share/diceri/diceri.db`, directory `0700`). Migrations run at API startup.

`diceri-backup.timer` runs `backup-db.sh` daily: `diceri-api backup <file>` writes a
`VACUUM INTO` snapshot to `~/.local/share/diceri/backups/`, checks it with
`PRAGMA integrity_check` and keeps the 7 newest. Restore: stop `diceri-api`, copy a
snapshot over `diceri.db` (delete the old `-wal`/`-shm` files), start it again.
Run `systemctl --user start diceri-backup` to take one on demand.

## Isolation from development

Production uses its own ports (8100/8105 vs 8090/8095) and `DICERI_DEPLOYMENT=prod`,
so voice clones are named `diceri-prod-…`; dev and end-to-end agents never sweep them.
