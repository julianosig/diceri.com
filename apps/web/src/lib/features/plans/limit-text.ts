import { ApiError } from '#lib/api/http-client.ts';
import type { Messages } from '#lib/i18n/index.ts';

const LIMITS = ['daily', 'quota', 'monthly', 'busy', 'network', 'beta'] as const;
type Limit = (typeof LIMITS)[number];

function isLimit(value: string | undefined): value is Limit {
	return LIMITS.some((limit) => limit === value);
}

/**
 * The visitor-language explanation of a plan or capacity limit the API refused with,
 * or `undefined` for any other error.
 */
export function limitText(cause: unknown, t: Messages): string | undefined {
	if (!(cause instanceof ApiError) || !isLimit(cause.limit)) return undefined;
	return t.limits[cause.limit];
}
