import { isEasyToGuess } from './password-strength.ts';

/** Mirrors `Password` in crates/domain/src/account/password.rs and its strength check. */
export const MIN_PASSWORD_LENGTH = 8;
export const MAX_PASSWORD_LENGTH = 128;

/** What is wrong with a new password; the component words it in the visitor's language. */
export type PasswordProblem = 'too-short' | 'too-long' | 'too-easy' | 'mismatch';

/** What is wrong with a new password for `username` and its confirmation, or `null`. */
export function passwordProblem(
	password: string,
	repeat: string,
	username: string
): PasswordProblem | null {
	const length = Array.from(password).length;
	if (length < MIN_PASSWORD_LENGTH) return 'too-short';
	if (length > MAX_PASSWORD_LENGTH) return 'too-long';
	if (isEasyToGuess(password, username)) return 'too-easy';
	if (repeat.length > 0 && repeat !== password) return 'mismatch';
	return null;
}
