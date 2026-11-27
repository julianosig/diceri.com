<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { LoaderCircle, RotateCw } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import { currentAccount } from '#lib/features/account/index.ts';
	import { SiteHeader } from '#lib/features/landing/index.ts';
	import { MeetingFlow } from '#lib/features/meeting/index.ts';
	import { limitText } from '#lib/features/plans/index.ts';
	import { openMyRoom, type Room } from '#lib/features/room/index.ts';
	import { i18n } from '#lib/i18n/index.ts';

	/**
	 * `/open`: opening your own room and hosting its call. Joining someone else's room is
	 * `/@username`; signing in is `/signin`, which sends you back here.
	 */
	let opened = $state<{ room: Room; hostKey: string; username: string }>();
	let problem = $state<string>();
	let pending = $state(true);

	async function open(): Promise<void> {
		pending = true;
		problem = undefined;
		try {
			const account = await currentAccount();
			if (!account) {
				await goto(resolve(`/signin?next=${encodeURIComponent('/open')}`), { replace: true });
				return;
			}
			opened = { ...(await openMyRoom(account.username)), username: account.username };
		} catch (cause) {
			problem = limitText(cause, i18n.t) ?? i18n.t.openRoom.failed;
		} finally {
			pending = false;
		}
	}

	onMount(() => {
		void open();
	});
</script>

<svelte:head>
	<title>{i18n.t.openRoom.submit} · Diceri</title>
</svelte:head>

{#if opened}
	{#key opened.room.code}
		<MeetingFlow
			room={opened.room}
			hostKey={opened.hostKey}
			meetingUrl={`${page.url.origin}${resolve('/@[username]', { username: opened.username })}`}
			oncall={() => {
				// In the call, the address shows the room you share; reloading comes back here.
				if (!opened) return;
				const room = resolve('/@[username]', { username: opened.username });
				void goto(room, { shallow: true, replace: true, state: {} });
			}}
		>
			{#snippet header()}<SiteHeader />{/snippet}
		</MeetingFlow>
	{/key}
{:else}
	<SiteHeader />
	<main class="flex flex-col items-center gap-4 px-4 pt-16 pb-16 text-center">
		{#if pending}
			<LoaderCircle class="size-8 animate-spin text-ink-muted" aria-hidden="true" />
			<p class="text-ink-muted" role="status">{i18n.t.openRoom.submitting}</p>
		{:else if problem}
			<p role="alert" class="max-w-md text-red-600">{problem}</p>
			<Button size="lg" variant="secondary" onclick={() => void open()}>
				<RotateCw class="size-5" aria-hidden="true" />
				{i18n.t.roomOffline.tryAgain}
			</Button>
		{/if}
	</main>
{/if}
