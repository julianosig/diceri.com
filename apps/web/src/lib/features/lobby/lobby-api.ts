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
	language: string;
}

/** How a participant presents themselves to the room. */
export interface Profile {
	name: string;
	/** BCP 47 tag of the language they speak and want to hear. */
	language: string;
}

export function hostSession(roomId: string, hostKey: string, profile: Profile) {
	return requestJson<MediaCredentials>(`${room(roomId)}/host-session`, {
		method: 'POST',
		accessKey: hostKey,
		body: profile
	});
}

export function knock(roomId: string, profile: Profile) {
	return requestJson<Knock>(`${room(roomId)}/requests`, { method: 'POST', body: profile });
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
