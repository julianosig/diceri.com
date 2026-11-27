<script lang="ts">
	import { onMount } from 'svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { getPlans } from '../plans-api.ts';
	import type { PlanInfo, Usage } from '../types.ts';
	import PlansDialog from './PlansDialog.svelte';

	interface Props {
		/** The signed-in account's use today; without it, the free plan's limits are shown. */
		usage?: Usage;
	}

	let { usage }: Props = $props();
	const t = $derived(i18n.t.plans);
	let plans = $state<PlanInfo[]>([]);
	let open = $state(false);
	const free = $derived(plans.find((plan) => plan.plan === 'free'));

	onMount(() => {
		getPlans()
			.then((list) => {
				plans = list;
			})
			.catch(() => {
				plans = [];
			});
	});
</script>

<!-- The limits that apply, stated plainly under the main button, and the plans behind them. -->
<p class="flex flex-wrap items-center justify-center gap-x-2 gap-y-1 text-sm text-ink-muted">
	{#if usage}
		<span>{t.usedToday(usage.usedToday, usage.dailyMinutes)}</span>
		<span aria-hidden="true">·</span>
		<span>{t.callUpTo(usage.callMinutes)}</span>
	{:else if free}
		<span>{t.freeLine(free.callMinutes, free.dailyMinutes)}</span>
	{/if}
	{#if plans.length > 0}
		<button
			type="button"
			class="font-medium text-brand-500 underline-offset-2 hover:underline"
			onclick={() => (open = true)}
		>
			{t.seePlans}
		</button>
	{/if}
</p>
{#if open}
	<PlansDialog {plans} onclose={() => (open = false)} />
{/if}
