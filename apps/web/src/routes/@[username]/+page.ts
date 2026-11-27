import { ApiError } from '#lib/api/http-client.ts';
import { currentAccount } from '#lib/features/account/index.ts';
import { getUserProfile, getUserRoom } from '#lib/features/room/index.ts';
import type { PageLoad } from './$types';

/**
 * The live room, or — while there is none — who the address belongs to (if anyone), whether
 * the visitor is signed in (only accounts may join) and whether they own the room (only the
 * owner may act as host).
 */
export const load: PageLoad = async ({ params, fetch }) => {
	const username = params.username.toLowerCase();
	const account = currentAccount(fetch).catch(() => null);
	const owner = account.then((signedIn) => signedIn?.username === username);
	const signedIn = account.then((signedIn) => signedIn !== null);
	try {
		const room = await getUserRoom(username, fetch);
		return { username, room, profile: null, isOwner: await owner, signedIn: await signedIn };
	} catch (cause) {
		if (!(cause instanceof ApiError && cause.status === 404)) throw cause;
		const profile = await getUserProfile(username, fetch);
		return { username, room: null, profile, isOwner: await owner, signedIn: await signedIn };
	}
};
