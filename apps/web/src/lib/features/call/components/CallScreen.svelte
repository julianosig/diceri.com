<script lang="ts">
	import type { Snippet } from 'svelte';
	import { CaptionsOverlay } from '#lib/features/translation/index.ts';
	import type { CallPanel } from '../call-layout.ts';
	import type { AudioSettings } from '../audio-settings.ts';
	import type { CallSession, MonitorMode } from '../call-session.svelte.ts';
	import AudioSettingsForm from './AudioSettingsForm.svelte';
	import CallStats from './CallStats.svelte';
	import ControlBar from './ControlBar.svelte';
	import MonitorControls from './MonitorControls.svelte';
	import ParticipantTile from './ParticipantTile.svelte';
	import SidePanel from './SidePanel.svelte';

	interface Props {
		session: CallSession;
		meetingCode: string;
		/** Badge on the People button (e.g. people waiting to join). */
		peopleBadge?: number;
		/** Extra content at the top of the People panel (host: join requests). */
		peopleExtra?: Snippet;
		/** Floating content over the stage (host: meeting card, join toasts). */
		overlay?: Snippet;
		onleave: () => void;
		/** Host only: ends the meeting for everyone. */
		onendforeveryone?: () => void;
	}

	let {
		session,
		meetingCode,
		peopleBadge = 0,
		peopleExtra,
		overlay,
		onleave,
		onendforeveryone
	}: Props = $props();
	let panel = $state<CallPanel>('none');
	const grid = $derived(session.participants.length > 1 ? 'sm:grid-cols-2' : 'max-w-4xl');
	const nameOf = (identity: string) =>
		session.participants.find((p) => p.identity === identity)?.name ?? 'Guest';
</script>

<div class="flex h-dvh flex-col bg-stage">
	{#if session.audioBlocked}
		<button
			type="button"
			class="bg-brand-500 px-4 py-2 text-sm text-white"
			onclick={() => session.startAudio()}
		>
			Click to enable audio from other participants
		</button>
	{/if}
	<div class="flex min-h-0 flex-1 gap-4 p-4">
		<main class="relative flex min-w-0 flex-1 items-center justify-center">
			<div class="grid w-full gap-4 {grid}">
				{#each session.participants as participant (participant.identity)}
					<ParticipantTile {participant} />
				{/each}
			</div>
			{#if session.status === 'connecting'}
				<p class="absolute text-white/80">Connecting…</p>
			{/if}
			{#if session.error}
				<p role="alert" class="absolute top-0 rounded-lg bg-danger px-4 py-2 text-sm text-white">
					{session.error}
				</p>
			{/if}
			<div class="pointer-events-none absolute inset-0 flex flex-col justify-end gap-3">
				<div class="pointer-events-auto flex flex-col items-start gap-3">
					{@render overlay?.()}
				</div>
				<CaptionsOverlay lines={session.captions.lines} {nameOf} />
			</div>
		</main>
		{#if panel === 'people'}
			<SidePanel title="People" onclose={() => (panel = 'none')}>
				{@render peopleExtra?.()}
				<section class="flex flex-col gap-2">
					<h3 class="text-xs font-medium tracking-wide text-ink-muted uppercase">In call</h3>
					<ul class="flex flex-col gap-2">
						{#each session.participants as participant (participant.identity)}
							<li class="flex items-center gap-3">
								<span
									class="flex size-8 items-center justify-center rounded-full bg-brand-600 text-sm text-white"
									aria-hidden="true"
								>
									{participant.name.charAt(0).toUpperCase()}
								</span>
								<span class="flex-1 truncate">
									{participant.name}{participant.isLocal ? ' (You)' : ''}
								</span>
								{#if participant.muted}<span class="text-xs text-ink-muted">muted</span>{/if}
							</li>
						{/each}
					</ul>
				</section>
			</SidePanel>
		{:else if panel === 'settings'}
			<SidePanel title="Audio settings" onclose={() => (panel = 'none')}>
				<AudioSettingsForm
					settings={session.settings}
					devices={session.devices}
					level={session.level}
					muted={session.muted}
					onchange={(patch: Partial<AudioSettings>) => session.updateSettings(patch)}
				/>
				<MonitorControls
					mode={session.monitor}
					previewLanguage={session.previewLanguage}
					spokenLanguage={session.ownSpokenLanguage}
					onchange={(mode: MonitorMode) => session.setMonitor(mode)}
					onpreviewchange={(language: string) => session.setPreviewLanguage(language)}
				/>
				<CallStats metrics={session.metrics} />
			</SidePanel>
		{/if}
	</div>
	<ControlBar
		{meetingCode}
		muted={session.muted}
		{panel}
		badge={peopleBadge}
		ontogglemute={() => session.toggleMute()}
		onpanel={(next: CallPanel) => (panel = next)}
		{onleave}
		{onendforeveryone}
	/>
</div>
