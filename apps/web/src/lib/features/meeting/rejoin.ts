/**
 * Remembers, per browser tab, the room whose call this tab is in, so reloading the page
 * goes straight back into the call. Leaving or the call ending forgets it; closing the tab
 * forgets it too (session storage). Storage can be unavailable: then nothing is resumed.
 */
const KEY = 'diceri:in-call';

export function markInCall(roomCode: string): void {
	try {
		sessionStorage.setItem(KEY, roomCode);
	} catch {
		// Storage blocked: a reload goes through pre-join as usual.
	}
}

export function forgetInCall(): void {
	try {
		sessionStorage.removeItem(KEY);
	} catch {
		// Nothing was kept.
	}
}

/** Whether this tab was in `roomCode`'s call before it was reloaded. */
export function wasInCall(roomCode: string): boolean {
	try {
		return sessionStorage.getItem(KEY) === roomCode;
	} catch {
		return false;
	}
}
