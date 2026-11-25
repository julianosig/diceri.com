<script lang="ts">
	import { X } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { languageLabel } from '#lib/features/translation/index.ts';
	import type { PendingRequest } from '../lobby-api.ts';

	interface Props {
		request: PendingRequest;
		ondecide: (admit: boolean) => void;
		ondismiss: () => void;
	}

	let { request, ondecide, ondismiss }: Props = $props();
</script>

<div
	class="flex w-full max-w-sm flex-col gap-3 rounded-tile bg-surface p-4 text-ink shadow-xl"
	role="alertdialog"
	aria-labelledby="join-request-title"
>
	<header class="flex items-start justify-between gap-3">
		<p id="join-request-title">
			<strong>{request.name}</strong> wants to join this call
			<span class="text-sm text-ink-muted">({languageLabel(request.language)})</span>
		</p>
		<button
			type="button"
			class="-m-1 rounded-full p-1 hover:bg-surface-raised"
			aria-label="Dismiss"
			onclick={ondismiss}
		>
			<X class="size-4" />
		</button>
	</header>
	<div class="flex justify-end gap-2">
		<Button
			size="sm"
			variant="ghost"
			onclick={() => {
				ondecide(false);
			}}
		>
			Deny
		</Button>
		<Button
			size="sm"
			onclick={() => {
				ondecide(true);
			}}
		>
			Admit
		</Button>
	</div>
</div>
