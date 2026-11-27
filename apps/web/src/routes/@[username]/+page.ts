import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { ApiError } from '#lib/api/http-client.ts';
import { currentAccount } from '#lib/features/account/index.ts';
import { getUserProfile, getUserRoom } from '#lib/features/room/index.ts';
import type { PageLoad } from './$types';

/**
 * `/@username` is for joining someone's room. Its owner is sent to `/open` (hosting
 * lives there), and a signed-out visitor to a live room signs in first. Otherwise: the
 * live room, or — while there is none — who the address belongs to (if anyone).
 */
export const load: PageLoad = async ({ params, fetch }) => {
	const username = params.username.toLowerCase();
	const account = await currentAccount(fetch).catch(() => null);
	if (account?.username === username) redirect(307, resolve('/open'));
	try {
		const room = await getUserRoom(username, fetch);
		// A room is live for visitors once someone is in its call (`people` unknown: live).
		if (room.people !== 0 && !account) {
			redirect(307, resolve(`/signin?next=${encodeURIComponent(`/@${username}`)}`));
		}
		return { username, room, profile: null };
	} catch (cause) {
		if (!(cause instanceof ApiError && cause.status === 404)) throw cause;
		return { username, room: null, profile: await getUserProfile(username, fetch) };
	}
};
