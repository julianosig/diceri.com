<script lang="ts">
	import { resolve } from '$app/paths';
	import { GithubMark } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';

	// AGPL-3.0 section 13: every user of the service can get its source. Literals here, not
	// imports, so lint can tell these links are external.
	const SOURCE_URL = 'https://github.com/julianosig/diceri.com';
	const LICENSE_URL = 'https://github.com/julianosig/diceri.com/blob/main/LICENSE';
</script>

<footer class="flex items-center gap-1.5 text-xs text-ink-muted">
	{i18n.t.footer.freeSoftware} ·
	<a class="underline" href={LICENSE_URL}>AGPL-3.0</a>
	·
	<a class="inline-flex items-center gap-1 underline" href={SOURCE_URL}>
		<GithubMark class="size-3.5" />
		{i18n.t.footer.sourceCode}
	</a>
	·
	<a class="underline" href={resolve('/terms')}>{i18n.t.footer.terms}</a>
</footer>
