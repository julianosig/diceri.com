/** Mirrors `RoomResponse` in apps/api/src/features/rooms/dto.rs. */
export interface Room {
	id: string;
	maxParticipants: number;
}

/** `speaker` publishes the microphone; `monitor` is a hidden listener. */
export type JoinRole = 'speaker' | 'monitor';

/** Mirrors `JoinResponse` in apps/api/src/features/rooms/dto.rs. */
export interface MediaCredentials {
	identity: string;
	serverUrl: string;
	token: string;
}
