<script lang="ts">
	import { onMount } from 'svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { getPlanOffer } from '../plans-api.ts';
	import type { BetaSpots, PlanInfo, Usage } from '../types.ts';
	import BetaSpotsBadge from './BetaSpots.svelte';

	interface Props {
		/** The signed-in account's use; without it, the free plan's allowance is shown. */
		usage?: Usage;
	}

	let { usage }: Props = $props();
	const t = $derived(i18n.t.plans);
	let free = $state<PlanInfo>();
	/** Shown to visitors who do not have an account yet. */
	let beta = $state<BetaSpots>();

	onMount(() => {
		getPlanOffer()
			.then((offer) => {
				free = offer.plans.find((plan) => plan.plan === 'free');
				beta = offer.beta;
			})
			.catch(() => {
				free = undefined;
			});
	});
</script>

<!-- The free allowance, stated plainly under the main button (or what is left of it). -->
<p class="text-center text-sm text-ink-muted">
	{#if usage}
		{t.usedThisMonth(usage.usedThisMonth, usage.monthlyMinutes)}
	{:else if free}
		{t.freeLine(free.monthlyMinutes)}
	{/if}
</p>
{#if beta && !usage}
	<div class="mt-4"><BetaSpotsBadge {beta} /></div>
{/if}
