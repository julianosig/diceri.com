/**
 * Room symbols: original animated mascots (`static/mascots/<id>.gif`). Mirrors
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

export function mascotUrl(id: string): string {
	return `/mascots/${encodeURIComponent(id)}.gif`;
}

export function mascotLabel(id: string): string {
	return MASCOTS.find((mascot) => mascot.id === id)?.label ?? 'Room';
}

export function randomMascot(random: () => number = Math.random): MascotId {
	const index = Math.floor(random() * MASCOTS.length) % MASCOTS.length;
	return (MASCOTS[index] ?? MASCOTS[0]).id;
}
