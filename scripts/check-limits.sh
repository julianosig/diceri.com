#!/usr/bin/env bash
# Enforces repository size limits that linters cannot express.
# Limits are documented in AGENTS.md; change both together.
set -euo pipefail

readonly MAX_RUST_FILE_LINES=400
readonly MAX_ENTRIES_PER_DIR=15
readonly EXCLUDE_DIRS='(^|/)(\.git|target|node_modules|\.svelte-kit|build|generated|research|\.pnpm-store)(/|$)'

cd "$(git rev-parse --show-toplevel)"
failures=0

while IFS= read -r file; do
	lines=$(wc -l <"$file")
	if ((lines > MAX_RUST_FILE_LINES)); then
		echo "error: $file has $lines lines (max $MAX_RUST_FILE_LINES)"
		failures=$((failures + 1))
	fi
done < <(git ls-files --cached --others --exclude-standard '*.rs')

while IFS= read -r dir; do
	[[ "$dir" =~ $EXCLUDE_DIRS ]] && continue
	entries=$(git ls-files --cached --others --exclude-standard "$dir" |
		sed "s|^$dir/||" | cut -d/ -f1 | sort -u | wc -l)
	if ((entries > MAX_ENTRIES_PER_DIR)); then
		echo "error: $dir has $entries entries (max $MAX_ENTRIES_PER_DIR)"
		failures=$((failures + 1))
	fi
done < <(git ls-files --cached --others --exclude-standard | xargs -n1 dirname | sort -u | grep -v '^\.$')

if ((failures > 0)); then
	echo "$failures limit violation(s). Split files or group folders by feature."
	exit 1
fi
echo "limits ok"
