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
cp "$root/target/release/diceri-api" "$root/target/release/diceri-media-agent" "$release/"
rm -rf "$release/web" && cp -r "$root/apps/web/build" "$release/web"
ln -sfn "$release" "$data/current"

cp "$root"/deploy/systemd/*.service "$units/"
systemctl --user daemon-reload
systemctl --user enable diceri-api diceri-agent diceri-tunnel >/dev/null
systemctl --user restart diceri-agent diceri-api diceri-tunnel

# Keep the three newest releases for a quick rollback (re-point `current`, restart).
ls -1dt "$data"/releases/*/ | tail -n +4 | xargs -r rm -rf
echo "deployed $commit"
