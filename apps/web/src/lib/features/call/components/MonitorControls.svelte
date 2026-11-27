<script lang="ts">
	import { Ear, Headphones, Server, VolumeX } from '@lucide/svelte';
	import { SegmentedControl } from '#lib/components/ui/index.ts';
	import { i18n } from '#lib/i18n/index.ts';
	import type { MonitorMode } from '../call-session.svelte.ts';
	import SettingsSection from './SettingsSection.svelte';

	interface Props {
		mode: MonitorMode;
		onchange: (mode: MonitorMode) => void;
	}

	let { mode, onchange }: Props = $props();

	const t = $derived(i18n.t.audio);
	const modes = $derived([
		{ value: 'off' as MonitorMode, label: t.monitorOff, icon: VolumeX, hint: t.monitorOffHint },
		{ value: 'local' as MonitorMode, label: t.local, icon: Ear, hint: t.localHint },
		{ value: 'loopback' as MonitorMode, label: t.viaServer, icon: Server, hint: t.viaServerHint }
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
</SettingsSection>
