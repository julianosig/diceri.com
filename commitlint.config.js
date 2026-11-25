/** Conventional Commits. Scopes and rules are documented in AGENTS.md. */
export default {
	extends: ['@commitlint/config-conventional'],
	rules: {
		'header-max-length': [2, 'always', 72],
		'body-max-line-length': [2, 'always', 100],
		'subject-case': [2, 'always', 'lower-case'],
		'scope-enum': [
			2,
			'always',
			[
				'web',
				'api',
				'domain',
				'application',
				'infra',
				'media',
				'translation',
				'rust',
				'ci',
				'deps',
				'docs',
				'repo'
			]
		]
	}
};
