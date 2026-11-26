/**
 * Room mascots: original animated characters (`static/mascots/{webp,gif}/<id>.*`). Mirrors
 * `Mascot` in crates/domain/src/mascot.rs.
 */
export const MASCOTS = [
	{ id: 'wizard', label: 'Wizard' },
	{ id: 'robot', label: 'Robot' },
	{ id: 'astronaut', label: 'Astronaut' },
	{ id: 'alien', label: 'Alien' },
	{ id: 'ninja', label: 'Ninja' },
	{ id: 'dragon', label: 'Dragon' },
	{ id: 'knight', label: 'Knight' },
	{ id: 'ghost', label: 'Ghost' },
	{ id: 'cybercat', label: 'Cyber cat' },
	{ id: 'slime', label: 'Slime' },
	{ id: 'pirate', label: 'Space pirate' },
	{ id: 'gamer', label: 'Gamer' }
] as const;

export type MascotId = (typeof MASCOTS)[number]['id'];

/**
 * Transparent animated asset: WebP (preferred, 128 px or the native 250 px as `webp-2x`)
 * or GIF (fallback, 128 px).
 */
export function mascotUrl(id: string, format: 'webp' | 'webp-2x' | 'gif' = 'webp'): string {
	const extension = format === 'gif' ? 'gif' : 'webp';
	return `/mascots/${format}/${encodeURIComponent(id)}.${extension}`;
}

/** Pixel widths of the WebP files, for `srcset`. */
export const MASCOT_WEBP_WIDTHS = { webp: 128, 'webp-2x': 250 } as const;

export function mascotLabel(id: string): string {
	return MASCOTS.find((mascot) => mascot.id === id)?.label ?? 'Room';
}

export function randomMascot(random: () => number = Math.random): MascotId {
	const index = Math.floor(random() * MASCOTS.length) % MASCOTS.length;
	return (MASCOTS[index] ?? MASCOTS[0]).id;
}
