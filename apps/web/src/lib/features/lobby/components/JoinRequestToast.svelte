<script lang="ts">
	import { X } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
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
			<strong class="font-semibold">{i18n.t.lobby.wantsToJoin(request.name)}</strong>
			<span class="text-sm text-ink-muted">
				{i18n.t.lobby.hears(languageLabel(request.outputLanguage))}
			</span>
		</p>
		<button
			type="button"
			class="-m-1 rounded-full p-1 hover:bg-surface-raised"
			aria-label={i18n.t.lobby.dismiss}
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
			{i18n.t.lobby.deny}
		</Button>
		<Button
			size="sm"
			onclick={() => {
				ondecide(true);
			}}
		>
			{i18n.t.lobby.admit}
		</Button>
	</div>
</div>
