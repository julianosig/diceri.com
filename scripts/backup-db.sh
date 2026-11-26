#!/usr/bin/env bash
# Daily checked snapshot of the production accounts database (AGENTS.md §14, rule 23).
# Uses the deployed binary (`diceri-api backup`, VACUUM INTO + integrity_check) and keeps
# the 7 newest snapshots, so erased rows leave backups within a week (docs/data-inventory.md).
set -euo pipefail

data="${XDG_DATA_HOME:-$HOME/.local/share}/diceri"
dir="$data/backups"
keep=7

mkdir -p "$dir"
chmod 700 "$data" "$dir"
target="$dir/diceri-$(date -u +%Y%m%dT%H%M%SZ).db"
"$data/current/diceri-api" backup "$target"
chmod 600 "$target"
ls -1t "$dir"/diceri-*.db | tail -n +$((keep + 1)) | xargs -r rm -f
echo "backup: $target"
