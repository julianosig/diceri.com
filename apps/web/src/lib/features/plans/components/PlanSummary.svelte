<script lang="ts">
	import { onMount } from 'svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { getPlans } from '../plans-api.ts';
	import type { PlanInfo, Usage } from '../types.ts';

	interface Props {
		/** The signed-in account's use; without it, the free plan's allowance is shown. */
		usage?: Usage;
	}

	let { usage }: Props = $props();
	const t = $derived(i18n.t.plans);
	let free = $state<PlanInfo>();

	onMount(() => {
		getPlans()
			.then((plans) => {
				free = plans.find((plan) => plan.plan === 'free');
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
