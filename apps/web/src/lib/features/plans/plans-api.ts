import { requestJson } from '#lib/api/http-client.ts';
import type { PlanInfo } from './types.ts';

/** Every plan and its limits, free first (everyone's default). */
export async function getPlans(fetchFn?: typeof fetch): Promise<PlanInfo[]> {
	const capabilities = await requestJson<{ plans: PlanInfo[] }>('/capabilities', { fetchFn });
	return capabilities.plans;
}
