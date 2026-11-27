import { requestJson } from '#lib/api/http-client.ts';
import type { BetaSpots, PlanInfo } from './types.ts';

/** Every plan (free first, everyone's default) and the private beta's spots, if any. */
export async function getPlanOffer(
	fetchFn?: typeof fetch
): Promise<{ plans: PlanInfo[]; beta?: BetaSpots }> {
	return requestJson<{ plans: PlanInfo[]; beta?: BetaSpots }>('/capabilities', { fetchFn });
}
