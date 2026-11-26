<script lang="ts">
	import { AudioLines, Mic, Sparkles, UserRound, VolumeX } from '@lucide/svelte';
	import { SegmentedControl, Toggle } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import type { AudioSettings, EnhancedFilter } from '../audio-settings.ts';
	import AudioMeter from './AudioMeter.svelte';
	import SettingsSection from './SettingsSection.svelte';

	interface Props {
		settings: AudioSettings;
		devices: MediaDeviceInfo[];
		/** Outgoing level, after the filters, for the meter. */
		level: number;
		muted: boolean;
		onchange: (patch: Partial<AudioSettings>) => void;
	}

	let { settings, devices, level, muted, onchange }: Props = $props();

	const t = $derived(i18n.t.audio);
	const filters = $derived([
		{ value: 'off' as EnhancedFilter, label: t.filterOff, icon: VolumeX, hint: t.filterOffHint },
		{
			value: 'noise' as EnhancedFilter,
			label: t.filterNoise,
			icon: Sparkles,
			hint: t.filterNoiseHint
		},
		{
			value: 'voice' as EnhancedFilter,
			label: t.filterVoice,
			icon: UserRound,
			hint: t.filterVoiceHint
		}
	]);
	const enhanced = $derived(settings.enhancedFilter !== 'off');
</script>

<SettingsSection title={t.microphone} icon={Mic}>
	<select
		class="w-full select-field rounded-lg bg-surface-raised py-2 ps-3 text-sm"
		aria-label={t.microphone}
		value={settings.deviceId ?? ''}
		onchange={(event) => {
			onchange({ deviceId: event.currentTarget.value || undefined });
		}}
	>
		<option value="">{t.systemDefault}</option>
		{#each devices as device (device.deviceId)}
			<option value={device.deviceId}>{device.label || t.microphone}</option>
		{/each}
	</select>
	<AudioMeter {level} {muted} />
</SettingsSection>

<SettingsSection title={t.noiseAndEcho} icon={AudioLines}>
	<div class="flex flex-col gap-2">
		<p class="text-sm font-medium">{t.aiFilter}</p>
		<SegmentedControl
			label={t.aiFilter}
			options={filters}
			value={settings.enhancedFilter}
			onchange={(enhancedFilter: EnhancedFilter) => {
				onchange({ enhancedFilter });
			}}
		/>
		<p class="text-xs text-ink-muted">
			{filters.find((f) => f.value === settings.enhancedFilter)?.hint}
		</p>
	</div>
	<div class="divide-y divide-ink-muted/15 rounded-xl border border-ink-muted/15">
		<Toggle
			class="px-4 py-3"
			label={t.echo}
			description={t.echoHint}
			checked={settings.echoCancellation}
			onchange={(echoCancellation: boolean) => {
				onchange({ echoCancellation });
			}}
		/>
		<Toggle
			class="px-4 py-3"
			label={t.suppression}
			description={enhanced ? t.suppressionOff : t.suppressionHint}
			checked={settings.noiseSuppression && !enhanced}
			disabled={enhanced}
			onchange={(noiseSuppression: boolean) => {
				onchange({ noiseSuppression });
			}}
		/>
		<Toggle
			class="px-4 py-3"
			label={t.autoGain}
			description={t.autoGainHint}
			checked={settings.autoGainControl}
			onchange={(autoGainControl: boolean) => {
				onchange({ autoGainControl });
			}}
		/>
	</div>
</SettingsSection>
