<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '#lib/components/ui/index.ts';
	import type { AudioSettings } from '../audio-settings.ts';
	import { CallSession, type MonitorMode } from '../call-session.svelte.ts';
	import AudioMeter from './AudioMeter.svelte';
	import AudioSettingsForm from './AudioSettingsForm.svelte';
	import CallStats from './CallStats.svelte';
	import MonitorControls from './MonitorControls.svelte';
	import PeerList from './PeerList.svelte';

	let { roomId }: { roomId: string } = $props();

	// The session lives as long as this component; roomId never changes here.
	// svelte-ignore state_referenced_locally
	const session = new CallSession(roomId);

	onMount(() => {
		return () => {
			void session.disconnect();
		};
	});
</script>

<section class="flex w-full max-w-xl flex-col gap-6 rounded-tile bg-surface-raised/40 p-6">
	{#if session.status === 'connected'}
		<div class="flex items-center justify-between gap-3">
			<h2 class="text-lg font-semibold">Audio test</h2>
			<div class="flex gap-2">
				<Button
					size="sm"
					variant="secondary"
					aria-pressed={session.muted}
					onclick={() => session.toggleMute()}
				>
					{session.muted ? 'Unmute' : 'Mute'}
				</Button>
				<Button size="sm" variant="ghost" onclick={() => session.disconnect()}>Leave</Button>
			</div>
		</div>
		{#if session.audioBlocked}
			<Button onclick={() => session.startAudio()}>Click to enable audio</Button>
		{/if}
		<AudioMeter level={session.level} muted={session.muted} />
		<MonitorControls
			mode={session.monitor}
			onchange={(mode: MonitorMode) => session.setMonitor(mode)}
		/>
		<AudioSettingsForm
			settings={session.settings}
			devices={session.devices}
			onchange={(patch: Partial<AudioSettings>) => session.updateSettings(patch)}
		/>
		<PeerList peers={session.peers} ownQuality={session.ownQuality} />
		<CallStats metrics={session.metrics} />
	{:else}
		<h2 class="text-lg font-semibold">Check your audio</h2>
		<AudioSettingsForm
			settings={session.settings}
			devices={session.devices}
			onchange={(patch: Partial<AudioSettings>) => session.updateSettings(patch)}
		/>
		<Button
			disabled={session.status === 'connecting'}
			onclick={async () => {
				await session.loadDevices();
				await session.connect();
			}}
		>
			{session.status === 'connecting' ? 'Connecting…' : 'Join with microphone'}
		</Button>
	{/if}
	{#if session.error}
		<p role="alert" class="text-sm text-red-600">{session.error}</p>
	{/if}
</section>
