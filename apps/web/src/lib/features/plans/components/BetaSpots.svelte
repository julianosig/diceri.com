<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import type { BetaSpots } from '../types.ts';

	let { beta }: { beta: BetaSpots } = $props();
	const t = $derived(i18n.t.plans);
</script>

<!-- The public beta's free spots, set like an editorial figure: the number does the talking. -->
{#if beta.spotsLeft > 0}
	<p class="mx-auto flex w-fit items-center gap-3 text-start">
		<span
			class="bg-gradient-to-r from-brand-violet to-brand-500 bg-clip-text text-5xl leading-none font-bold tracking-tight text-transparent tabular-nums"
		>
			{beta.spotsLeft}
		</span>
		<span class="max-w-40 text-sm leading-snug text-ink-muted">{t.betaCaption(beta.spots)}</span>
	</p>
{:else}
	<p class="text-center text-sm font-medium text-ink-muted">{t.betaFull}</p>
{/if}
