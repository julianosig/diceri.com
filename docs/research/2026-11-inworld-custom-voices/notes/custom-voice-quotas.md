# Inworld AI "custom voices" plan allowance and attached quotas (as of 2026-11-27)

Scope: what the per-plan "custom voices" number means for an app that instantly clones each call
participant's voice on join and deletes it when the call ends (thousands of create/delete cycles
per month). All pages were fetched on 2026-11-27 unless stated otherwise. "Official" means an
inworld.ai or docs.inworld.ai page; third-party sources are labelled.

## 1. Is "custom voices" a concurrent (stored) cap, a monthly cap or a lifetime cap? What counts, and at what scope?

### Takeaway
It is a cap on the number of custom voices the organization **holds at the same time** (a stock
limit, not a creation counter). Instant clones, designed voices and voices copied in with a copy
token all count toward it; professional voice clones (PVC) do not, because they have separate
"PVC voice slots". The limit is per **organization**, shared by all its workspaces.

### Cited Findings
Official:
- Current plan cards: On-Demand "100 custom voices" / "2 professional voice clones"; Creator 500 /
  4; Builder 3,000 / 10; Developer 10,000 / 25; Growth 30,000 / 40; Enterprise "Custom limits" /
  "100+ professional voice clones". On-Demand lists "Voice cloning & voice design" —
  [Inworld pricing](https://inworld.ai/pricing); identical figures on
  [Inworld founder pricing](https://inworld.ai/founder-pricing).
- "Your plan sets how many custom voices your organization can have." —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- "Instant clones, designed voices and voices copied in with a copy token all count toward one
  limit"; "Professional voice clones are not counted here: they have their own voice slots." —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- The limit is "shared by every workspace in your organization"; the voice list endpoint "covers
  only the API key's workspace, so its count can be lower than `usedCustomVoices`." —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- Capacity endpoint `GET https://api.inworld.ai/voices/v1/voices:customVoiceCapacity` (Basic auth;
  "It is free, and a standard key with read access to voices works"). `usedCustomVoices` is "how
  many custom voices your organization has now, across all its workspaces"; `maxCustomVoices` is
  "your plan's limit plus any add-ons"; "If `maxCustomVoices` is absent, no limit applies." —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- API reference for the same endpoint: `usedCustomVoices` = "Custom voices that count toward
  `maxCustomVoices`: instant clones, designed voices and copied voices ... in every workspace of
  your organization. Voices produced by Professional Voice Cloning are not counted."
  `maxCustomVoices` = "The most custom voices your organization can have, from its plan plus any
  add-ons." No billing period or reset is mentioned. —
  [Get custom voice capacity API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/get-custom-voice-capacity.md)
- PVC slots are reported separately: `usedPvcVoiceSlots` ("PVC voice slots currently claimed by
  this account") and `maxPvcVoiceSlots` ("Total PVC voice slots this account's plan allows,
  including add-ons"), via `pvcVoices:resolveUploadLimits` —
  [Resolve PVC upload limits API reference](https://docs.inworld.ai/api-reference/pvcAPI/pvcvoiceservice/resolve-pvc-upload-limits.md)
- "On-Demand accounts need a payment method on file before creating a professional voice clone." —
  [Inworld pricing](https://inworld.ai/pricing)
- Instant cloning is "available to all users through Portal or via API." —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- Instant clone creation is `POST https://api.inworld.ai/voices/v1/voices:clone`; IVC voice ids are
  workspace-prefixed; best with a 10–30 s clip, longer clips cut at 30 s; WAV/MP3 base64. Write
  permission is required for POST/PATCH/DELETE. —
  [Clone voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice.md)

### Inferences
- The wording "has now", "can have" and a used/max pair with no period strongly indicates a
  concurrent stock cap, not a monthly or lifetime creation counter. For Diceri the binding number is
  peak simultaneous cloned participants (plus any clones not yet deleted), not monthly joins. Even
  On-Demand (100) would cover ~100 simultaneous cloned speakers if every clone is deleted on call
  end.
- Because the count is organization-wide, staging/dev workspaces under the same organization share
  the production allowance; leaked (undeleted) clones in any workspace eat into it. A periodic
  sweeper that compares `voices:customVoiceCapacity` with live calls would catch leaks.

### Gaps
- Not found: an explicit sentence saying "this is not a monthly limit". The conclusion rests on
  the field definitions above.
- How add-ons to `maxCustomVoices` are bought and priced is not documented on any page fetched.
- Whether Inworld's "organization" maps 1:1 to the billing account was not stated explicitly (rate
  limit pages say "account"; voice pages say "organization").

## 2. Does deleting a voice free the slot immediately? Monthly creation cap, edit cap or per-clone fee?

### Takeaway
Yes: deleting a private voice "frees room right away". No official page fetched mentions a monthly
creation cap, an edit cap or a per-clone fee (no equivalent of ElevenLabs' monthly add/edit
operations). The only exception to deletion is a voice shared to the Voice Library.

### Cited Findings
Official:
- "To create more, delete private voices you no longer use, which frees room right away, or upgrade
  your plan." —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- "A voice shared to the Voice Library can't be deleted until you remove it from the library", and
  "removal has a 2-year notice period", so that room is not freed soon. —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- Delete endpoint: `DELETE /voices/v1/voices/{voiceId}`, "delete a custom voice from your
  workspace"; no quota or slot text, no specific error codes (generic `code`/`message`/`details`). —
  [Delete voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/delete-voice.md)
- The instant cloning page mentions no monthly creation limit, edit limit, per-clone fee or
  creation cost; the only cap is the total count. —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md); the clone API
  reference also lists no pricing —
  [Clone voice API reference](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice.md)
- Billing docs mention per-request billing only for design prompts: "A voice design prompt sent
  with a synthesis request counts once per request." Plan features list "Number of custom voices
  (see Custom voice limit to check your usage)". —
  [TTS billing docs](https://docs.inworld.ai/tts/resources/billing)
- Pricing page FAQ does not define a custom voice or say whether deletion frees a slot. —
  [Inworld pricing](https://inworld.ai/pricing)

### Inferences
- For create-on-join/delete-on-leave, the voice count does not accumulate; the practical limiter
  is the cloning rate limit (section 3), not the voice allowance.
- Diceri must never publish participant clones to the Voice Library (2-year removal notice would
  also conflict with the "clones are deleted when the call ends" rule in ADR 0006).

### Gaps
- Whether deleting a PVC frees a PVC slot is not stated (PVC limits page silent).
- No official statement that cloning itself is free of charge; it is only absent from every pricing
  and billing page fetched. Worth confirming with Inworld sales/support before relying on it.
- Whether there is any abuse/fair-use limit on create/delete churn is not documented.

## 3. Rate limits specific to voice cloning/creation, and behaviour at the limit

### Takeaway
Voice cloning and voice design each have a per-plan requests-per-minute limit: 2 (On-Demand),
3 (Creator), 5 (Builder), 10 (Developer), 20 (Growth), custom (Enterprise). Exceeding a rate limit
returns HTTP 429; reaching the custom voice cap returns HTTP 400 `INVALID_ARGUMENT`.

### Cited Findings
Official:
- "Voice cloning API (requests/min)": 2 / 3 / 5 / 10 / 20 / Custom for On-Demand / Creator /
  Builder / Developer / Growth / Enterprise; "Voice design API (requests/min)": same values. —
  [TTS billing docs](https://docs.inworld.ai/tts/resources/billing)
- Other TTS limits: concurrent generations 5 / 10 / 50 / 150 / 500 / Custom; WebSocket connections
  50 / 100 / 500 / 1,500 / 5,000 / Custom; "Concurrency limits apply per account and are shared
  across API keys"; "Requests above these guaranteed limits are best-effort based on available
  capacity." — [TTS billing docs](https://docs.inworld.ai/tts/resources/billing). Pricing page shows
  the same concurrency figures and "Estimated concurrent user sessions" 20 / 40 / 200 / 600 /
  2,000 — [Inworld pricing](https://inworld.ai/pricing)
- "Voice cloning has lower rate limits than regular speech synthesis." —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)
- Rate limits "apply per account and are shared across your API keys"; over-limit requests get
  `429 Too Many Requests` and are not processed; per-second limits return plain-text "Rate limit
  exceeded" with no `Retry-After`; longer windows (e.g. per minute) may include `Retry-After`;
  recommended handling is exponential backoff with jitter and honouring `Retry-After`. The general
  page gives no numeric TTS values and does not mention voice-management (list/delete) limits. —
  [Rate limits docs](https://docs.inworld.ai/resources/rate-limits)
- At the custom voice cap, create requests fail with `400` (`INVALID_ARGUMENT`) when
  `usedCustomVoices` equals `maxCustomVoices`; example message: "You have exceeded your plan's
  maximum number of custom voices (500). Please upgrade to unlock more." "Your existing voices keep
  working." Portal shows a notice at 80% use and locks with "Voice limit reached" at the cap. —
  [Instant voice cloning docs](https://docs.inworld.ai/tts/instant-voice-cloning.md)

Third-party (unverified):
- A search-engine summary attributed a single "Voice cloning API requests/min 5" figure to the
  founder pricing page; the live page fetched today shows no cloning rate limit —
  [Inworld founder pricing](https://inworld.ai/founder-pricing).

### Inferences
- Rate is the real constraint for Diceri: at 2 clones/min (On-Demand) a burst of participants
  joining together would queue; 20 people joining in the same minute needs Growth (20/min) or
  queuing. Per hour that is 120 (On-Demand) to 1,200 (Growth) clones, so "thousands per month"
  fits even On-Demand on volume, but not on burst.
- Clone requests should go through a queue with 429 backoff, and the app should fall back to a
  stock voice (already the ADR 0006 default) while a clone is pending or rate-limited.
- The 400 at the cap is the same status as invalid input; distinguish it by message or by calling
  `voices:customVoiceCapacity` first.

### Gaps
- No documented per-day cloning limit, and no documented rate limit for delete or list calls.
- Whether the cloning rpm is per account or per organization when several workspaces exist is
  implied (per account) but not stated for cloning specifically.
- No official confirmation that cloning 429s carry `Retry-After`.

## 4. Recent changes (changelog, 2026 plan changes) and staff statements

### Takeaway
The TTS release notes list no change to custom voice limits; the visible 2026 changes are product
(Voice Design update in May 2026, Realtime TTS-2 / PVC on `inworld-tts-2` in August 2026).
Search snippets show an older, much lower set of limits (50 / 700 / 1,000), which suggests the
allowance was raised at some point, but that older version could not be fetched or dated.

### Cited Findings
Official:
- 2026-08-09: Realtime TTS-2 Flash entry references instant voice cloning; "Professional Voice
  Cloning are supported on `inworld-tts-2` only." 2026-05-05: "Released an updated version of Voice
  Design with improved generations." 2025-08-22: "Cloned voices sound closer to the originals". No
  entry mentions custom voice limits, capacity, deletion, copy tokens or rate limits. —
  [TTS release notes](https://docs.inworld.ai/release-notes/tts.md)
- Payment failure: "If all retries are exhausted, your account is downgraded to the On-Demand tier
  and API usage is disabled." Upgrades carry unused credits forward. —
  [TTS billing docs](https://docs.inworld.ai/tts/resources/billing)

Third-party / unverified:
- A web-search summary attributed to Inworld's pricing page older caps (On-Demand 5, Creator 50, a
  mid tier 700, Growth 1,000) and the statements "if you have more voices than your plan limit, you
  will not be able to create new voices unless you upgrade" and existing voices "remain available
  regardless of plan". The live pricing page does not contain these figures or phrases today
  (verified by fetch of [Inworld pricing](https://inworld.ai/pricing) and
  [founder pricing](https://inworld.ai/founder-pricing)). Treat as an outdated cached version.
- iTechGuides review (checked 2026-09-20 per the search summary) lists 100 / 500 / 10,000 / 30,000,
  matching the current official page —
  [iTechGuides Inworld TTS](https://www.itechguides.com/products/inworld-tts/) (not fetched directly).
- LiveKit (third-party integrator) offers Inworld custom voices on its Ship plan or higher and
  deletes them from all providers via its dashboard —
  [LiveKit custom voices](https://docs.livekit.io/agents/models/tts/custom-voices/) (search summary
  only). Not relevant if Diceri calls Inworld directly.

### Inferences
- The current docs (capacity endpoint, 80% notice, explicit "frees room right away") look recent
  and deliberate; the allowance appears to have grown roughly 10–20x versus the older snippet.

### Gaps
- Date of the custom voice limit change could not be established; no changelog or blog entry found.
- No Inworld Discord, community forum or help-center statement by staff was found; Discord is behind
  login and was not searched.
- Docs page https://docs.inworld.ai/tts/voice-cloning.md redirects to a Portal login
  (`platform.inworld.ai/docs-auth/login-redirect`), so its content could not be read.
- `dev.docs.inworld.ai` did not resolve (DNS failure); only the production docs were used.
- Terms of service were not reviewed for voice-cloning consent or churn clauses.
