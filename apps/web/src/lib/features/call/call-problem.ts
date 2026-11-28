/** Problems the call itself detects; components word them in the visitor's language. */
export type CallProblem =
	'filterUnsupported' | 'filterFailed' | 'micUnpublished' | 'micChangeFailed';

const PROBLEMS: readonly string[] = [
	'filterUnsupported',
	'filterFailed',
	'micUnpublished',
	'micChangeFailed'
];

/** Whether an error message is one of ours (a key) rather than a server's text. */
export function isCallProblem(message: string): message is CallProblem {
	return PROBLEMS.includes(message);
}
