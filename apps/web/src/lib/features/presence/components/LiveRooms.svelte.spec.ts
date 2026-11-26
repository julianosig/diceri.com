import { page } from 'vitest/browser';
import { describe, expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import type { RankedRoom } from '../types.ts';
import LiveRooms from './LiveRooms.svelte';

const open = {
	code: 'aaaaaaaaaaaaaaaaaaaaaa',
	owner: 'ana',
	entry: 'open',
	mascot: 'dragon',
	people: 5,
	languages: ['pt-BR', 'en']
} satisfies RankedRoom;
const approval = {
	code: 'bbbbbbbbbbbbbbbbbbbbbb',
	owner: 'bob',
	entry: 'approval',
	mascot: 'robot',
	people: 2,
	languages: ['es']
} satisfies RankedRoom;

describe('LiveRooms', () => {
	it('ranks listed rooms by owner and links to their @ room', async () => {
		await render(LiveRooms, {
			snapshot: { online: 9, liveRooms: 3, ranking: [open, approval] }
		});

		await expect.element(page.getByRole('heading', { name: '9 online now' })).toBeVisible();
		await expect.element(page.getByText('3 live rooms')).toBeVisible();
		const first = page.getByRole('link', {
			name: '#1 @ana: 5 people, hearing Português (Brasil), English. join now'
		});
		await expect.element(first).toHaveAttribute('href', '/@ana');
		await expect
			.element(page.getByRole('link', { name: /^#2 @bob: 2 people.*ask to join$/ }))
			.toBeVisible();
	});

	it('invites hosts to go public when nothing is listed', async () => {
		await render(LiveRooms, { snapshot: { online: 2, liveRooms: 1, ranking: [] } });
		await expect
			.element(page.getByText('No public rooms right now', { exact: false }))
			.toBeVisible();
	});
});
