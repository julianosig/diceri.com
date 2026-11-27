/** One plan and what it allows (`GET /api/v1/capabilities`). */
export interface PlanInfo {
	plan: 'free' | 'pro' | 'team';
	/** Longest single call. */
	callMinutes: number;
	/** Call minutes per UTC day, across all of the owner's calls. */
	dailyMinutes: number;
}

/** The signed-in account's use today (`GET /api/v1/accounts/me`). */
export interface Usage {
	usedToday: number;
	dailyMinutes: number;
	callMinutes: number;
}
