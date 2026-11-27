<script lang="ts">
	import { i18n } from '#lib/i18n/index.ts';
	import { onMount, type Snippet } from 'svelte';
	import { resolve } from '$app/paths';
	import { Button } from '#lib/components/ui/index.ts';
	import { CallScreen, MeetingInfoCard, TimeLeft } from '#lib/features/call/index.ts';
	import { TranscriptView } from '#lib/features/translation/index.ts';
	import {
		JoinRequestList,
		JoinRequestToast,
		PreJoin,
		WaitingScreen,
		type Profile
	} from '#lib/features/lobby/index.ts';
	import type { Room } from '#lib/features/room/index.ts';
	import { MeetingFlow } from '../meeting-flow.svelte.ts';

	interface Props {
		room: Room;
		hostKey: string | undefined;
		meetingUrl: string;
		/** The site header, shown around the call (pre-join, waiting, end) but not in it. */
		header?: Snippet;
	}

	let { room, hostKey, meetingUrl, header }: Props = $props();
	// Every room API path and the visible meeting code use the link code.
	// svelte-ignore state_referenced_locally
	const roomId = room.code;

	// One flow per page visit; the route remounts this component for another room.
	// svelte-ignore state_referenced_locally
	const flow = new MeetingFlow(roomId, hostKey, room.minutesLeft);
	let showInfo = $state(true);
	const requests = $derived(flow.hostRequests);

	onMount(() => {
		return () => {
			void flow.leave();
		};
	});
</script>

{#if flow.phase === 'call' && flow.session}
	<CallScreen
		session={flow.session}
		meetingCode={roomId}
		peopleBadge={requests?.pending.length ?? 0}
		onleave={() => flow.leave()}
		onendforeveryone={flow.isHost ? () => flow.endForEveryone() : undefined}
	>
		{#snippet peopleExtra()}
			{#if requests}
				<JoinRequestList
					requests={requests.pending}
					ondecide={(id: string, admit: boolean) => requests.decide(id, admit)}
				/>
			{/if}
		{/snippet}
		{#snippet overlay()}
			{#if flow.deadline}
				<TimeLeft deadline={flow.deadline} />
			{/if}
			{#if requests?.toast}
				{@const request = requests.toast}
				<JoinRequestToast
					{request}
					ondecide={(admit: boolean) => requests.decide(request.id, admit)}
					ondismiss={() => {
						requests.dismiss(request.id);
					}}
				/>
			{/if}
			{#if flow.isHost && showInfo}
				<MeetingInfoCard url={meetingUrl} onclose={() => (showInfo = false)} />
			{/if}
		{/snippet}
	</CallScreen>
{:else}
	{@render header?.()}
	<main class={['flex flex-col items-center py-10', header ? 'pt-6' : 'min-h-dvh justify-center']}>
		{#if flow.phase === 'waiting'}
			<WaitingScreen
				status={flow.knock.status}
				error={flow.knock.error}
				oncancel={() => {
					flow.cancelKnock();
				}}
			/>
		{:else if flow.phase === 'left' || flow.phase === 'ended'}
			<section class="flex w-full flex-col items-center gap-6 px-4 text-center">
				<h1 class="text-3xl">
					{#if flow.phase === 'left'}{i18n.t.meeting.left}{:else if flow.endedBy === 'limit'}{i18n.t
							.limits.timeUp}{:else if flow.endedBy === 'me'}{i18n.t.meeting.endedByMe}{:else}{i18n
							.t.meeting.endedByHost}{/if}
				</h1>
				{#if flow.endedBy === 'limit'}
					<p class="max-w-md text-ink-muted">{i18n.t.limits.timeUpHint}</p>
				{/if}
				<div class="flex gap-3">
					{#if flow.phase === 'left'}
						<Button variant="secondary" onclick={() => (flow.phase = 'prejoin')}>
							{i18n.t.meeting.rejoin}
						</Button>
					{/if}
					<a
						class="rounded-full px-6 py-2.5 text-brand-600 hover:bg-surface-raised"
						href={resolve('/')}
					>
						{i18n.t.meeting.home}
					</a>
				</div>
				{#if flow.transcript}
					<TranscriptView transcript={flow.transcript} />
				{/if}
			</section>
		{:else}
			<PreJoin
				{roomId}
				entry={room.entry}
				isHost={flow.isHost}
				busy={flow.phase === 'joining'}
				error={flow.error}
				onjoin={(profile: Profile, deviceId: string | undefined) => flow.join(profile, deviceId)}
			/>
		{/if}
	</main>
{/if}
