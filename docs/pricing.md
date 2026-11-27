# Pricing and margins

How the plan prices and allowances (defaults in `crates/application/src/use_cases/plan_limits.rs`,
overridable with `DICERI_<PLAN>_*`, see `apps/api/AGENTS.md`) were derived. Prices are list
prices checked on 2026-11-27; recheck them when a provider or model changes.

## Cost of one call-minute

A two-person call where each person hears the other in another language. Every minute of the
call costs, whether or not anyone is talking, because each person's audio streams to
speech-to-text for the whole call.

| Item | Provider (model) | Assumption | Cost / min |
| --- | --- | --- | --- |
| Speech-to-text | ElevenLabs Scribe v3 Realtime | $0.22 per audio hour, one stream per person (2) | $0.0073 |
| Translation | Google Gemini 3.5 Flash-Lite | ~12 phrases/min × ~1,500 input tokens (prompt + context) at $0.30/M, ~300 output tokens at $2.50/M | $0.0062 |
| Speech synthesis | ElevenLabs v4 Turbo | ~700 translated characters/min at $0.05 per 1,000 | $0.0350 |
| Media participants | LiveKit Cloud (Ship) | 3 participants (2 people + translator agent) at $0.0005/min | $0.0015 |
| Media bandwidth | LiveKit Cloud (Ship) | ~15 MB/min downstream (two ~1 Mbps videos) at $0.12/GB | $0.0018 |
| **Total** | | | **≈ $0.052 (≈ $3.10/hour)** |

Speech synthesis is two thirds of the cost; a cheaper TTS route is the largest lever. The LiveKit
lines are charged at the overage rate for safety: the Ship plan includes 150,000 participant
minutes and 250 GB a month, so they are close to zero until about 50,000 call minutes a month.
More people in a call raise speech-to-text (one stream each) and synthesis (one voice per
listener language), which is why every plan is limited to two people today.

## Fixed monthly costs

| Item | Cost / month |
| --- | --- |
| LiveKit Cloud Ship plan (needed to sell; Build allows ~1,660 call minutes) | $50 |
| Free plan, capped at 1,500 call minutes across all accounts (`DICERI_MONTHLY_CALL_MINUTES`) | up to $78 |
| This server (already running the site) | not included |
| **Total, excluding the server** | **≈ $128** |

## Plans

Payment processing is taken as 4.5% + $0.30 per charge (international card, conservative).
Taxes are not included. "Full use" spends every included minute; "half use" is the more usual case
for subscriptions.

| Plan | Price / month | Included | Per call | Per day | Cost at full use | Margin at full use | Margin at half use |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Free | $0 | 200 min | 20 min | 40 min | up to $10.40 per account | n/a (marketing cost, capped globally) | n/a |
| Pro | $29 | 300 min (5 h) | 60 min | 300 min | $15.60 + $1.61 fees = $17.21 | **$11.79 (41%)** | $19.59 (68%) |
| Team | $79 | 800 min (13 h 20) | 120 min | 600 min | $41.60 + $3.86 fees = $45.46 | **$33.54 (42%)** | $54.34 (69%) |

Break-even on the fixed costs (excluding the server): about 11 Pro subscribers at full use, or
7 at half use.

The monthly allowance is what keeps the margin: daily limits alone would let a Pro account use
300 × 30 = 9,000 minutes (≈ $470) for $29. Calls end when the call, daily or monthly limit is
reached, and the account cannot start another until the limit renews.
