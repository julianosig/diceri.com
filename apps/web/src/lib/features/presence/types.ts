/** One room in the public ranking: no id or names, only size and languages. */
export interface RankedRoom {
	people: number;
	/** Output languages heard in the room, most listened first. */
	languages: string[];
}

/** `GET /api/v1/presence`. */
export interface Presence {
	online: number;
	liveRooms: number;
	ranking: RankedRoom[];
}
