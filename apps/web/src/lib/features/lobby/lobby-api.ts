import { requestJson, requestVoid } from '#lib/api/http-client.ts';
import type { MediaCredentials } from '#lib/features/room/index.ts';

const room = (id: string) => `/rooms/${encodeURIComponent(id)}`;

export interface Knock {
	requestId: string;
	requestKey: string;
}

export type KnockOutcome =
	| { status: 'pending' }
	| { status: 'denied' }
	| { status: 'admitted'; credentials: MediaCredentials };

export interface PendingRequest {
	id: string;
	name: string;
}

export function hostSession(roomId: string, hostKey: string, name: string) {
	return requestJson<MediaCredentials>(`${room(roomId)}/host-session`, {
		method: 'POST',
		accessKey: hostKey,
		body: { name }
	});
}

export function knock(roomId: string, name: string) {
	return requestJson<Knock>(`${room(roomId)}/requests`, { method: 'POST', body: { name } });
}

export function knockOutcome(roomId: string, request: Knock) {
	return requestJson<KnockOutcome>(
		`${room(roomId)}/requests/${encodeURIComponent(request.requestId)}`,
		{ accessKey: request.requestKey }
	);
}

export function pendingRequests(roomId: string, hostKey: string) {
	return requestJson<PendingRequest[]>(`${room(roomId)}/requests`, { accessKey: hostKey });
}

export function decideRequest(roomId: string, hostKey: string, requestId: string, admit: boolean) {
	const action = admit ? 'admit' : 'deny';
	return requestVoid(`${room(roomId)}/requests/${encodeURIComponent(requestId)}/${action}`, {
		method: 'POST',
		accessKey: hostKey
	});
}

/** Hidden listener used to hear yourself through the server. */
export function monitorSession(roomId: string, accessKey: string) {
	return requestJson<MediaCredentials>(`${room(roomId)}/monitor-session`, {
		method: 'POST',
		accessKey
	});
}
