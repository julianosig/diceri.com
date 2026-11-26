/** How people get in: straight away, or after the host approves them. */
export type RoomEntry = 'open' | 'approval';

/** Mirrors `RoomResponse` in apps/api/src/features/rooms/dto.rs. */
export interface Room {
	/** UUIDv7: identifies the room, grants nothing. */
	id: string;
	/** The shareable link code (`/r/<code>`); every room API path takes it. */
	code: string;
	maxParticipants: number;
	title: string;
	/** Username of the account that owns (hosts) the room. */
	owner: string;
	entry: RoomEntry;
	/** The room's animated symbol; see `mascots.ts`. */
	mascot: string;
}

/** Mirrors `CreateRoomRequest`: what the host chooses when creating a room. */
export interface NewRoom {
	title: string;
	/** Show on the public live board (title and host name). */
	listed: boolean;
	entry: RoomEntry;
	mascot: string;
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
