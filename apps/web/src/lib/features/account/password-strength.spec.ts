import { describe, expect, it } from 'vitest';
import { isEasyToGuess } from './password-strength.ts';

// Same examples as crates/domain/src/account/password_strength.rs.
describe('isEasyToGuess', () => {
	it('refuses guessable passwords', () => {
		for (const raw of [
			'12345678',
			'87654321',
			'abcdefgh',
			'aaaaaaaa',
			'abababab',
			'11223344',
			'password',
			'Password123!',
			'qwerty123',
			'iloveyou',
			'48291736',
			'Senha@2024'
		]) {
			expect(isEasyToGuess(raw, 'ana'), raw).toBe(true);
		}
		expect(isEasyToGuess('ana_92-rocks!', 'ana_92')).toBe(true);
	});

	it('accepts hard-to-guess passwords', () => {
		for (const raw of [
			'correct horse battery',
			'Tr0ub4dor&3',
			'blue-otter-sings',
			'829174650312'
		]) {
			expect(isEasyToGuess(raw, 'ana'), raw).toBe(false);
		}
	});
});
