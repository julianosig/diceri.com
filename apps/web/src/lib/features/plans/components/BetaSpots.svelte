<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import type { BetaSpots } from '../types.ts';

	let { beta }: { beta: BetaSpots } = $props();
	const t = $derived(i18n.t.plans);
</script>

<!-- The public beta's free spots, set like an editorial figure: a large number, then what it
     counts in readable type, then the context as a small label. -->
{#if beta.spotsLeft > 0}
	<p class="mx-auto flex w-fit items-center gap-4 text-start">
		<span
			class="bg-gradient-to-r from-brand-violet to-brand-500 bg-clip-text text-6xl leading-none font-bold tracking-tight text-transparent tabular-nums"
		>
			{beta.spotsLeft}
		</span>
		<span class="border-line flex flex-col gap-1 border-s ps-4">
			<span class="text-lg leading-tight font-semibold text-ink">{t.betaLeft}</span>
			<span
				class="text-xs font-semibold tracking-wider text-brand-600 uppercase dark:text-brand-400"
			>
				{t.betaDetail(beta.spots)}
			</span>
		</span>
	</p>
{:else}
	<p class="text-center text-sm font-medium text-ink-muted">{t.betaFull}</p>
{/if}
