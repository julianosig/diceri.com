import { error } from '@sveltejs/kit';
import { ApiError } from '#lib/api/http-client.ts';
import { getUserRoom } from '#lib/features/room/index.ts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	try {
		return { room: await getUserRoom(params.username, fetch) };
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 404) {
			error(404, `@${params.username.toLowerCase()} is not live right now. Try again soon.`);
		}
		throw cause;
	}
};
