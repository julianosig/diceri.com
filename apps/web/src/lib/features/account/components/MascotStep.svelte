<script lang="ts">
	import { resolve } from '$app/paths';
	import { ArrowLeft, LoaderCircle, UserPlus } from '@lucide/svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { MascotAvatar, MascotPicker, type MascotId } from '#lib/features/room/index.ts';
	import { i18n } from '#lib/i18n/index.ts';

	interface Props {
		/** The claimed @, shown with the chosen mascot. */
		handle: string;
		value: MascotId;
		pending: boolean;
		error: string | null;
		onchange: (mascot: MascotId) => void;
		onback: () => void;
		oncreate: () => void;
	}

	let { handle, value, pending, error, onchange, onback, oncreate }: Props = $props();
	const t = $derived(i18n.t.onboarding);
</script>

<!-- Sign-up's last step: only the mascot, big, with a random one already chosen. -->
<form
	class="flex w-full max-w-lg flex-col items-center gap-6 text-center"
	onsubmit={(event) => {
		event.preventDefault();
		oncreate();
	}}
>
	<div class="flex flex-col items-center gap-2">
		<MascotAvatar mascot={value} size={128} />
		<h1 class="text-3xl font-semibold tracking-tight">{t.mascotTitle}</h1>
		<p class="text-ink-muted">@{handle} · {t.mascotHint}</p>
	</div>
	<MascotPicker {value} {onchange} variant="large" />
	<!-- Creating the account accepts the terms, which cover keeping the voice (ADR 0015). -->
	<p class="max-w-md text-sm text-ink-muted">
		{i18n.t.terms.notice}
		<a class="font-medium text-brand-600 underline" href={resolve('/terms')} target="_blank">
			{i18n.t.terms.read}
		</a>
	</p>
	<div class="flex w-full flex-col-reverse items-center justify-center gap-3 sm:flex-row">
		<Button variant="secondary" size="lg" type="button" disabled={pending} onclick={onback}>
			<ArrowLeft class="size-5 rtl:rotate-180" aria-hidden="true" />
			{t.back}
		</Button>
		<Button size="lg" type="submit" disabled={pending}>
			{#if pending}
				<LoaderCircle class="size-5 animate-spin" aria-hidden="true" />
				{t.creating}
			{:else}
				<UserPlus class="size-5" aria-hidden="true" />
				{t.continue}
			{/if}
		</Button>
	</div>
	{#if error}<p role="alert" class="text-sm font-medium text-red-600">{error}</p>{/if}
</form>
