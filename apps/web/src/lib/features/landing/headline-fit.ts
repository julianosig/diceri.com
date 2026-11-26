/** Average advance of a Latin, Cyrillic, Arabic or Devanagari glyph, in em (Inter: ~0.42). */
const NARROW_EM = 0.47;
/** CJK ideographs, kana and hangul are a full em wide. */
const WIDE_EM = 1;
const WIDE = /[ᄀ-ᇿ⺀-鿿가-힯豈-﫿＀-￯]/u;

/** Approximate width of one line of text, in em. */
export function lineWidthEm(text: string): number {
	let width = 0;
	for (const char of text) width += WIDE.test(char) ? WIDE_EM : NARROW_EM;
	return width;
}

/**
 * A CSS font size that fits the longest of `lines` on one line each: as large as
 * `maxPx`, otherwise shrinking with the viewport (minus the page gutters).
 */
export function fitFontSize(lines: readonly string[], maxPx: number, maxWidthPx: number): string {
	const widest = Math.max(1, ...lines.map(lineWidthEm));
	const scale = (1 / widest).toFixed(4);
	return `min(${String(maxPx)}px, calc((min(100vw, ${String(maxWidthPx)}px) - 2rem) * ${scale}))`;
}
