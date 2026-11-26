/** How people get in: straight away, or after the host approves them. */
export type RoomEntry = 'open' | 'approval';

/** Mirrors `RoomResponse` in apps/api/src/features/rooms/dto.rs. */
export interface Room {
	/** UUIDv7: identifies the room, grants nothing. */
	id: string;
	/** The code every lobby API path takes. */
	code: string;
	maxParticipants: number;
	/** The owner's username, which is also the room's name (`/@owner`). */
	owner: string;
	entry: RoomEntry;
	/** The room's animated mascot; see `mascots.ts`. */
	mascot: string;
}

/** Mirrors `OpenRoomRequest`: how the owner runs their room this time. */
export interface RoomOptions {
	/** Show on the public live board. */
	listed: boolean;
	entry: RoomEntry;
}

/** Returned to the owner; `hostKey` proves they are the host. */
export interface OpenedRoom extends Room {
	hostKey: string;
}

/** Mirrors `JoinResponse`: credentials to connect to the media server. */
export interface MediaCredentials {
	identity: string;
	serverUrl: string;
	token: string;
}

/** The public face of a username: what `/@username` shows while its room is closed. */
export interface UserProfile {
	username: string;
	mascot: string;
}
