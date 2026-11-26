<script lang="ts">
	import { resolve } from '$app/paths';
	import { languageLabel } from '#lib/features/translation/index.ts';
	import { avatarHue, avatars, languageTags, peopleLabel } from '../ranking.ts';
	import type { Presence, RankedRoom } from '../types.ts';

	interface Props {
		/** `null` while the first snapshot loads. */
		snapshot: Presence | null;
	}

	let { snapshot }: Props = $props();

	function describe(room: RankedRoom, rank: number): string {
		const host = ` hosted by @${room.owner}`;
		const heard = room.languages.map(languageLabel).join(', ');
		const action = room.entry === 'open' ? 'join now' : 'ask to join';
		return `#${String(rank)} ${room.title}${host}: ${peopleLabel(room.people)}${heard ? `, hearing ${heard}` : ''}. ${action}`;
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
			No public rooms right now. Create one, turn on “Show on the live board” and top it.
		</p>
	{:else}
		<ol class="flex flex-col gap-0.5">
			{#each snapshot.ranking as room, index (room.code)}
				{@const dots = avatars(room)}
				{@const tags = languageTags(room)}
				<li>
					<a
						href={resolve('/r/[room]', { room: room.code })}
						aria-label={describe(room, index + 1)}
						class="group flex items-center gap-3 rounded-lg px-2 py-2 transition-colors hover:bg-tile focus-visible:bg-tile focus-visible:outline-none"
					>
						<span
							class={[
								'w-5 shrink-0 text-center text-sm font-bold tabular-nums',
								index === 0 ? 'text-gold' : 'text-white/50'
							]}
							aria-hidden="true"
						>
							{index + 1}
						</span>
						<span class="flex min-w-0 flex-1 flex-col" aria-hidden="true">
							<span class="truncate font-semibold text-white">{room.title}</span>
							<span class="truncate text-xs text-white/60">
								by @{room.owner}{tags ? ` · ${tags}` : ''}
							</span>
						</span>
						<span class="flex shrink-0 items-center -space-x-1.5" aria-hidden="true">
							{#each { length: dots.shown }, dot (dot)}
								<span
									class="size-5 rounded-full ring-2 ring-stage group-hover:ring-tile"
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
						<span
							class={[
								'shrink-0 rounded-full px-2.5 py-1 text-xs font-semibold',
								room.entry === 'open' ? 'bg-online text-stage' : 'bg-control text-white'
							]}
							aria-hidden="true"
						>
							{room.entry === 'open' ? 'Join' : 'Ask'}
						</span>
					</a>
				</li>
			{/each}
		</ol>
	{/if}
</section>
