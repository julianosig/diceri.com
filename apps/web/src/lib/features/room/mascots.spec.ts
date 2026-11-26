import { describe, expect, it } from 'vitest';
import { MASCOTS, mascotLabel, mascotUrl, randomMascot } from './mascots.ts';

describe('mascots', () => {
	it('has the twelve mascots the server accepts', () => {
		expect(MASCOTS.map((m) => m.id).sort()).toEqual([
			'alien',
			'astronaut',
			'cybercat',
			'dragon',
			'gamer',
			'ghost',
			'knight',
			'ninja',
			'pirate',
			'robot',
			'slime',
			'wizard'
		]);
	});

	it('builds asset URLs and labels', () => {
		expect(mascotUrl('robot')).toBe('/mascots/robot.gif');
		expect(mascotLabel('cybercat')).toBe('Cyber cat');
		expect(mascotLabel('unknown')).toBe('Room');
	});

	it('picks a mascot from the catalog', () => {
		expect(randomMascot(() => 0)).toBe('wizard');
		expect(randomMascot(() => 0.999)).toBe('gamer');
	});
});
