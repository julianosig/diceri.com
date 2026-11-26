<script lang="ts">
	import { AudioLines, Mic, Sparkles, UserRound, VolumeX } from '@lucide/svelte';
	import { SegmentedControl, Toggle } from '#lib/components/ui/index.ts';
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

	const filters = [
		{ value: 'off', label: 'Off', icon: VolumeX, hint: 'Browser processing only.' },
		{
			value: 'noise',
			label: 'Noise',
			icon: Sparkles,
			hint: 'Removes background noise, keeps all voices.'
		},
		{
			value: 'voice',
			label: 'My voice only',
			icon: UserRound,
			hint: 'Also removes other people talking nearby (billed separately).'
		}
	] as const satisfies readonly {
		value: EnhancedFilter;
		label: string;
		icon: unknown;
		hint: string;
	}[];
	const enhanced = $derived(settings.enhancedFilter !== 'off');
</script>

<SettingsSection title="Microphone" icon={Mic}>
	<select
		class="w-full select-field rounded-lg bg-surface-raised py-2 ps-3 text-sm"
		aria-label="Microphone"
		value={settings.deviceId ?? ''}
		onchange={(event) => {
			onchange({ deviceId: event.currentTarget.value || undefined });
		}}
	>
		<option value="">System default</option>
		{#each devices as device (device.deviceId)}
			<option value={device.deviceId}>{device.label || 'Microphone'}</option>
		{/each}
	</select>
	<AudioMeter {level} {muted} />
</SettingsSection>

<SettingsSection title="Noise and echo" icon={AudioLines}>
	<div class="flex flex-col gap-2">
		<p class="text-sm font-medium">AI noise filter (Krisp)</p>
		<SegmentedControl
			label="AI noise filter (Krisp)"
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
			label="Echo cancellation"
			description="Stops the other person's voice from coming back through your mic."
			checked={settings.echoCancellation}
			onchange={(echoCancellation: boolean) => {
				onchange({ echoCancellation });
			}}
		/>
		<Toggle
			class="px-4 py-3"
			label="Browser noise suppression"
			description={enhanced ? 'Off while the AI filter is on.' : 'Basic built-in filter.'}
			checked={settings.noiseSuppression && !enhanced}
			disabled={enhanced}
			onchange={(noiseSuppression: boolean) => {
				onchange({ noiseSuppression });
			}}
		/>
		<Toggle
			class="px-4 py-3"
			label="Auto gain"
			description="Keeps your volume steady."
			checked={settings.autoGainControl}
			onchange={(autoGainControl: boolean) => {
				onchange({ autoGainControl });
			}}
		/>
	</div>
</SettingsSection>
