import type { RoomEntry } from '#lib/features/room/index.ts';

/** One listed room on the live board (hosts opt in when creating the room). */
export interface RankedRoom {
	/** Link code of a listed room (its host chose to publish it). */
	code: string;
	title: string;
	/** Username of the room's owner. */
	owner: string;
	entry: RoomEntry;
	/** The room's animated symbol. */
	mascot: string;
	people: number;
	/** Output languages heard in the room, most listened first. */
	languages: string[];
}

/** `GET /api/v1/presence`: totals cover every room, the ranking only listed ones. */
export interface Presence {
	online: number;
	liveRooms: number;
	ranking: RankedRoom[];
}
