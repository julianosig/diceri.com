/** Mirrors `Password` in crates/domain/src/account/password.rs (length only). */
export const MIN_PASSWORD_LENGTH = 8;
export const MAX_PASSWORD_LENGTH = 128;

/** What is wrong with a new password; the component words it in the visitor's language. */
export type PasswordProblem = 'too-short' | 'too-long' | 'mismatch';

/** What is wrong with a new password and its confirmation, or `null` when fine. */
export function passwordProblem(password: string, repeat: string): PasswordProblem | null {
	const length = Array.from(password).length;
	if (length < MIN_PASSWORD_LENGTH) return 'too-short';
	if (length > MAX_PASSWORD_LENGTH) return 'too-long';
	if (repeat.length > 0 && repeat !== password) return 'mismatch';
	return null;
}
