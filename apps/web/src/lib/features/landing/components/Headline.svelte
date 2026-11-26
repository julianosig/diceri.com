<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import { fitFontSize } from '../headline-fit.ts';

	/** The column the lines must fit in, and each line's largest size (desktop). */
	const MAX_WIDTH_PX = 768;
	const HEADLINE_MAX_PX = 60;
	const PROMISE_MAX_PX = 36;

	const t = $derived(i18n.t.hero);
	const headlineSize = $derived(fitFontSize([t.headline], HEADLINE_MAX_PX, MAX_WIDTH_PX));
	const promiseSize = $derived(fitFontSize([t.promise], PROMISE_MAX_PX, MAX_WIDTH_PX));
</script>

<!--
	The promise: what Diceri is (the headline), what it does for you (the gradient line),
	each kept on one line by shrinking instead of wrapping, then the voice as the moat.
-->
<div class="flex flex-col items-center gap-4">
	<h1 class="flex flex-col items-center gap-1 font-semibold tracking-tight whitespace-nowrap">
		<span class="leading-tight" style:font-size={headlineSize}>{t.headline}</span>
		<span
			class="bg-gradient-to-r from-brand-violet to-brand-500 bg-clip-text leading-tight text-transparent"
			style:font-size={promiseSize}
		>
			{t.promise}
		</span>
	</h1>
	<p class="max-w-xl text-base text-balance text-ink-muted sm:text-lg">{t.voice}</p>
</div>
