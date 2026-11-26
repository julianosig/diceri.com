import { describe, expect, it } from 'vitest';
import { encodeWav } from './wav.ts';

async function bytes(blob: Blob): Promise<DataView> {
	return new DataView(await blob.arrayBuffer());
}

describe('encodeWav', () => {
	it('writes a 16 kHz mono 16-bit PCM header', async () => {
		const blob = encodeWav(new Float32Array(160), 16_000);
		const view = await bytes(blob);
		expect(blob.type).toBe('audio/wav');
		expect(view.byteLength).toBe(44 + 320);
		expect(String.fromCharCode(view.getUint8(0), view.getUint8(1), view.getUint8(2))).toBe('RIF');
		expect(view.getUint16(20, true)).toBe(1);
		expect(view.getUint16(22, true)).toBe(1);
		expect(view.getUint32(24, true)).toBe(16_000);
		expect(view.getUint16(34, true)).toBe(16);
		expect(view.getUint32(40, true)).toBe(320);
	});

	it('scales and clips samples to 16 bits', async () => {
		const view = await bytes(encodeWav(new Float32Array([0, 0.5, -1, 3]), 16_000));
		expect(view.getInt16(44, true)).toBe(0);
		expect(view.getInt16(46, true)).toBe(16_384);
		expect(view.getInt16(48, true)).toBe(-32_767);
		expect(view.getInt16(50, true)).toBe(32_767);
	});
});
