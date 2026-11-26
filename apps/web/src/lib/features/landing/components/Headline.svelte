<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import { fitFontSize } from '../headline-fit.ts';

	/** Largest size (desktop) and the column the two lines must fit in. */
	const MAX_PX = 60;
	const MAX_WIDTH_PX = 768;

	const t = $derived(i18n.t.hero);
	const fontSize = $derived(fitFontSize([t.headline, t.promise], MAX_PX, MAX_WIDTH_PX));
</script>

<!-- The promise, always two lines in every language: the size shrinks instead of wrapping. -->
<h1 class="font-semibold tracking-tight whitespace-nowrap" style:font-size={fontSize}>
	<span class="block leading-tight">{t.headline}</span>
	<span
		class="block bg-gradient-to-r from-brand-violet to-brand-500 bg-clip-text leading-tight text-transparent"
	>
		{t.promise}
	</span>
</h1>
