<script lang="ts">
	import { resolve } from '$app/paths';
	import { House, RotateCw, Sparkles, Video } from '@lucide/svelte';
	import { buttonVariants } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import MascotAvatar from './MascotAvatar.svelte';

	interface Props {
		/** The handle from the address (`/@username`), lowercase. */
		username: string;
		/** The owner's mascot, or `null` when nobody has claimed this username yet. */
		mascot: string | null;
		/** Looks again: the owner may have just opened the room. */
		onretry: () => void;
	}

	let { username, mascot, onretry }: Props = $props();
	const t = $derived(i18n.t.roomOffline);
	const claimed = $derived(mascot !== null);
	const home = resolve('/');
</script>

<!--
	What `/@username` shows while there is no live room: the room's own mascot says so in
	a speech bubble, and the way forward is your own room. An unclaimed name is offered
	instead (a ghost: nobody lives here yet).
-->
<section class="flex w-full max-w-xl flex-col items-center gap-6 text-center">
	<div class="flex flex-col items-center">
		<p
			class="pop relative mb-5 max-w-full rounded-2xl border-2 border-ink bg-surface px-6 py-4 text-2xl font-bold text-balance shadow-[0_3px_0_var(--color-ink)] sm:text-3xl"
			role="status"
		>
			{claimed ? t.liveTitle(username) : t.freeTitle(username)}
			<span
				class="absolute -bottom-[11px] left-1/2 size-5 -translate-x-1/2 rotate-45 border-r-2 border-b-2 border-ink bg-surface"
				aria-hidden="true"
			></span>
		</p>
		<MascotAvatar mascot={mascot ?? 'ghost'} size={176} decorative class="float" />
	</div>
	<p class="max-w-md text-lg text-pretty text-ink-muted">
		{claimed ? t.liveBody(username) : t.freeBody}
	</p>
	<div class="flex w-full flex-col items-center justify-center gap-3 sm:flex-row">
		{#if claimed}
			<a class={buttonVariants({ size: 'lg' })} href={home}>
				<Video class="size-5" aria-hidden="true" />
				{t.createOwn}
			</a>
			<button
				type="button"
				class={buttonVariants({ size: 'lg', variant: 'ghost' })}
				onclick={onretry}
			>
				<RotateCw class="size-5" aria-hidden="true" />
				{t.tryAgain}
			</button>
		{:else}
			<a class={buttonVariants({ size: 'lg' })} href="{home}?claim={encodeURIComponent(username)}">
				<Sparkles class="size-5" aria-hidden="true" />
				{t.claim(username)}
			</a>
			<a class={buttonVariants({ size: 'lg', variant: 'ghost' })} href={home}>
				<House class="size-5" aria-hidden="true" />
				{t.home}
			</a>
		{/if}
	</div>
</section>

<style>
	/* Bubble pops in, the mascot keeps floating: Tailwind has no such keyframes. */
	.pop {
		transform-origin: bottom center;
		animation: pop 0.45s cubic-bezier(0.34, 1.56, 0.64, 1) both;
	}

	:global(.float) {
		animation: float 3.2s ease-in-out infinite;
	}

	@keyframes pop {
		from {
			opacity: 0;
			scale: 0.7;
		}
		to {
			opacity: 1;
			scale: 1;
		}
	}

	@keyframes float {
		50% {
			translate: 0 -8px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.pop,
		:global(.float) {
			animation: none;
		}
	}
</style>
