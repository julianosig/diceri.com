import { describe, expect, it } from 'vitest';
import { avatarHue, avatars, channelName, peopleLabel } from './ranking.ts';

const room = (people: number, languages: string[]) => ({ people, languages });

describe('ranking', () => {
	it('names a room after its base languages, like a channel', () => {
		expect(channelName(room(3, ['pt-BR', 'en', 'pt-PT']))).toBe('pt-en');
		expect(channelName(room(1, []))).toBe('lounge');
	});

	it('caps avatar dots and counts the rest', () => {
		expect(avatars(room(3, []))).toEqual({ shown: 3, hidden: 0 });
		expect(avatars(room(8, []))).toEqual({ shown: 5, hidden: 3 });
	});

	it('gives stable, distinct avatar hues', () => {
		const r = room(2, ['en']);
		expect(avatarHue(r, 0)).toBe(avatarHue(r, 0));
		expect(avatarHue(r, 0)).not.toBe(avatarHue(r, 1));
	});

	it('pluralizes people', () => {
		expect(peopleLabel(1)).toBe('1 person');
		expect(peopleLabel(4)).toBe('4 people');
	});
});
