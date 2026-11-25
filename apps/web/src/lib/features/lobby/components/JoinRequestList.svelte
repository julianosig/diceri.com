<script lang="ts">
	import { Button } from '#lib/components/ui/index.ts';
	import type { PendingRequest } from '../lobby-api.ts';

	interface Props {
		requests: PendingRequest[];
		ondecide: (requestId: string, admit: boolean) => void;
	}

	let { requests, ondecide }: Props = $props();
</script>

{#if requests.length > 0}
	<section class="flex flex-col gap-2" aria-live="polite">
		<h3 class="text-xs font-medium tracking-wide text-ink-muted uppercase">Waiting to join</h3>
		<ul class="flex flex-col gap-2">
			{#each requests as request (request.id)}
				<li class="flex items-center gap-2">
					<span class="flex-1 truncate">{request.name}</span>
					<Button
						size="sm"
						variant="ghost"
						onclick={() => {
							ondecide(request.id, false);
						}}
					>
						Deny
					</Button>
					<Button
						size="sm"
						onclick={() => {
							ondecide(request.id, true);
						}}
					>
						Admit
					</Button>
				</li>
			{/each}
		</ul>
	</section>
{/if}
