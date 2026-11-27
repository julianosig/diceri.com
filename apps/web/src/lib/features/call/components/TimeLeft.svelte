<script lang="ts">
	import { Timer } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import { cn } from '#lib/utils/cn.ts';

	interface Props {
		/** When the owner's plan ends the call (ms since epoch). */
		deadline: number;
	}

	let { deadline }: Props = $props();

	/** In the last minutes the timer turns red, so nobody is cut off by surprise. */
	const WARN_MS = 5 * 60_000;

	let now = $state(Date.now());
	$effect(() => {
		const timer = setInterval(() => {
			now = Date.now();
		}, 1000);
		return () => {
			clearInterval(timer);
		};
	});

	const left = $derived(Math.max(0, deadline - now));
	const label = $derived(
		`${String(Math.floor(left / 60_000))}:${String(Math.floor(left / 1000) % 60).padStart(2, '0')}`
	);
</script>

<!-- The call time the owner's plan has left, all call long; a timer, not a live announcement. -->
<div
	class={cn(
		'flex w-fit items-center gap-2 rounded-full px-3 py-1.5 text-sm font-semibold text-white tabular-nums',
		left <= WARN_MS ? 'bg-danger' : 'bg-control'
	)}
	role="timer"
>
	<Timer class="size-4" aria-hidden="true" />
	{i18n.t.limits.timeLeft(label)}
</div>
