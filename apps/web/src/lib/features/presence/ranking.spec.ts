import { describe, expect, it } from 'vitest';
import { avatarHue, avatars, languageTags, peopleLabel } from './ranking.ts';

describe('ranking', () => {
	it('lists base languages once', () => {
		expect(languageTags({ languages: ['pt-BR', 'en', 'pt-PT'] })).toBe('pt · en');
		expect(languageTags({ languages: [] })).toBe('');
	});

	it('caps avatar dots and counts the rest', () => {
		expect(avatars({ people: 3 })).toEqual({ shown: 3, hidden: 0 });
		expect(avatars({ people: 8 })).toEqual({ shown: 5, hidden: 3 });
	});

	it('gives stable, distinct avatar hues per room', () => {
		const room = { id: 'abcdefghijklmnopqrstuv' };
		expect(avatarHue(room, 0)).toBe(avatarHue(room, 0));
		expect(avatarHue(room, 0)).not.toBe(avatarHue(room, 1));
	});

	it('pluralizes people', () => {
		expect(peopleLabel(1)).toBe('1 person');
		expect(peopleLabel(4)).toBe('4 people');
	});
});
