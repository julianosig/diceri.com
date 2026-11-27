/**
 * Refuses passwords that are easy to guess (NIST SP 800-63B: no composition rules, but no
 * common, sequential, repetitive or context-specific passwords). Mirrors
 * `crates/domain/src/account/password_strength.rs`, which the server enforces; keep both
 * lists and rules in step (the tests share their examples).
 */

/** Among the most used passwords of 8 or more characters (lowercase). */
const COMMON = new Set([
	'password',
	'password1',
	'password12',
	'password123',
	'passw0rd',
	'p@ssw0rd',
	'qwertyui',
	'qwertyuiop',
	'qwerty123',
	'qwerty12',
	'1q2w3e4r',
	'1qaz2wsx',
	'zaq12wsx',
	'asdfghjk',
	'asdfghjkl',
	'zxcvbnm1',
	'iloveyou',
	'iloveyou1',
	'abcd1234',
	'abc12345',
	'abcdefg1',
	'aa123456',
	'a1b2c3d4',
	'trustno1',
	'sunshine',
	'princess',
	'football',
	'baseball',
	'superman',
	'starwars',
	'whatever',
	'welcome1',
	'welcome123',
	'letmein1',
	'admin123',
	'administrator',
	'changeme',
	'computer',
	'internet',
	'michelle',
	'jennifer',
	'shadow12',
	'master12',
	'monkey12',
	'dragon12',
	'loveyou1',
	'88888888',
	'987654321',
	'123123123',
	'123qweasd',
	'qweasdzxc',
	'senha123',
	'senha1234',
	'mudar123',
	'brasil123'
]);

/** Words that make up most guessable passwords once digits and symbols are removed. */
const COMMON_WORDS = new Set([
	'password',
	'passwort',
	'senha',
	'contrasena',
	'motdepasse',
	'qwerty',
	'azerty',
	'asdf',
	'abc',
	'abcd',
	'iloveyou',
	'welcome',
	'admin',
	'letmein',
	'monkey',
	'dragon',
	'master',
	'sunshine',
	'princess',
	'football',
	'baseball',
	'soccer',
	'superman',
	'batman',
	'love',
	'hello',
	'test',
	'user',
	'login',
	'secret',
	'brasil',
	'brazil',
	'diceri',
	'qwe',
	'asd'
]);

/** Fewer distinct characters than this is a repetition (`aaaaaaaa`, `abababab`). */
const MIN_DISTINCT_CHARS = 5;
/** Digits only is a PIN; long ones are fine. */
const MIN_DIGITS_ONLY = 12;

/** Every character one step after (or before) the previous one: `12345678`, `hgfedcba`. */
function isSequence(chars: string[]): boolean {
	const steps = chars
		.slice(1)
		.map((char, index) => (char.codePointAt(0) ?? 0) - (chars[index]?.codePointAt(0) ?? 0));
	return steps.every((step) => step === 1) || steps.every((step) => step === -1);
}

/** Whether a new password is easy to guess for the account named `username`. */
export function isEasyToGuess(raw: string, username: string): boolean {
	const password = raw.toLowerCase();
	const name = username.toLowerCase();
	const chars = Array.from(password);
	const letters = chars.filter((char) => /\p{L}/u.test(char)).join('');
	return (
		COMMON.has(password) ||
		new Set(chars).size < MIN_DISTINCT_CHARS ||
		isSequence(chars) ||
		(letters === '' && chars.length < MIN_DIGITS_ONLY) ||
		COMMON_WORDS.has(letters) ||
		(Array.from(name).length >= 3 && password.includes(name))
	);
}
