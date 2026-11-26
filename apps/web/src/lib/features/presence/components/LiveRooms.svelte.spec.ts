import { page } from 'vitest/browser';
import { describe, expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import LiveRooms from './LiveRooms.svelte';

describe('LiveRooms', () => {
	it('shows who is online and ranks rooms without links or names', async () => {
		await render(LiveRooms, {
			snapshot: {
				online: 7,
				liveRooms: 2,
				ranking: [
					{ people: 5, languages: ['pt-BR', 'en'] },
					{ people: 2, languages: ['es'] }
				]
			}
		});

		await expect.element(page.getByRole('heading', { name: '7 online now' })).toBeVisible();
		await expect.element(page.getByText('2 live rooms')).toBeVisible();
		const rows = page.getByRole('listitem');
		await expect
			.element(rows.nth(0))
			.toHaveAccessibleName('#1: 5 people, hearing Português (Brasil), English');
		await expect.element(rows.nth(1)).toHaveAccessibleName('#2: 2 people, hearing Español');
		expect(page.getByRole('link').all()).toHaveLength(0);
	});

	it('invites to start the first room when nobody is online', async () => {
		await render(LiveRooms, { snapshot: { online: 0, liveRooms: 0, ranking: [] } });
		await expect.element(page.getByText('Quiet right now', { exact: false })).toBeVisible();
	});
});
