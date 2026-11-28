const PROBLEMS = [
	'filterUnsupported',
	'filterFailed',
	'micUnpublished',
	'micChangeFailed',
	'micLost'
] as const;

/** Problems the call itself detects; components word them in the visitor's language. */
export type CallProblem = (typeof PROBLEMS)[number];

/** Whether an error message is one of ours (a key) rather than a server's text. */
export function isCallProblem(message: string): message is CallProblem {
	return PROBLEMS.some((problem) => problem === message);
}
