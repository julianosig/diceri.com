<script lang="ts">
	import { Ear, Headphones, Languages, Server, VolumeX } from '@lucide/svelte';
	import { SegmentedControl } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
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

	const t = $derived(i18n.t.audio);
	const modes = $derived([
		{ value: 'off' as MonitorMode, label: t.monitorOff, icon: VolumeX, hint: t.monitorOffHint },
		{ value: 'local' as MonitorMode, label: t.local, icon: Ear, hint: t.localHint },
		{ value: 'loopback' as MonitorMode, label: t.viaServer, icon: Server, hint: t.viaServerHint },
		{
			value: 'translated' as MonitorMode,
			label: t.translated,
			icon: Languages,
			hint: t.translatedHint
		}
	]);
</script>

<SettingsSection title={t.hearMyself} icon={Headphones}>
	<p class="-mt-1 text-xs text-ink-muted">{t.headphones}</p>
	<SegmentedControl
		label={t.hearMyself}
		options={modes}
		value={mode}
		onchange={(next: MonitorMode) => {
			onchange(next);
		}}
	/>
	<p class="text-xs text-ink-muted">{modes.find((m) => m.value === mode)?.hint}</p>
	{#if mode === 'translated'}
		<label class="flex flex-col gap-2 text-sm font-medium">
			{t.translateMeInto}
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
			{t.sameLanguage(languageLabel(spokenLanguage))}
		</p>
	{/if}
</SettingsSection>
