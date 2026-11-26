/** Mirrors `Username` in crates/domain/src/username.rs (the server has the final say). */
const PATTERN = /^[a-z][a-z0-9_]{2,19}$/;

/** What the server will store: trimmed, without a leading `@`, lowercase. */
export function normalizeUsername(raw: string): string {
	return raw.trim().replace(/^@/, '').toLowerCase();
}

export function isValidUsername(raw: string): boolean {
	return PATTERN.test(normalizeUsername(raw));
}
