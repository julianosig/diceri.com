<script lang="ts">
	import { Timer } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';

	interface Props {
		/** When the owner's plan ends the call (ms since epoch). */
		deadline: number;
	}

	let { deadline }: Props = $props();

	/** The countdown shows only in the last minutes, so it never nags a short call. */
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

<!-- The plan's time left, in the last minutes of a call; a timer, not a live announcement. -->
{#if left <= WARN_MS}
	<div
		class="absolute top-4 left-1/2 flex -translate-x-1/2 items-center gap-2 rounded-full bg-black/70 px-4 py-2 text-sm font-semibold text-white tabular-nums shadow-lg"
		role="timer"
	>
		<Timer class="size-4" aria-hidden="true" />
		{i18n.t.limits.timeLeft(label)}
	</div>
{/if}
