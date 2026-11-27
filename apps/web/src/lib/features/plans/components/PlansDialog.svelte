<script lang="ts">
	import { Check, Sparkles, X } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import type { PlanInfo } from '../types.ts';

	interface Props {
		plans: PlanInfo[];
		onclose: () => void;
	}

	let { plans, onclose }: Props = $props();
	const t = $derived(i18n.t.plans);
	let dialog = $state<HTMLDialogElement>();

	$effect(() => {
		dialog?.showModal();
	});
</script>

<!-- Every plan side by side: Free is everyone's, the paid ones are coming soon. -->
<dialog
	bind:this={dialog}
	class="m-auto w-[min(100vw-2rem,48rem)] rounded-tile border border-ink-muted/20 bg-surface p-6 text-ink backdrop:bg-black/60"
	aria-labelledby="plans-title"
	{onclose}
>
	<header class="mb-2 flex items-center justify-between gap-4">
		<h2 id="plans-title" class="text-2xl font-semibold">{t.title}</h2>
		<button
			type="button"
			class="-me-2 rounded-full p-2 text-ink-muted hover:bg-surface-raised hover:text-ink"
			aria-label={t.close}
			onclick={() => dialog?.close()}
		>
			<X class="size-5" aria-hidden="true" />
		</button>
	</header>
	<p class="mb-5 text-sm text-ink-muted">{t.intro}</p>
	<ul class="grid gap-3 sm:grid-cols-3">
		{#each plans as info (info.plan)}
			{@const free = info.plan === 'free'}
			<li
				class={[
					'flex flex-col gap-3 rounded-xl border p-4 text-start',
					free ? 'border-brand-500 bg-brand-500/10' : 'border-ink-muted/20'
				]}
			>
				<div class="flex items-center justify-between gap-2">
					<p class="text-lg font-semibold">{t[info.plan]}</p>
					<span
						class={[
							'rounded-full px-2 py-0.5 text-xs font-semibold',
							free ? 'bg-brand-500 text-white' : 'bg-surface-raised text-ink-muted'
						]}
					>
						{free ? t.yourPlan : t.soon}
					</span>
				</div>
				<ul class="flex flex-col gap-1.5 text-sm">
					<li class="flex items-center gap-2">
						<Check class="size-4 shrink-0 text-online" aria-hidden="true" />
						{t.callUpTo(info.callMinutes)}
					</li>
					<li class="flex items-center gap-2">
						<Check class="size-4 shrink-0 text-online" aria-hidden="true" />
						{t.perDay(info.dailyMinutes)}
					</li>
					{#if !free}
						<li class="flex items-center gap-2 text-ink-muted">
							<Sparkles class="size-4 shrink-0 text-brand-500" aria-hidden="true" />
							{t.soon}
						</li>
					{/if}
				</ul>
			</li>
		{/each}
	</ul>
</dialog>
