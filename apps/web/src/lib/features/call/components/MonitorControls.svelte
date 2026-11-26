<script lang="ts">
	import { Ear, Headphones, Languages, Server, VolumeX } from '@lucide/svelte';
	import { SegmentedControl } from '#lib/components/ui/index.ts';
	import { LANGUAGES, languageLabel, sameLanguage } from '#lib/features/translation/index.ts';
	import type { MonitorMode } from '../call-session.svelte.ts';
	import SettingsSection from './SettingsSection.svelte';

	interface Props {
		mode: MonitorMode;
		previewLanguage: string;
		/** Language your speech was last detected in, if any. */
		spokenLanguage: string | undefined;
		onchange: (mode: MonitorMode) => void;
		onpreviewchange: (language: string) => void;
	}

	let { mode, previewLanguage, spokenLanguage, onchange, onpreviewchange }: Props = $props();
	const sameAsSpoken = $derived(
		mode === 'translated' && sameLanguage(spokenLanguage, previewLanguage)
	);

	const modes = [
		{ value: 'off', label: 'Off', icon: VolumeX, hint: 'You do not hear yourself.' },
		{
			value: 'local',
			label: 'Local',
			icon: Ear,
			hint: 'Hear your mic after the noise filter, instantly. Judges filter quality.'
		},
		{
			value: 'loopback',
			label: 'Via server',
			icon: Server,
			hint: 'Hear yourself after a round trip through the media server. Judges transmission.'
		},
		{
			value: 'translated',
			label: 'Translated',
			icon: Languages,
			hint: 'Hear your own voice translated by the agent. Speak, pause, and listen.'
		}
	] as const satisfies readonly {
		value: MonitorMode;
		label: string;
		icon: unknown;
		hint: string;
	}[];
</script>

<SettingsSection title="Hear myself" icon={Headphones}>
	<p class="-mt-1 text-xs text-ink-muted">Use headphones, or the room hears you twice.</p>
	<SegmentedControl
		label="Hear myself"
		options={modes}
		value={mode}
		onchange={(next: MonitorMode) => {
			onchange(next);
		}}
	/>
	<p class="text-xs text-ink-muted">{modes.find((m) => m.value === mode)?.hint}</p>
	{#if mode === 'translated'}
		<label class="flex flex-col gap-2 text-sm font-medium">
			Translate me into
			<select
				class="w-full select-field rounded-lg bg-surface-raised py-2 ps-3 text-sm font-normal"
				value={previewLanguage}
				onchange={(event) => {
					onpreviewchange(event.currentTarget.value);
				}}
			>
				{#each LANGUAGES as option (option.code)}
					<option value={option.code}>{option.label}</option>
				{/each}
			</select>
		</label>
	{/if}
	{#if sameAsSpoken && spokenLanguage}
		<p role="status" class="rounded-lg bg-amber-100 px-3 py-2 text-xs text-amber-900">
			You're speaking {languageLabel(spokenLanguage)}, the language you picked, so there is nothing
			to translate. Pick another language to hear yourself translated.
		</p>
	{/if}
</SettingsSection>
