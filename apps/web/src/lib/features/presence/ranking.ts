import type { RankedRoom } from './types.ts';

/** Avatars drawn per room before collapsing the rest into "+N". */
export const MAX_AVATARS = 5;

function base(code: string): string {
	return (code.split('-')[0] ?? code).toLowerCase();
}

/** Compact language tag line from the room's languages, e.g. `pt · en · es`. */
export function languageTags(room: Pick<RankedRoom, 'languages'>): string {
	return [...new Set(room.languages.map(base))].join(' · ');
}

/** Avatar dots for a room: one per person up to the cap, plus how many are hidden. */
export function avatars(room: Pick<RankedRoom, 'people'>): { shown: number; hidden: number } {
	const shown = Math.min(room.people, MAX_AVATARS);
	return { shown, hidden: room.people - shown };
}

/** Stable hue (0–359) for the n-th avatar of a room, so dots differ but never flicker. */
export function avatarHue(room: Pick<RankedRoom, 'id'>, index: number): number {
	const name = room.id;
	let seed = 0;
	for (let i = 0; i < name.length; i++) seed += name.charCodeAt(i);
	return (seed * 37 + index * 67) % 360;
}

export function peopleLabel(count: number): string {
	return count === 1 ? '1 person' : `${String(count)} people`;
}
