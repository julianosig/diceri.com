import prettier from 'eslint-config-prettier';
import path from 'node:path';
import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import { defineConfig, includeIgnoreFile } from 'eslint/config';
import globals from 'globals';
import ts from 'typescript-eslint';

const gitignorePath = path.resolve(import.meta.dirname, '.gitignore');

/** Size limits shared with AGENTS.md. Change both together. */
const LIMITS = {
	tsFileLines: 400,
	svelteFileLines: 300,
	functionLines: 80,
	complexity: 12,
	depth: 4,
	params: 4
};

const countOptions = { skipBlankLines: true, skipComments: true };

export default defineConfig(
	includeIgnoreFile(gitignorePath),
	{ ignores: ['**/generated/**'] },
	js.configs.recommended,
	ts.configs.strictTypeChecked,
	svelte.configs.recommended,
	prettier,
	svelte.configs.prettier,
	{
		languageOptions: {
			parserOptions: { projectService: true },
			globals: { ...globals.browser, ...globals.node }
		},
		rules: {
			// typescript-eslint recommends disabling no-undef on TypeScript projects.
			'no-undef': 'off',
			'max-lines': ['error', { max: LIMITS.tsFileLines, ...countOptions }],
			'max-lines-per-function': ['error', { max: LIMITS.functionLines, ...countOptions }],
			complexity: ['error', LIMITS.complexity],
			'max-depth': ['error', LIMITS.depth],
			'max-params': ['error', LIMITS.params],
			'no-console': ['error', { allow: ['warn', 'error'] }],
			'no-restricted-imports': [
				'error',
				{
					patterns: [
						{ group: ['$lib', '$lib/*'], message: 'SvelteKit 3 uses #lib imports.' },
						{ group: ['$app/stores'], message: 'Use $app/state instead.' },
						{
							group: ['#lib/features/*/*'],
							message: 'Import features only through their public index.ts.'
						}
					]
				}
			],
			'svelte/button-has-type': 'error',
			'svelte/require-each-key': 'error'
		}
	},
	{
		files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: ['.svelte'],
				parser: ts.parser
			}
		}
	},
	{
		files: ['**/*.svelte'],
		rules: {
			'max-lines': ['error', { max: LIMITS.svelteFileLines, ...countOptions }]
		}
	},
	{
		files: ['**/*.spec.ts', '**/*.e2e.ts'],
		rules: { 'max-lines-per-function': 'off' }
	},
	{
		files: ['*.config.js', '*.config.ts'],
		...ts.configs.disableTypeChecked
	}
);
