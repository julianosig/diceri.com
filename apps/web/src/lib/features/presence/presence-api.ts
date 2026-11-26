import { requestJson } from '#lib/api/http-client.ts';
import type { Presence } from './types.ts';

export function getPresence(fetchFn?: typeof fetch): Promise<Presence> {
	return requestJson<Presence>('/presence', { fetchFn });
}
