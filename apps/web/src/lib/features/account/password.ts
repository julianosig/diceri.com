/** Mirrors `Password` in crates/domain/src/account/password.rs (length only). */
export const MIN_PASSWORD_LENGTH = 8;
export const MAX_PASSWORD_LENGTH = 128;

/** What is wrong with a new password and its confirmation, or `null` when fine. */
export function passwordProblem(password: string, repeat: string): string | null {
	const length = Array.from(password).length;
	if (length < MIN_PASSWORD_LENGTH) {
		return `Use at least ${String(MIN_PASSWORD_LENGTH)} characters.`;
	}
	if (length > MAX_PASSWORD_LENGTH) {
		return `Use at most ${String(MAX_PASSWORD_LENGTH)} characters.`;
	}
	if (repeat.length > 0 && repeat !== password) return 'The passwords do not match.';
	return null;
}
