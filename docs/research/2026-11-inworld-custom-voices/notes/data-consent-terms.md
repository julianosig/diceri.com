# Inworld AI: data handling, consent and legal terms for voice cloning and TTS (as of 2026-11-27)

Scope: what Diceri must know before sending users' voice recordings to Inworld for instant
cloning. Diceri is Brazil-based, AGPL, worldwide users, subject to LGPD and GDPR. All pages
were fetched on 2026-11-27. Where a page was rendered by JavaScript or gated, this is flagged.

Document versions seen:
- Terms of Service: "Effective Date: June 11, 2025" — https://inworld.ai/terms
- Acceptable Use Policy: "last updated June 11, 2025" — https://inworld.ai/aup
- Privacy Notice: "Last Updated: January 18, 2026" — https://inworld.ai/privacy
- Data Processing Addendum: last updated March 11, 2026 — https://inworld.ai/data-processing-addendum
- Voice Library Addendum: "Last Updated: June 10, 2026" — https://inworld.ai/voice-library-addendum
- Service Specific Terms (no date captured) — https://inworld.ai/service-specific-terms

## 1. Retention, Zero Data Retention (ZDR), logging and training

### Takeaway
ZDR is available only on Developer ($300/mo), Growth and Enterprise. It redacts TTS input text
and output from logs, but it **does not cover voice-cloning audio samples**: Inworld keeps the
source audio needed to build the voice. Training is restricted by the ToS ("No Training" on
non-public Materials), but the Privacy Notice and Service Specific Terms let Inworld use voice data
to "improve" the services, and the Privacy Notice says biometric data is used to improve services
"unless you opt out". The documents are not consistent with each other.

### Cited Findings
- The pricing page lists "Zero data retention" first on the Developer plan ($300/mo), and Growth
  ($1,500/mo) and Enterprise include it through "Everything in…". It is not listed on On-Demand,
  Creator ($25/mo) or Builder ($100/mo). The page banner says "GDPR & SOC 2 Type II on every plan".
  The checkmarks were inferred from the extracted text, so verify them on the live page —
  [Pricing](https://inworld.ai/pricing)
- ZDR plans: Developer, Growth, Enterprise and Enterprise - Trial. On-Demand, Creator and Builder
  can see the setting but cannot change it. Owners and Admins toggle it under Team > Data Retention,
  per organization or per workspace (a workspace override wins). Changes apply from the next
  request — [Inworld docs: Data & regions / ZDR](https://docs.inworld.ai/tts/zero-data-retention)
- Coverage: "Zero Data Retention is available across Inworld TTS, STT, hosted LLMs, and the Realtime
  API". Eligible inputs and outputs are processed in memory and redacted from logs. Usage metadata
  (model, counts, timestamps, identifiers) is still kept for billing and monitoring —
  [ZDR docs](https://docs.inworld.ai/tts/zero-data-retention)
- **Voice cloning audio samples and Voice Design descriptions and scripts are not eligible for
  ZDR.** TTS redacts all input text, including text sent to cloned voices, "while only retaining the
  initial source audio required to build the voice itself" —
  [ZDR docs](https://docs.inworld.ai/tts/zero-data-retention); [Security](https://inworld.ai/security)
  says "voice cloning samples and voice design prompts" "are not covered"
- One snapshot of the ZDR page in search results said ZDR covers only TTS-1.5-Mini and TTS-1.5-Max,
  not legacy TTS-1 or TTS-1-Max. A Cloudflare model listing marks TTS-2 as ZDR. The current docs
  page did not repeat the model restriction, so check model coverage before relying on it —
  [ZDR (TTS resources)](https://docs.inworld.ai/tts/resources/zero-data-retention);
  [Cloudflare TTS-2 listing](https://developers.cloudflare.com/ai/models/inworld/tts-2/index.md)
- With ZDR on, async and batch jobs are refused at submit unless results go to your own storage via
  `outputConfig.resultsUri`. Without that, results are kept for about 7 days —
  [ZDR docs](https://docs.inworld.ai/tts/zero-data-retention)
- Turning ZDR on "does not delete content already collected before you turned it on". Existing data
  is deleted by contacting support — [ZDR docs](https://docs.inworld.ai/tts/zero-data-retention)
- The ZDR page does not state the default retention setting (whether non-ZDR plans log text and
  audio by default) — [ZDR docs](https://docs.inworld.ai/tts/zero-data-retention)
- Training, STT only: "Audio processed through the Inworld STT API is never used for training by
  Inworld or any upstream provider." There is no equivalent sentence for TTS —
  [ZDR docs](https://docs.inworld.ai/tts/zero-data-retention)
- ToS §4 "No Training": "We will not train our generally available models on any Materials that are
  not publicly available". Exceptions: feedback (perpetual licence) and content flagged for
  violations, which "we may use or analyze … to improve our ability to detect and enforce any
  violations". §4 also licenses Materials "to operate, maintain, and improve the Services" —
  [Terms](https://inworld.ai/terms)
- Service Specific Terms §1(d): the customer grants a licence to "use your Input to provide the
  Services, to improve the Services, and to ensure the safety and security of the Services". For
  voices this covers "your voice, and other indicia of your persona". §1(b): "Inworld AI is
  permitted to use those audio recordings of the voice you provide to us subject to these terms" —
  [Service Specific Terms](https://inworld.ai/service-specific-terms)
- Privacy Notice: voice recordings, "where we have your consent", are used "to derive a digital model
  of speech characteristics". This biometric data is used to provide and improve the services
  "(unless you opt out)". Elsewhere the notice says information is used to develop products,
  "including to train our artificial intelligence models" — [Privacy](https://inworld.ai/privacy)
- Privacy Notice retention: biometric data is kept "for up to three years after you accessed your
  account, whichever occurs first", or earlier once the purpose is met. The extracted sentence looks
  truncated; read it in the original. Other personal data is kept "for as long as necessary" —
  [Privacy](https://inworld.ai/privacy)
- Security page: "Voice information extracted from your recordings is kept until its purpose is
  met." — [Security](https://inworld.ai/security)
- Privacy Notice scope: it "does not apply to information we process on behalf of our enterprise
  customers". That data falls under the DPA — [Privacy](https://inworld.ai/privacy)

### Inferences
- For Diceri, every reference recording sent for cloning is retained by Inworld, on any plan
  including Developer with ZDR, at least as long as the voice exists. ZDR only protects the
  translated text and synthesized audio, which still matters because those are transcripts of
  the call (CLAUDE.md §9 forbids logging them). Diceri therefore needs at least the Developer plan
  with ZDR enabled for the workspace, and must delete each clone at call end.
- The "improve the Services" licence and the Privacy Notice "unless you opt out" wording leave
  room for voice data to be used for service improvement beyond pure processing. Diceri should get
  written confirmation, ideally in the DPA or an order form, that customer voice samples are used
  only to provide the service and never for training or improvement.
- If Diceri's end users are the data subjects and Diceri is the controller, the DPA (processor
  terms), not the consumer Privacy Notice, should govern. The DPA's text does not mention training.

### Gaps
- No page states the default (non-ZDR) logging retention for TTS text and audio.
- No page states how "opt out" of biometric-data use for improvement works for API customers.
- The ZDR model coverage (TTS-1.5 only, or TTS-2 as well) is inconsistent between snapshots.

## 2. Voice cloning consent, prohibited uses, impersonation, watermarking

### Takeaway
Inworld puts all consent responsibility on the customer. The only gate is a Portal checkbox
("Confirm you have the rights to clone the voice"), and the clone API has **no consent or
attestation field**. The AUP bans impersonation without consent and requires AI disclosure. No
document mentions per-language consent rules or audio watermarking.

### Cited Findings
- Portal flow: "Confirm you have the rights to clone the voice, then click "Continue"." No identity
  or speaker verification is described. Instant cloning is "available to all users through Portal or
  via API" — [Voice cloning docs](https://docs.inworld.ai/tts/voice-cloning)
- The clone API request body has no consent, attestation or rights-confirmation field. It accepts
  base64 WAV/MP3 ("works best with a 10-30 sec audio clip; longer clips will be cut off at 30 sec"),
  an optional transcription, `removeBackgroundNoise`, and `languageCode` (or `"auto"`) —
  [Clone voice API](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/clone-voice.md)
- Service Specific Terms §1(b): clones may sound like "your voice or a voice you are authorized to
  share with us". §1(f): "You may not provide Input or create Output for which you do not have all
  the rights necessary to grant us" the licence —
  [Service Specific Terms](https://inworld.ai/service-specific-terms)
- AUP: no "Impersonating another individual or organization without consent or legal right". No
  "Create or expand facial, voice, or other biometric recognition databases without consent".
  "Always clearly and prominently disclose to users they are interacting with AI rather than a
  human". No sharing output to "defraud, scam, spam, mislead". No "subliminal, manipulative, or
  deceptive techniques". Users must assess their own classification under "Applicable AI Laws"
  including the EU AI Act. The AUP does not mention voice cloning, deepfakes or watermarking by
  name — [AUP](https://inworld.ai/aup)
- Inworld guidance (blog, 2026-09-08, not contractual): "Consent should be explicit, documented, and
  scoped to languages, channels, use cases, duration". "Employment or public availability does not
  automatically grant cloning rights". "Store proof of consent and rights". "Provide a revocation
  process". "Voice cloning requires authorization from the voice owner and legal review appropriate
  to the jurisdiction and use case" —
  [Voice design vs voice cloning](https://inworld.ai/resources/voice-design-vs-voice-cloning)
- ToS §1: users "must be at least 18 years old". This applies to the account holder (Diceri) —
  [Terms](https://inworld.ai/terms)
- Watermarking: no Inworld page found that says TTS output is watermarked. The launch blog only
  mentions collaborating on "safety standards" —
  [Introducing Inworld TTS](https://inworld.ai/blog/introducing-inworld-tts); also absent from the
  [AUP](https://inworld.ai/aup) and [Service Specific Terms](https://inworld.ai/service-specific-terms)

### Inferences
- Diceri's existing per-call opt-in (ADR 0006) is the consent mechanism Inworld expects. Each
  speaker clones only their own voice for their own translated speech, which fits "your voice". Keep
  a consent record (who, when, which call, scope: languages and duration = this call), to match
  Inworld's guidance and the §1(f) warranty.
- The AUP AI-disclosure rule means listeners must be told the translated voice is synthetic. The
  UI should label it.
- Because no watermark is documented, Diceri cannot rely on provider-side provenance.
- Minors: Inworld's 18+ rule binds the account holder. Inworld's guide says instant cloning "may not
  perform well" for children's voices. Diceri should decide its own policy for minors' voices under
  LGPD Art. 14 and GDPR Art. 8.

### Gaps
- No contractual per-language consent requirement found. "Scoped to languages" appears only in
  non-binding guidance.
- No Inworld statement on whether voice data is "biometric" for API customers. The Voice Library
  Addendum treats it as a biometric identifier, but only for shared voices.

## 3. DPA, subprocessors, regions, transfers, certifications

### Takeaway
A public DPA (Inworld as processor, SCCs Module 2/3, UK Addendum) is incorporated for customers
using the Services. It has no LGPD or Brazil clause. The subprocessor list and security details
live at trust.inworld.ai, which renders by JavaScript and could not be read. Data is in the US by
default; EU and India regional deployments are **Enterprise-only**. SOC 2 Type II is claimed on
every plan.

### Cited Findings
- DPA: "Inworld AI acts as a Data Processor on the Customer's behalf". It is incorporated into the
  Services Agreement: "accepting the Order Form, or using the Services, Customer agrees to this
  Agreement". The page does not restrict it to Enterprise. Last updated March 11, 2026 —
  [DPA](https://inworld.ai/data-processing-addendum)
- Transfers: EEA/Swiss under SCCs "as amended by the EU Addendum" (Module Two
  controller-to-processor, Module Three referenced; Clause 17 under the exporter's Member State law).
  UK under SCCs "as amended by the UK Addendum". There is no mention of the EU-US Data Privacy
  Framework or Brazil/LGPD. The text has internal inconsistencies: Schedule 1 §8.1 refers to "EU Data
  under Section 4.2" while §4.2 covers UK data, and "EU Addendum" is undefined —
  [DPA](https://inworld.ai/data-processing-addendum)
- Subprocessors: "Sub-Processor List" means the list at https://trust.inworld.ai/. Changes are
  notified "via blog post, notification within the Services or other reasonable means, or via
  email". There is a 30-day objection window, after which either party may terminate with a refund
  of prepaid fees — [DPA](https://inworld.ai/data-processing-addendum)
- The trust center (https://trust.inworld.ai/) returned only the title "Trust Center" (client-side
  rendered). Subprocessor names, locations and the SOC 2 report could not be retrieved. It may need
  a browser or access request — [Trust Center](https://trust.inworld.ai/)
- Breach notice: "without undue delay after becoming aware of any Personal Data Breach", with no
  hour count. Audits are allowed once a year at the customer's expense, or Inworld may provide
  report summaries instead — [DPA](https://inworld.ai/data-processing-addendum)
- Deletion at end of contract: "at Customer's instruction, return or delete Customer Data", unless
  law requires retention. No timeline is given — [DPA](https://inworld.ai/data-processing-addendum)
- Certifications: "SOC 2 Type II certified, HIPAA and GDPR compliant". There is a yearly independent
  SOC 2 audit. A BAA is available on Enterprise or as a Growth add-on. ISO is not mentioned —
  [Security](https://inworld.ai/security)
- Regions: US by default. "Fully isolated regional deployments store and process inputs and outputs
  in-region" for the EU and India on Enterprise plans. Regional workspaces, API keys and voices do
  not carry over between regions — [Security](https://inworld.ai/security)
- Encryption: "Data is encrypted at rest with AES and in transit with TLS" —
  [Security](https://inworld.ai/security)
- Privacy Notice: data may go to jurisdictions "that may or may not provide equivalent levels of
  data protection". There is no SCC or DPF mention, no LGPD or Brazil mention, contact is
  privacy@inworld.ai, and no DPO is named — [Privacy](https://inworld.ai/privacy)
- Pricing lists "SLA & DPA" only under Enterprise, while the DPA page says it applies to anyone
  "using the Services" — [Pricing](https://inworld.ai/pricing) vs
  [DPA](https://inworld.ai/data-processing-addendum)
- Inworld is itself listed as a LiveKit subprocessor for model inference —
  [LiveKit subprocessors](https://livekit.com/legal/subprocessors)

### Inferences
- On non-Enterprise plans, EU and Brazilian users' voice samples go to the US. For GDPR, the
  SCCs in the public DPA give a transfer mechanism. For LGPD (Art. 33), Brazil's ANPD standard
  contractual clauses (Resolution CD/ANPD 19/2024) are not in the DPA. Diceri would need to rely on
  another Art. 33 basis, such as specific consent, or negotiate ANPD clauses (likely Enterprise).
  This is a legal reading, not verified with counsel.
- The pricing/DPA mismatch (DPA "Enterprise" vs auto-incorporated) should be confirmed in writing
  with Inworld before launch.

### Gaps
- Subprocessor list, hosting cloud and SOC 2 report: behind the JS-rendered trust center, not
  retrieved.
- No LGPD statement anywhere.

## 4. Commercial licence for generated audio

### Takeaway
Output is assigned to the customer, and "Commercial license" is listed from the free On-Demand
tier up. However, the ToS grants service use "solely for your internal business purposes",
non-transferable and non-sublicensable. There is no explicit clause allowing or forbidding
embedding Inworld in a product used by third-party end users, and no per-plan licensing
difference was found.

### Cited Findings
- ToS §4: "we assign to you all of our right, title, and interest in Outputs". Inputs stay the
  customer's — [Terms](https://inworld.ai/terms)
- Service Specific Terms §1(c): "you retain all rights in and to your Output". Output "does not
  include, Inworld AI's foundational and other artificial intelligence voice models". §1(a): Output
  may be used "outside of the Services but always subject to these Terms and our Acceptable Use
  Policy" — [Service Specific Terms](https://inworld.ai/service-specific-terms)
- §1(e): if the customer owns IP in User Voice Models, it grants Inworld "a license to use,
  reproduce, modify" them — [Service Specific Terms](https://inworld.ai/service-specific-terms)
- ToS §3(a): the licence is "solely for your internal business purposes and as permitted by the
  documentation", "non-transferable, non-sublicensable". There is no explicit resale or competing-
  product clause — [Terms](https://inworld.ai/terms)
- ToS §13: "Upon termination, your license ends, and you must delete all Services, Models and
  Outputs" — [Terms](https://inworld.ai/terms)
- ToS §11: the customer indemnifies Inworld for its use, Materials, or violations of the Terms/AUP.
  California law, Santa Clara courts (§14) — [Terms](https://inworld.ai/terms)
- "Commercial license" is listed on On-Demand and inherited by all higher plans —
  [Pricing](https://inworld.ai/pricing)

### Inferences
- Serving live translated speech to call participants is how a TTS API is normally used, and the
  docs ("as permitted by the documentation") describe API integration. Still, "internal business
  purposes" is ambiguous for a public product, so ask Inworld to confirm. The §13 duty to delete
  Outputs on termination is harmless for Diceri, which does not store audio.
- Diceri indemnifies Inworld for misuse, which strengthens the case for keeping consent records.

### Gaps
- No plan-by-plan licence differences found beyond the pricing checkmark.

## 5. Deleting voices, accounts and workspaces

### Takeaway
A private instant clone can be deleted via `DELETE /voices/v1/voices/{voiceId}`, which "frees space
right away". No document says whether the stored reference audio is purged immediately or
retained afterwards. Shared Voice Library voices have a 2-year notice period, so **never share
Diceri clones to the library**.

### Cited Findings
- The delete endpoint is described only as "delete a custom voice from your workspace". It
  returns `{}`. It says nothing about purging samples, retention windows or irreversibility —
  [Delete voice API](https://docs.inworld.ai/api-reference/voiceAPI/voiceservice/delete-voice.md)
- "Private voices can be deleted via API, and deletion frees space right away". A voice shared to
  the Voice Library cannot be deleted until removed, and "removal has a 2-year notice period".
  Instant clones share the plan's custom-voice cap across workspaces, and a 400 is returned at the
  cap — [Voice cloning docs](https://docs.inworld.ai/tts/voice-cloning)
- Voice Library Addendum (shared "Community Voices" only): the creator gives express consent to the
  "collection, storage, use, and disclosure of such biometric identifier". The Notice Period is
  "currently 2 years" and cannot be shortened. Inworld gets "a worldwide, royalty-free, sublicensable
  license" to let other users generate audio —
  [Voice Library Addendum](https://inworld.ai/voice-library-addendum)
- Existing logged content is deleted only via support. Enabling ZDR does not delete it —
  [ZDR docs](https://docs.inworld.ai/tts/zero-data-retention)
- Account end: the DPA promises return or deletion "at Customer's instruction" with no timeline —
  [DPA](https://inworld.ai/data-processing-addendum). The Privacy Notice keeps biometric data up to
  three years or until the purpose is met — [Privacy](https://inworld.ai/privacy)

### Inferences
- Diceri can meet its "clones are deleted when the call ends" rule at the API level, and doing so
  also stops the plan voice cap filling up. It cannot currently prove the reference audio is purged.
  Ask Inworld for a written purge SLA, for example in the DPA, covering samples and derived
  embeddings.

### Gaps
- No documented purge timeline for reference audio or embeddings after `DELETE`. No backup
  retention window. These would need a direct question to privacy@inworld.ai or sales.
- Workspace and organization deletion behavior is not documented in the pages reviewed.
