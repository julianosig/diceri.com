const KEY = 'diceri:display-name';
export const MAX_NAME_LENGTH = 40;

/** Last name the user typed, so returning users skip retyping. */
export function loadDisplayName(): string {
	try {
		return localStorage.getItem(KEY) ?? '';
	} catch {
		return '';
	}
}

export function saveDisplayName(name: string): void {
	try {
		localStorage.setItem(KEY, name.trim());
	} catch {
		// Storage blocked: nothing to remember.
	}
}

export function isValidDisplayName(name: string): boolean {
	const trimmed = name.trim();
	return trimmed.length > 0 && trimmed.length <= MAX_NAME_LENGTH;
}
