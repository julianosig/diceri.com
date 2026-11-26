<script lang="ts">
	import { GithubMark } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	// The public repository. A literal here, not an import, so lint can tell it is external.
	const SOURCE_URL = 'https://github.com/julianosig/diceri.com';
</script>

<!-- Says Diceri is open source and takes you to the code; just the mark on small phones. -->
<a
	href={SOURCE_URL}
	target="_blank"
	rel="noopener noreferrer"
	aria-label={i18n.t.hero.openSource}
	class="inline-flex items-center gap-2 rounded-full border border-ink-muted/25 bg-surface-raised/60 p-2 text-sm font-medium whitespace-nowrap text-ink transition-colors hover:border-ink-muted/50 hover:bg-surface-raised focus-visible:ring-2 focus-visible:ring-brand-500 focus-visible:outline-none sm:py-1.5 sm:ps-2.5 sm:pe-3.5"
>
	<GithubMark class="size-4" />
	<span class="hidden sm:inline">{i18n.t.hero.openSource}</span>
</a>
