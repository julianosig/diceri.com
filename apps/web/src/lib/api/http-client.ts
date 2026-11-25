/** Error thrown for non-2xx API responses (RFC 9457 problem details). */
export class ApiError extends Error {
	constructor(
		readonly status: number,
		readonly title: string
	) {
		super(`${String(status)} ${title}`);
		this.name = 'ApiError';
	}
}

type FetchFn = typeof fetch;

/**
 * Sends a JSON request to the Rust API. Requests are same-origin: the Vite dev
 * server proxies `/api` and production serves the SPA from the API host.
 */
export async function requestJson<T>(
	path: string,
	init: RequestInit = {},
	fetchFn: FetchFn = fetch
): Promise<T> {
	const headers = new Headers(init.headers);
	headers.set('accept', 'application/json');
	const response = await fetchFn(`/api/v1${path}`, { ...init, headers });
	if (!response.ok) {
		const problem = (await response.json().catch(() => ({}))) as { title?: string };
		throw new ApiError(response.status, problem.title ?? response.statusText);
	}
	return (await response.json()) as T;
}
