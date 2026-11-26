<script lang="ts">
	import { Volume2 } from '@lucide/svelte';
	import { languageLabel } from '#lib/features/translation/index.ts';
	import { avatarHue, avatars, channelName, peopleLabel } from '../ranking.ts';
	import type { Presence, RankedRoom } from '../types.ts';

	interface Props {
		/** `null` while the first snapshot loads. */
		snapshot: Presence | null;
	}

	let { snapshot }: Props = $props();

	function describe(room: RankedRoom, rank: number): string {
		const heard = room.languages.map(languageLabel).join(', ');
		return `#${String(rank)}: ${peopleLabel(room.people)}${heard ? `, hearing ${heard}` : ''}`;
	}
</script>

<section
	aria-labelledby="live-rooms-title"
	class="w-full max-w-md rounded-tile bg-stage p-3 text-left text-white shadow-xl ring-1 ring-white/5"
>
	<header class="flex items-center justify-between gap-3 px-2 pb-2">
		<h2 id="live-rooms-title" class="flex items-center gap-2 text-sm font-semibold">
			<span class="relative flex size-2.5" aria-hidden="true">
				<span class="absolute inline-flex size-full animate-ping rounded-full bg-online opacity-75"
				></span>
				<span class="relative inline-flex size-2.5 rounded-full bg-online"></span>
			</span>
			<span><span data-testid="online-count">{snapshot?.online ?? '–'}</span> online now</span>
		</h2>
		<span class="text-xs text-white/60">
			{snapshot?.liveRooms ?? 0}
			{snapshot?.liveRooms === 1 ? 'live room' : 'live rooms'}
		</span>
	</header>

	{#if !snapshot || snapshot.ranking.length === 0}
		<p class="px-2 py-3 text-sm text-white/60">
			Quiet right now. Start a meeting and top the board.
		</p>
	{:else}
		<ol class="flex flex-col gap-0.5">
			{#each snapshot.ranking as room, index (index)}
				{@const dots = avatars(room)}
				<li
					class="flex items-center gap-2.5 rounded-lg px-2 py-1.5 transition-colors hover:bg-tile"
					aria-label={describe(room, index + 1)}
				>
					<span
						class={[
							'w-5 text-center text-xs font-bold tabular-nums',
							index === 0 ? 'text-gold' : 'text-white/50'
						]}
					>
						{index + 1}
					</span>
					<Volume2 class="size-4 shrink-0 text-white/50" aria-hidden="true" />
					<span class="truncate font-medium text-white/90">{channelName(room)}</span>
					<span class="ml-auto flex shrink-0 items-center -space-x-1.5" aria-hidden="true">
						{#each { length: dots.shown }, dot (dot)}
							<span
								class="size-5 rounded-full ring-2 ring-stage"
								style:background-color="oklch(0.7 0.14 {avatarHue(room, dot)})"
							></span>
						{/each}
						{#if dots.hidden > 0}
							<span
								class="flex size-5 items-center justify-center rounded-full bg-control text-[10px] ring-2 ring-stage"
							>
								+{dots.hidden}
							</span>
						{/if}
					</span>
					<span class="w-6 text-right text-xs text-white/60 tabular-nums" aria-hidden="true">
						{room.people}
					</span>
				</li>
			{/each}
		</ol>
	{/if}
</section>
