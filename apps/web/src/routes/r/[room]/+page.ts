import { error } from '@sveltejs/kit';
import { ApiError } from '#lib/api/http-client.ts';
import { getRoom } from '#lib/features/room/index.ts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	try {
		return { room: await getRoom(params.room, fetch) };
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 404) {
			error(404, 'This meeting does not exist or has ended.');
		}
		throw cause;
	}
};
