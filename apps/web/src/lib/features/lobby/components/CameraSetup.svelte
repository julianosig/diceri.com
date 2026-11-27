<script lang="ts">
	import { Camera, Video, VideoOff } from '@lucide/svelte';
	import { i18n } from '#lib/i18n/index.ts';
	import type { CameraPreview } from '../camera-preview.svelte.ts';

	interface Props {
		camera: CameraPreview;
	}

	let { camera }: Props = $props();
	const t = $derived(i18n.t.lobby);
	let element = $state<HTMLVideoElement>();

	$effect(() => {
		if (element) element.srcObject = camera.stream ?? null;
	});
</script>

<!-- How you will look: a mirrored preview, the camera on/off switch and which camera. -->
<section class="flex w-full flex-col gap-2 text-start" aria-label={t.camera}>
	<div class="relative aspect-video w-full overflow-hidden rounded-tile bg-stage">
		{#if camera.stream}
			<video
				bind:this={element}
				class="size-full -scale-x-100 object-cover"
				autoplay
				playsinline
				muted
			></video>
		{:else}
			<div class="flex size-full items-center justify-center text-sm text-white/70">
				{#if camera.on && camera.error}{t[camera.error]}{:else}<VideoOff
						class="size-10"
						aria-hidden="true"
					/>{/if}
			</div>
		{/if}
		<button
			type="button"
			class={[
				'absolute bottom-3 left-1/2 flex size-11 -translate-x-1/2 items-center justify-center rounded-full text-white',
				camera.on ? 'bg-black/50 hover:bg-black/70' : 'bg-danger hover:bg-danger/90'
			]}
			aria-pressed={camera.on}
			aria-label={camera.on ? t.cameraOff : t.cameraOn}
			title={camera.on ? t.cameraOff : t.cameraOn}
			onclick={() => void camera.toggle()}
		>
			{#if camera.on}<Video class="size-5" aria-hidden="true" />{:else}<VideoOff
					class="size-5"
					aria-hidden="true"
				/>{/if}
		</button>
	</div>
	{#if camera.on && camera.devices.length > 1}
		<label class="text-sm">
			<span class="mb-1 block text-ink-muted">{t.camera}</span>
			<span
				class="flex items-center gap-2 rounded-lg border border-ink-muted/30 bg-surface ps-3 focus-within:border-brand-500"
			>
				<Camera class="size-5 shrink-0 text-ink-muted" aria-hidden="true" />
				<select
					class="w-full select-field truncate bg-transparent py-2 text-base outline-none"
					value={camera.deviceId ?? ''}
					onchange={(event) => void camera.start(event.currentTarget.value || undefined)}
				>
					{#each camera.devices as device, index (device.deviceId)}
						<option value={device.deviceId}>
							{device.label || `${t.camera} ${String(index + 1)}`}
						</option>
					{/each}
				</select>
			</span>
		</label>
	{/if}
</section>
