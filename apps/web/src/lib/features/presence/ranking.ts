import type { RankedRoom } from './types.ts';

/** Avatars drawn per room before collapsing the rest into "+N". */
export const MAX_AVATARS = 5;

function base(code: string): string {
	return (code.split('-')[0] ?? code).toLowerCase();
}

/** Channel-style name from the room's languages, e.g. `pt-en-es`. */
export function channelName(room: RankedRoom): string {
	const unique = [...new Set(room.languages.map(base))];
	return unique.length > 0 ? unique.join('-') : 'lounge';
}

/** Avatar dots for a room: one per person up to the cap, plus how many are hidden. */
export function avatars(room: RankedRoom): { shown: number; hidden: number } {
	const shown = Math.min(room.people, MAX_AVATARS);
	return { shown, hidden: room.people - shown };
}

/** Stable hue (0–359) for the n-th avatar of a room, so dots differ but never flicker. */
export function avatarHue(room: RankedRoom, index: number): number {
	const name = channelName(room);
	let seed = 0;
	for (let i = 0; i < name.length; i++) seed += name.charCodeAt(i);
	return (seed * 37 + index * 67) % 360;
}

export function peopleLabel(count: number): string {
	return count === 1 ? '1 person' : `${String(count)} people`;
}
