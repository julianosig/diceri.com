<script lang="ts">
	import { ConnectionQuality } from 'livekit-client';
	import type { RemotePeer } from '../call-session.svelte.ts';

	let { peers, ownQuality }: { peers: RemotePeer[]; ownQuality: ConnectionQuality } = $props();
</script>

<div class="flex flex-col gap-2 text-sm">
	<p>Your connection: <strong>{ownQuality}</strong></p>
	{#if peers.length === 0}
		<p class="text-ink-muted">Waiting for the other person to join…</p>
	{:else}
		<ul class="flex flex-col gap-1" aria-live="polite">
			{#each peers as peer (peer.identity)}
				<li class="flex items-center gap-2">
					<span
						class="size-2.5 rounded-full {peer.speaking ? 'bg-green-500' : 'bg-ink-muted/40'}"
						aria-hidden="true"
					></span>
					Guest · connection <strong>{peer.quality}</strong>
					{#if peer.speaking}<span class="sr-only">speaking</span>{/if}
				</li>
			{/each}
		</ul>
	{/if}
</div>
