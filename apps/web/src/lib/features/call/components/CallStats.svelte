<script lang="ts">
	import type { TransportMetrics } from '../transport-stats.ts';
	import type { CallMetrics } from '../stats-poller.ts';

	let { metrics }: { metrics: CallMetrics } = $props();

	function format(value: number | undefined, unit: string): string {
		return value === undefined ? '—' : `${String(value)} ${unit}`;
	}

	const rows = $derived(
		(
			[
				['Sending', metrics.send],
				['Receiving', metrics.receive]
			] as const
		).map(([label, m]: readonly [string, TransportMetrics | undefined]) => ({ label, m }))
	);
</script>

<table class="w-full text-left text-sm">
	<thead class="text-xs text-ink-muted">
		<tr>
			<th class="font-normal"></th>
			<th class="font-normal">Bitrate</th>
			<th class="font-normal">Loss</th>
			<th class="font-normal">Jitter</th>
			<th class="font-normal">Round trip</th>
		</tr>
	</thead>
	<tbody class="tabular-nums">
		{#each rows as row (row.label)}
			<tr>
				<th class="py-1 font-medium">{row.label}</th>
				<td>{format(row.m?.bitrateKbps, 'kbps')}</td>
				<td>{format(row.m?.lossPercent, '%')}</td>
				<td>{format(row.m?.jitterMs, 'ms')}</td>
				<td>{format(row.m?.roundTripMs, 'ms')}</td>
			</tr>
		{/each}
	</tbody>
</table>
