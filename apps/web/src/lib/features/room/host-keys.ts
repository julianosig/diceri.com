/**
 * Remembers which rooms this browser created, so the creator re-enters as host.
 * Storage can be unavailable (private mode); then hosting lasts for the tab only.
 */
const PREFIX = 'diceri:host-key:';
const memory = new Map<string, string>();

export function saveHostKey(roomId: string, hostKey: string): void {
	memory.set(roomId, hostKey);
	try {
		localStorage.setItem(PREFIX + roomId, hostKey);
	} catch {
		// Storage blocked: keep the in-memory copy.
	}
}

export function loadHostKey(roomId: string): string | undefined {
	try {
		return localStorage.getItem(PREFIX + roomId) ?? memory.get(roomId);
	} catch {
		return memory.get(roomId);
	}
}
