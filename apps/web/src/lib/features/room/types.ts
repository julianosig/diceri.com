/** Mirrors `RoomResponse` in apps/api/src/features/rooms/dto.rs. */
export interface Room {
	id: string;
	maxParticipants: number;
}

/** Returned once on creation; `hostKey` proves the creator is the host. */
export interface CreatedRoom extends Room {
	hostKey: string;
}

/** Mirrors `JoinResponse`: credentials to connect to the media server. */
export interface MediaCredentials {
	identity: string;
	serverUrl: string;
	token: string;
}
