import { ApiError } from '#lib/api/http-client.ts';
import { currentAccount } from '#lib/features/account/index.ts';
import { getUserProfile, getUserRoom } from '#lib/features/room/index.ts';
import type { PageLoad } from './$types';

/**
 * The live room, or — while there is none — who the address belongs to (if anyone), and
 * whether the visitor is its signed-in owner (only the owner may act as host).
 */
export const load: PageLoad = async ({ params, fetch }) => {
	const username = params.username.toLowerCase();
	const owner = currentAccount(fetch)
		.then((account) => account?.username === username)
		.catch(() => false);
	try {
		const room = await getUserRoom(username, fetch);
		return { username, room, profile: null, isOwner: await owner };
	} catch (cause) {
		if (!(cause instanceof ApiError && cause.status === 404)) throw cause;
		const profile = await getUserProfile(username, fetch);
		return { username, room: null, profile, isOwner: await owner };
	}
};
