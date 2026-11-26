#!/usr/bin/env bash
# Builds a release and (re)starts the production user services on this host.
# Layout: ~/.local/share/diceri/releases/<commit>/ with `current` pointing at the live one,
# so rebuilding for development never swaps binaries under running services.
# Needs ~/.config/diceri/{production.env,tunnel.env}; see docs/deployment.md.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
data="${XDG_DATA_HOME:-$HOME/.local/share}/diceri"
units="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
commit="$(git -C "$root" rev-parse --short HEAD)"
release="$data/releases/$commit"

for file in production.env tunnel.env; do
  [[ -f "$HOME/.config/diceri/$file" ]] || { echo "missing ~/.config/diceri/$file" >&2; exit 1; }
done

(cd "$root" && just build)
mkdir -p "$release" "$units"
# The database lives in $data: owner-only (AGENTS.md §14, rule 23).
chmod 700 "$data"
cp "$root/target/release/diceri-api" "$root/target/release/diceri-media-agent" "$release/"
cp "$root/scripts/backup-db.sh" "$release/"
rm -rf "$release/web" && cp -r "$root/apps/web/build" "$release/web"
ln -sfn "$release" "$data/current"

cp "$root"/deploy/systemd/*.service "$root"/deploy/systemd/*.timer "$units/"
systemctl --user daemon-reload
systemctl --user enable diceri-api diceri-agent diceri-tunnel >/dev/null
systemctl --user enable --now diceri-backup.timer >/dev/null
systemctl --user restart diceri-agent diceri-api
# The tunnel only forwards to the API port; restarting it would drop every visitor
# for a few seconds (Cloudflare error 1033), so it is only started if stopped.
systemctl --user start diceri-tunnel

# Keep the three newest releases for a quick rollback (re-point `current`, restart).
ls -1dt "$data"/releases/*/ | tail -n +4 | xargs -r rm -rf
echo "deployed $commit"
