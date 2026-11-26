<script lang="ts">
	import { ArrowUpRight } from '@lucide/svelte';
	import { GithubMark } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	// The public repository. A literal here, not an import, so lint can tell it is external.
	const SOURCE_URL = 'https://github.com/julianosig/diceri.com';
</script>

<!-- Says up front that Diceri is free software, and takes you to the code. -->
<a
	href={SOURCE_URL}
	target="_blank"
	rel="noopener noreferrer"
	class="inline-flex items-center gap-2 rounded-full border border-ink-muted/25 bg-surface-raised/60 py-1.5 ps-2.5 pe-3 text-sm font-medium text-ink transition-colors hover:border-ink-muted/50 hover:bg-surface-raised focus-visible:ring-2 focus-visible:ring-brand-500 focus-visible:outline-none"
>
	<GithubMark class="size-4" />
	{i18n.t.hero.openSource}
	<span class="rounded-full bg-brand-500/15 px-2 py-0.5 text-xs text-brand-500">AGPL-3.0</span>
	<ArrowUpRight class="size-4 text-ink-muted" aria-hidden="true" />
</a>
