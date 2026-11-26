/** How people get in: straight away, or after the host approves them. */
export type RoomEntry = 'open' | 'approval';

/** Mirrors `RoomResponse` in apps/api/src/features/rooms/dto.rs. */
export interface Room {
	id: string;
	maxParticipants: number;
	title: string;
	/** Known once the host has joined. */
	hostName: string | null;
	entry: RoomEntry;
}

/** Mirrors `CreateRoomRequest`: what the host chooses when creating a room. */
export interface NewRoom {
	title: string;
	/** Show on the public live board (title and host name). */
	listed: boolean;
	entry: RoomEntry;
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
