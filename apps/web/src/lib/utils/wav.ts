/** Bytes in the canonical 44-byte header of a PCM WAV file. */
const HEADER_BYTES = 44;

/**
 * A 16-bit PCM mono WAV of `samples` (floats in −1…1, clipped) at `sampleRate`. Used to
 * send speech in the format speech-to-text reads, instead of a compressed recording.
 */
export function encodeWav(samples: Float32Array, sampleRate: number): Blob {
	const dataBytes = samples.length * 2;
	const view = new DataView(new ArrayBuffer(HEADER_BYTES + dataBytes));
	const text = (at: number, value: string) => {
		for (let i = 0; i < value.length; i++) view.setUint8(at + i, value.charCodeAt(i));
	};
	text(0, 'RIFF');
	view.setUint32(4, HEADER_BYTES - 8 + dataBytes, true);
	text(8, 'WAVEfmt ');
	view.setUint32(16, 16, true); // fmt chunk size
	view.setUint16(20, 1, true); // PCM
	view.setUint16(22, 1, true); // mono
	view.setUint32(24, sampleRate, true);
	view.setUint32(28, sampleRate * 2, true); // bytes per second
	view.setUint16(32, 2, true); // bytes per frame
	view.setUint16(34, 16, true); // bits per sample
	text(36, 'data');
	view.setUint32(40, dataBytes, true);
	samples.forEach((sample, i) => {
		const clipped = Math.max(-1, Math.min(1, sample));
		view.setInt16(HEADER_BYTES + i * 2, Math.round(clipped * 0x7fff), true);
	});
	return new Blob([view.buffer], { type: 'audio/wav' });
}
