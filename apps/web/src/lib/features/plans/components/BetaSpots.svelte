<script lang="ts">
	import { Sparkles, Users } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import type { BetaSpots } from '../types.ts';

	let { beta }: { beta: BetaSpots } = $props();
	const t = $derived(i18n.t.plans);
	/** How full the beta is, for the bar (the text says the same in words). */
	const taken = $derived(beta.spots > 0 ? ((beta.spots - beta.spotsLeft) / beta.spots) * 100 : 100);
</script>

<!-- The public beta's free spots, at a glance: label, count and how full it is. -->
<div
	class="mx-auto flex w-full max-w-md flex-col gap-2.5 rounded-2xl border border-brand-500/30 bg-brand-500/10 px-4 py-3"
>
	<div class="flex flex-wrap items-center justify-between gap-x-3 gap-y-1">
		<span
			class="flex items-center gap-1.5 text-xs font-semibold tracking-wider text-brand-600 uppercase dark:text-brand-400"
		>
			<Sparkles class="size-4" aria-hidden="true" />
			{t.betaLabel}
		</span>
		<span class="flex items-center gap-1.5 text-sm font-semibold text-ink">
			<Users class="size-4 text-brand-500" aria-hidden="true" />
			{beta.spotsLeft > 0 ? t.betaSpots(beta.spotsLeft, beta.spots) : t.betaFull}
		</span>
	</div>
	<div class="h-1.5 overflow-hidden rounded-full bg-brand-500/15" aria-hidden="true">
		<div class="h-full rounded-full bg-brand-500 transition-[width]" style:width="{taken}%"></div>
	</div>
</div>
