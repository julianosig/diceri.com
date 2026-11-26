/** Matches `MAX_TITLE_CHARS` in crates/domain/src/room_settings.rs. */
export const MAX_TITLE_LENGTH = 60;

/** Starting points so nobody faces an empty box; the host edits freely. */
const SUGGESTIONS = [
	'Coffee chat',
	'Language exchange',
	'Practice English',
	'Hablemos español',
	'Bate-papo em português',
	'Travel stories',
	'Open mic, any language',
	'Meet the world'
] as const;

export function suggestTitle(random: () => number = Math.random): string {
	const index = Math.floor(random() * SUGGESTIONS.length) % SUGGESTIONS.length;
	return SUGGESTIONS[index] ?? SUGGESTIONS[0];
}

export function isValidTitle(title: string): boolean {
	const trimmed = title.trim();
	// Code points, like `chars().count()` on the server.
	return trimmed.length > 0 && Array.from(trimmed).length <= MAX_TITLE_LENGTH;
}
