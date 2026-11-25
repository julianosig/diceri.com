<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { Button } from '#lib/components/ui/index.ts';
	import { CallScreen, MeetingInfoCard } from '#lib/features/call/index.ts';
	import {
		JoinRequestList,
		JoinRequestToast,
		PreJoin,
		WaitingScreen,
		type Profile
	} from '#lib/features/lobby/index.ts';
	import { MeetingFlow } from '../meeting-flow.svelte.ts';

	interface Props {
		roomId: string;
		hostKey: string | undefined;
		meetingUrl: string;
	}

	let { roomId, hostKey, meetingUrl }: Props = $props();

	// One flow per page visit; the route remounts this component for another room.
	// svelte-ignore state_referenced_locally
	const flow = new MeetingFlow(roomId, hostKey);
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
	<main class="flex min-h-dvh flex-col items-center justify-center py-10">
		{#if flow.phase === 'waiting'}
			<WaitingScreen
				status={flow.knock.status}
				error={flow.knock.error}
				oncancel={() => {
					flow.cancelKnock();
				}}
			/>
		{:else if flow.phase === 'left'}
			<section class="flex flex-col items-center gap-4 text-center">
				<h1 class="text-3xl">You left the meeting</h1>
				<div class="flex gap-3">
					<Button variant="secondary" onclick={() => (flow.phase = 'prejoin')}>Rejoin</Button>
					<a
						class="rounded-full px-6 py-2.5 text-brand-600 hover:bg-surface-raised"
						href={resolve('/')}
					>
						Return to home screen
					</a>
				</div>
			</section>
		{:else}
			<PreJoin
				isHost={flow.isHost}
				busy={flow.phase === 'joining'}
				error={flow.error}
				onjoin={(profile: Profile, deviceId: string | undefined) => flow.join(profile, deviceId)}
			/>
		{/if}
	</main>
{/if}
