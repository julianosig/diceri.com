# Low-Latency Streaming Voice-Cloning TTS for a Live Multilingual Video-Call App (state as of 25 Nov 2026)

Scope: re-synthesizing each speaker's translated speech in the listener's language, in a voice that sounds like the original speaker. Research date: 2026-11-25. All prices are list prices observed Sep–Nov 2026 unless stated. "Vendor claim" = number published by the vendor; "Independent" = third-party measurement (Artificial Analysis, Coval, etc.).

Unit used for cost conversion: ~850 characters per minute of synthesized speech (the figure Fora Soft uses in its Nov 2026 cost model — [Fora Soft, 18 Nov 2026](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)). $/min = ($ per 1M chars) x 0.00085.

---

## 1. What ElevenLabs actually offers now (verifying "ElevenLabs v4 Turbo")

### Takeaway
"Eleven v4 Turbo" is real: ElevenLabs launched Eleven v4 and Eleven v4 Turbo on 28 Sep 2026. v4 Turbo (`eleven_v4_turbo`) is the realtime/agent variant, 90+ languages, priced the same as Flash v2.5 ($0.04/1K chars) and it sits at or near the top of Artificial Analysis quality boards. Turbo v2.5 is now deprecated; Flash v2.5 is still listed as the fastest model in their table.

### Cited Findings
- Current TTS model table (ElevenLabs docs, accessed 25 Nov 2026): `eleven_v4` (flagship, 90+ languages, 100,000 char limit), `eleven_v4_turbo` (realtime variant of v4, 90+ languages, ~100 ms latency, Text-to-Dialogue WebSocket), `eleven_v3` (70+ languages, "previous generation"), `eleven_v3_conversational` (70+ langs, ~165 ms, previous generation), `eleven_multilingual_v2` (29 langs), `eleven_flash_v2_5` (32 langs, ~100 ms), `eleven_flash_v2` (English only, ~135 ms). Latency figures are vendor medians excluding application and network latency — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
- `eleven_turbo_v2_5` and `eleven_turbo_v2` are **deprecated** ("functionally equivalent to Flash v2.5; replace with `eleven_flash_v2_5`"), no removal date given — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
- ElevenLabs also now lists speech-to-speech "Realtime" agent models `eleven_realtime_v1_mini` and `eleven_realtime_v1_max` (90+ languages, OpenAI-compatible Realtime API over WebSocket). These bundle ASR+LLM+voice; they are agent models, not a pure TTS endpoint — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
- Launch (28 Sep 2026): v4 for produced audio, v4 Turbo for voice agents; available in ElevenCreative, ElevenAgents and the API. Turbo accepts text as an LLM generates it and returns audio before the sentence is complete. Request length up to 10,000 chars (secondary source; note conflict with the docs' 100,000-char figure for v4) — [Progressive Robot, 28 Sep 2026](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/)
- Cloning on v4: instant clones need ~10 s of audio; Professional Voice Clones are supported again (they were unavailable on v3); older clones reportedly need retraining for v4; cloning requires verified owner consent (via TestingCatalog). Cross-lingual: PVCs keep emotional range across languages and "an instant clone can speak other languages with a native accent" (vendor claim relayed by press) — [Progressive Robot](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/)
- Vendor latency claim: v4 Turbo median time-to-first-speech 150 ms vs 262 ms for Cartesia Sonic 3.6 and 814 ms for OpenAI gpt-4o-mini-tts, measured by ElevenLabs in Sep 2026 over WebSocket with network removed — [search summary of ElevenLabs/Inworld coverage](https://inworld.ai/resources/elevenlabs-v3-review); also reported in [Progressive Robot](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/)
- API pricing (page accessed 25 Nov 2026, "same on every plan"): v4 $0.08/1K chars; v4 Turbo $0.04; Flash/Turbo v2.5 $0.04; v3 $0.08; Multilingual v2 $0.08. Plans: Free $0, Starter $6, Creator $22, Pro $99, Scale $299, Business $990 (12.375M v4-class chars included); Enterprise custom. Concurrency for Realtime/Speech Engine (mapped by row order, inferred): Free 4, Starter 6, Creator 10, Pro 20, Scale 30, Business 40 — [ElevenLabs API pricing](https://elevenlabs.io/pricing/api)
- Launch promo: v4 $0.022/1K and v4 Turbo $0.011/1K until 12 Oct 2026 (expired) — [Progressive Robot](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/)
- Artificial Analysis provider-voice board at launch (29 Sep 2026): v4 #1 at Elo 1,319 (±19) — [Progressive Robot](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/). Nov 2026 snapshots: v4 Turbo 1340, v4 1338 — [Fora Soft, 18 Nov 2026](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology); other mirrors show v4 1340 / v4 Turbo 1339 or v4 Turbo 1345 — [Artificial Analysis leaderboard (via search)](https://artificialanalysis.ai/text-to-speech/leaderboard/provider-voice)
- Vendor claim: v4 "preferred by ~75% of listeners in blind head-to-head tests" (ElevenLabs' own test) — [Progressive Robot](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/)
- Consent: ElevenAPI requires a consent attestation for every clone; PVC adds a spoken verification statement. Instant Voice Clones are not shareable; voice deletion is permanent — [ElevenLabs voice cloning API blog](https://elevenlabs.io/blog/voice-cloning-api); [ElevenLabs Voice Library docs](https://elevenlabs.io/docs/help-center/product/voices/voice-library)

### Inferences
- The user's "v4 turbo" choice is a legitimate current model, not a misremembering. For a call app, `eleven_v4_turbo` dominates `eleven_flash_v2_5` (same price, ~same measured TTFA, far higher quality, 90+ vs 32 languages).
- ~$0.034 per output minute at list ($0.04/1K x 850 chars). Volume discounts only via Enterprise.

### Gaps
- Per-plan instant-clone quotas and exact create/delete API endpoints for v4 were not confirmed from primary docs in this pass (historically `POST /v1/voices/add` and `DELETE /v1/voices/{id}`; verify).
- Supported WebSocket output formats/sample rates for v4 Turbo not confirmed this pass (historically PCM 16/22/24/44.1 kHz, mu-law 8 kHz, MP3; verify).
- Whether a ~10 s instant clone captured live in a call yields good v4 similarity is not independently measured.

---

## 2. Benchmarks: quality Elo, cloning Elo, latency, SIM/WER

### Takeaway
On Artificial Analysis' **voice-cloning** ("Controlled Voice") board, Alibaba Qwen-Audio-3.1-TTS-Plus leads (1210), followed by HeyGen Voice, Speechify Simba 3.5, HeyGen Voice Turbo; ElevenLabs v4/v4 Turbo, Cartesia Sonic 3.6 and Inworld Realtime TTS-2 form a statistically tied tier (~1150–1158). Independent TTFA (Coval, 30-day avg incl. network) puts ElevenLabs Flash v2.5/v4 Turbo at ~215–219 ms and Sonic 3.6 at ~288 ms, i.e. vendor "75–100 ms" claims are model-only numbers. The cloning board is **English-only** — no independent cross-lingual clone benchmark exists.

### Cited Findings — Artificial Analysis Controlled Voice (cloning) board, accessed 25 Nov 2026 (8 cloned voices, 4 US + 4 UK, English only; Elo anchored at OpenAudio S1 = 1000; price per 1M chars)
- Qwen-Audio-3.1-TTS-Plus (Alibaba, Sep 2026) 1210 ±18, $19.31 — [Artificial Analysis Controlled Voice](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- HeyGen Voice (Oct 2026) 1190 ±12, $30; HeyGen Voice Turbo 1184 ±14, $4.00 — [same](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- Speechify Simba 3.5 (Nov 2026) 1186 ±16, $6.58 — [same](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- Qwen-Audio-3.1-TTS-Flash 1171, $19.31 — [same](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- Eleven v4 Turbo 1158 ±11, $40; Eleven v4 1157, $80; Cartesia Sonic 3.6 (Aug 2026) 1153, $49; Inworld Realtime TTS-2 (Aug 2026) 1150, $20.83 — all within overlapping CIs (rank 6–9) — [same](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- StepFun StepAudio 3 TTS 1136 ($14); Cartesia Sonic 3.5 1118; Smallest.ai Lightning V3.1 Pro 1104 ($19.50); AssemblyAI-TTS-1 1089 ($15); Eleven v3 1076 ($100); Ant Group Ming-3.0-tts 1073 (open, N/A); Fish Audio Drama 3 1071 ($15); ElevenLabs v3 Conversational 1068 ($50); Inworld TTS-2 Flash 1067 ($10.42); Mistral Voxtral 3 TTS 1066 ($13); Gemini 3.8 Flash-Lite TTS 1058 ($11.03); Gemini 3.8 Flash TTS 1055 ($16.49); MiniMax Speech 2.8 HD 1031 ($100) / Turbo 1027 ($60); Fish Audio S2.1 Pro 1009; Eleven Flash v2.5 986 ($50); Higgs Audio V3 965; Cartesia Sonic 3 962; NVIDIA Magpie TTS Zeroshot 944; Resemble Chatterbox 942 ($25); XTTS v2 847; OpenVoice v2 827 — [same](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- Note the AA price column lists Eleven v4 Turbo at $40/1M and Flash v2.5 at $50/1M while ElevenLabs' own pricing page lists both at $0.04/1K ($40/1M) — minor discrepancy — [AA](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice) vs [ElevenLabs pricing](https://elevenlabs.io/pricing/api)

### Cited Findings — general quality (provider-voice) board
- Nov 2026: Eleven v4 Turbo 1340, Eleven v4 1338, Qwen-Audio-3.1-TTS-Plus 1298, Cartesia Sonic 3.6 1295 — [Fora Soft, 18 Nov 2026](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology); Gemini 3.8 Flash TTS ~1283; Smallest Lightning V3.1 Pro 1279 — [AA provider-voice (via search)](https://artificialanalysis.ai/text-to-speech/leaderboard/provider-voice)
- Open-weights board: Fish Audio Drama 3 1277, Ant Group Ming-3.0-tts 1179–1188 (Oct 2026) — [AA open-weights board (via search)](https://artificialanalysis.ai/text-to-speech/leaderboard/provider-voice/open-weights); [micdrop.dev](https://micdrop.dev/blog/top-open-source-text-to-speech-models)
- Cartesia Sonic 3.6 briefly led both AA arenas at its Aug 2026 launch (since overtaken by Eleven v4) — [MarkTechPost, 18 Aug 2026](https://www.marktechpost.com/2026/08/18/cartesia-ships-sonic-3-6-a-streaming-tts-model-that-now-leads-both-artificial-analysis-speech-arenas/)

### Cited Findings — latency (Independent: Coval, 30-day average TTFA of hosted endpoints, page "last measured 2026-11-25")
- Rank 12 ElevenLabs Flash v2.5 215 ms; rank 14 Eleven v4 Turbo 219 ms; Speechify Simba 3.5 110 ms; Simba 3.2 102 ms; Qwen3 TTS Fast 77 ms; Qwen3 TTS 1.7b 105 ms; "TTS Flash 2" 68 ms; "TTS 2" 164 ms (labels likely Inworld TTS-2 Flash / TTS-2 but not confirmed); AssemblyAI TTS 1 218 ms; Cartesia Sonic 3.6 288 ms; Deepgram Aura 2 315 ms; Smallest Lightning v3.1 Pro 320 ms; Fish drama3-preview 329 ms; Fish S2.1 Pro 378 ms; Google Chirp 3 HD 565 ms; Qwen3 TTS Flash Realtime 722 ms — [Coval benchmarks](https://benchmarks.coval.ai/providers/elevenlabs)
- Another Oct 2026 Coval-based comparison: v4 Turbo 192 ms median (31 ms spread), Flash v2.5 188 ms — [search summary citing Coval](https://benchmarks.coval.ai/providers/elevenlabs)
- Cartesia independent measurement: median 166 ms first audio incl. network (secondary blog); Cartesia's own changelog cited at ~190 ms median; marketing "sub-90 ms" is model latency — [Invideo blog](https://invideo.io/blog/cartesia-sonic-ai-voice/)
- Inworld TTS-2: vendor 100 ms P90 server-side TTFB at high concurrency; TTS-2 Flash 20 ms (excl. network) — [Inworld TTS models docs](https://docs.inworld.ai/tts/tts-models)
- Hume Octave 2 (preview): vendor ~100 ms model latency (Octave 1 ~200 ms) — [Hume TTS docs](https://dev.hume.ai/docs/text-to-speech-tts/overview)

### Cited Findings — SIM/WER (papers; different test sets, not directly comparable)
- Qwen3-TTS (open, Jan 2026): beats CosyVoice series on cross-lingual cloning; zh→ko error 4.82 vs 14.4 for CosyVoice3; claims best speaker similarity across all 10 evaluated languages — [Qwen3-TTS Technical Report, arXiv 2601.15621](https://arxiv.org/html/2601.15621v1)
- Qwen-Audio-3.0-TTS: CV3-Eval cross-lingual average error 4.05% vs 10.09% for CosyVoice3-1.5B — [arXiv 2607.23938](https://arxiv.org/html/2607.23938)
- IndexTTS 2.5: Chinese-prompt cross-lingual avg WER 6.22, SIM 68.62 (best average), though CosyVoice3-0.5B wins some pairs — [arXiv 2601.03888](https://arxiv.org/html/2601.03888v5)
- CosyVoice 3: 1.5B better WER than 0.5B at similar SIM in cross-lingual eval — [arXiv 2505.17589](https://arxiv.org/pdf/2505.17589)
- Typical similarity ranges (ECAPA-TDNN cosine, practitioner guidance): zero-shot 3–10 s sample 0.70–0.82; instant 30 s–2 min 0.80–0.88; PVC 30+ min 0.90–0.96; targets ≥0.75 zero-shot, WER <5% for agents — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)

### Inferences
- For a call app, any of v4 Turbo / Sonic 3.6 / Inworld TTS-2 is within the same cloning-quality CI; the decision should rest on cross-lingual behavior, price and latency, not Elo.
- Qwen-Audio-3.1, HeyGen Voice and Speechify Simba 3.5 lead English cloning but their realtime WebSocket APIs, consent flows and multilingual cloning were not verified here.
- Real-world TTFA incl. network ~200–300 ms for all leading hosted options; TTS is not the dominant latency term vs ASR endpointing + MT.

### Gaps
- No independent benchmark of **cross-lingual** cloning (e.g., Portuguese reference → English output) for any commercial API. AA's cloning board is English-only. Recommend an in-house A/B: 10 s PT/ES/FR references → EN output, scored with ECAPA SIM + Whisper WER + listener MOS.
- HeyGen Voice and Simba 3.5 API latency/streaming/consent details not researched.

---

## 3. Other vendors: status and cloning support

### Takeaway
Cartesia Sonic 3.6 and Inworld TTS-2 are the strongest low-latency alternatives with open instant-cloning APIs. OpenAI, Azure and Google Chirp 3 cloning are gated (eligible/approved/allow-list customers only). Gemini 3.8 Flash TTS now clones from 30 s with a consent recording but is geo-restricted. PlayHT is shut down.

### Cited Findings
- **Cartesia Sonic 3.6**: `sonic-3.6` alias, stable snapshot `sonic-3.6-2026-08-27`; 44 languages (en, fr, de, es, pt, zh, ja, hi, it, ko, nl, pl, ru, sv, tr, tl, bg, ro, ar, cs, el, fi, hr, ms, sk, da, ta, uk, hu, no, vi, bn, th, he, ka, id, te, gu, kn, ml, mr, pa, or, ur); PVCs from older models work on 3.6 — [Cartesia Sonic 3.6 docs](https://docs.cartesia.ai/build-with-cartesia/tts-models/latest)
- Cartesia changelog: Instant Voice Clones can now speak several languages natively; existing clones keep IDs; new watermark applied to all Sonic-3.6 audio; self-hosted option exists for customers — [Cartesia Changelog 2026](https://docs.cartesia.ai/changelog/2026) (via search summary)
- Cartesia cloning: instant from ~10 s (up to 60 s); PVC 30+ min; price $0.038–$0.065/1K chars depending on plan (Oct 2026); "instant clones keep accent across languages" — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology); Pro plan $5/mo 100K credits (~$50/1M), cloning not on free tier — [eesel.ai](https://www.eesel.ai/blog/cartesia-sonic-3-pricing)
- **Inworld Realtime TTS-2** (Aug 2026): $25/1M on-demand (TTS-2), $15/1M (TTS-2 Flash), enterprise as low as $5/1M; voice cloning free; clone from 3–15 s (sources differ: 5–15 s in launch PR, 3 s on cloning page); languages claimed 200+ locales but other languages "experimental" on TTS-2 per one source — [Inworld pricing](https://inworld.ai/pricing); [Inworld voice cloning](https://inworld.ai/voice-cloning); [Inworld launch PR](https://sg.finance.yahoo.com/news/inworld-launches-realtime-tts-2-160000791.html); [OrcaRouter](https://www.orcarouter.ai/blog/inworld-realtime-tts-2-flash-launch)
- **OpenAI**: custom voices usable with TTS API, Realtime API, GPT-Live and Chat Completions audio; recording-based custom voices limited to **eligible customers**; require separate consent recording that must match the voice; no children's voices; disclosure of AI voice required — [OpenAI Custom voices guide](https://developers.openai.com/api/docs/guides/custom-voices); third-party: max 20 custom voices/org, 30 s max sample (unverified) — [openaifm.org](https://www.openaifm.org/openai-tts-custom-voice)
- **Azure Personal Voice**: 5–90 s + spoken consent; approved customers only; ~$0.024/1K chars — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- **Google Chirp 3 HD Instant Custom Voice**: fixed consent script recording (≤10 s, matching language), allow-list access, 30+ locales (ja-JP added), cross-language transfer from en-US to de-DE, es-US, es-ES, fr-CA, fr-FR, pt-BR; ~$0.06/1K — [Google Chirp 3 ICV docs](https://docs.cloud.google.com/text-to-speech/docs/chirp3-instant-custom-voice); [Google release notes](https://docs.cloud.google.com/text-to-speech/docs/release-notes); [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology). Independent TTFA 565 ms — [Coval](https://benchmarks.coval.ai/providers/elevenlabs)
- **Gemini 3.8 Flash TTS / Flash-Lite TTS** (GA 22 Sep 2026): replicate an authorized voice from a 30 s sample with verbal consent recording checked against the reference; SynthID watermark + C2PA; voice replication via AI Studio excluded in Illinois, Texas, EEA, UK, Switzerland, India (API regional limits unclear); audio output $9/1M tokens rising to $18 from 1 Jan 2027 (third-party) — [The Next Web](https://thenextweb.com/news/gemini-tts-3-8-flash-voice-design-cloning); [Mixed News](https://mixed-news.com/en/gemini-3-8-flash-tts-voice-cloning-30-second-sample/); [Digital Applied](https://www.digitalapplied.com/blog/gemini-3-8-flash-tts-voice-cloning-price-doubles-january)
- **Hume Octave 2**: clone from 15 s; availability depends on subscription tier (one review says API cloned voices Enterprise-only) — [Hume voice cloning docs](https://dev.hume.ai/docs/voice/voice-cloning); [Hume Octave 2 launch](https://hume.ai/blog/octave-2-launch)
- **MiniMax Speech 2.8** HD/Turbo: cloning Elo 1031/1027 at $100/$60 per 1M — expensive for its rank — [AA Controlled Voice](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- **Fish Audio**: Drama 3 (Oct 2026) tops open-weights quality board, $15/1M hosted; community licence, free commercial use only under $500k annual revenue; 83 languages — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology); [AA](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- **PlayHT**: acquired by Meta (July 2025, per most sources) and shut down ~31 Dec 2025; clones and accounts deleted — [Infrabase](https://infrabase.ai/audio/playht); [aipedia](https://aipedia.wiki/dead/play-ht/) (low-authority sources, dates vary)
- **Deepgram Aura 2**: TTFA 315 ms (Coval); cloning support not confirmed — [Coval](https://benchmarks.coval.ai/providers/elevenlabs)

### Inferences
- Gated providers (OpenAI, Azure, Google Chirp 3) are impractical for ad-hoc cloning of arbitrary call participants without an enterprise approval process; Gemini's 30 s minimum and geo-exclusions (EEA/UK/India/IL/TX) are a bad fit for a global call app.

### Gaps
- Rime, Sesame, Resemble hosted API, Deepgram cloning, HeyGen Voice API details were not researched in this pass.

---

## 4. Cloning workflow, live capture, consent/ToS, deletion

### Takeaway
All major APIs can create an instant clone from ~3–30 s via API, so capturing a few seconds live in a call is technically feasible (ElevenLabs ~10 s, Cartesia ~10 s, Inworld 3–15 s, Hume 15 s, Gemini 30 s). But every vendor requires the voice owner's consent (attestation or recorded consent statement), and EU AI Act Art. 50 transparency duties have applied since 2 Aug 2026. Silently cloning "anonymous" participants is not permissible; build explicit opt-in that doubles as the reference capture.

### Cited Findings
- Reference audio: ElevenLabs instant ~10 s (v4) — [Progressive Robot](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/); Cartesia 10 s (up to 60 s) — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology); Inworld 3–15 s — [Inworld](https://inworld.ai/voice-cloning); Hume 15 s — [Hume](https://dev.hume.ai/docs/voice/voice-cloning); Gemini 30 s — [TNW](https://thenextweb.com/news/gemini-tts-3-8-flash-voice-design-cloning); OpenAI 10–30 s + consent, Azure 5–90 s + consent, Chirp 3 ~10 s + consent — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- ElevenLabs: consent attestation required on every API clone; IVCs not shareable; deletion permanent — [ElevenLabs blog](https://elevenlabs.io/blog/voice-cloning-api); [Voice Library docs](https://elevenlabs.io/docs/help-center/product/voices/voice-library)
- EU AI Act Art. 50 transparency duties apply since 2 Aug 2026 (machine-readable marking; deployers must disclose deepfake audio incl. cloned voices); Digital Omnibus grace period to 2 Dec 2026 only for systems on market before 2 Aug 2026; fines up to €15M / 3% turnover — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- US: Illinois BIPA treats voiceprints as biometrics ($1,000/$5,000 per violation, per-person since Aug 2024 amendment); Tennessee ELVIS Act; California AB 2602/AB 1836; NY synthetic-performer ad disclosure since 9 Jun 2026; NO FAKES Act not law (UC vote blocked 30 Sep 2026) — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- Denmark bill L 49 (tabled 8 Oct 2026) for rights over own voice/likeness, proposed 1 Jan 2027 — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- Recommended practices: written consent with scope/expiry/revocation, liveness challenge phrase at capture, registry check per inference (fail closed on revoked), watermarks (AudioSeal, Perth, ElevenLabs) + C2PA, audit logs 12–36 months — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- Watermarking built in: Cartesia Sonic 3.6 watermark on all audio — [Cartesia changelog](https://docs.cartesia.ai/changelog/2026); Gemini SynthID + C2PA — [TNW](https://thenextweb.com/news/gemini-tts-3-8-flash-voice-design-cloning)

### Inferences
- Product flow: on joining, user opts into "speak in my voice", reads a short consent + liveness sentence (~10–15 s) which is used as the clone reference; clone is session-scoped and deleted via API at call end (or retained only with explicit opt-in). Fallback for non-consenting users: a gender/age-matched stock voice. Avoid cloning in Illinois/Texas/EEA without written consent; this also sidesteps BIPA voiceprint retention issues.
- Session-scoped clones avoid hitting per-account voice-count quotas (unverified for most vendors) — design must assume a cap and delete aggressively.

### Gaps
- Exact per-account instant-clone quotas (ElevenLabs, Cartesia, Inworld) and whether vendors allow programmatic clone-per-end-user under their ToS at scale were not confirmed; requires reading each ToS / contacting sales.

---

## 5. Streaming APIs, formats, concurrency, pricing

### Takeaway
ElevenLabs v4 Turbo, Cartesia Sonic and Inworld TTS-2 all support incremental text-in/audio-out streaming suited to MT output. Per output minute at list: Inworld TTS-2 ~$0.021 (enterprise ~$0.004), ElevenLabs v4 Turbo ~$0.034, Cartesia ~$0.032–0.055, Eleven v4 ~$0.068.

### Cited Findings
- v4 Turbo accepts streaming text and emits audio before sentence completion; Text-to-Dialogue WebSocket — [ElevenLabs docs](https://elevenlabs.io/docs/overview/models); [Progressive Robot](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/)
- Inworld TTS-2: 100 ms P90 TTFB "at high concurrency" (vendor) — [Inworld docs](https://docs.inworld.ai/tts/tts-models)
- Qwen-Audio-3.1-TTS-Flash: streaming via SSE/WebSocket on gateways; Alibaba Model Studio voice cloning ~$0.01/clone (Qwen3-TTS line); pricing moved to tokens from 22 Sep 2026 ($0.23 in / $1.87 out per 1M tokens, Singapore) — third-party, conflicting — [ai-tldr.dev](https://ai-tldr.dev/models/qwen-audio-3-1-tts/); [llmlearner](https://llmlearner.com/models/qwen-audio-3-1-tts-flash); [wavespeed](https://wavespeed.ai/blog/ai-api-pricing/qwen3-tts-api-pricing/)
- Managed cost for 500k talk-min/month (425M chars): Inworld TTS-2 ~$10,625; Cartesia Sonic 3.6 Scale ~$16,150; ElevenLabs Flash/v4 Turbo ~$17,000; ElevenLabs v3/v4 ~$34,000; OpenAI GPT-Live voice layer ~$25,000 + LLM — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- Price per 1M chars (AA list, Nov 2026): HeyGen Voice Turbo $4; Simba 3.5 $6.58; Gemini 3.8 Flash-Lite $11.03; Voxtral 3 $13; Fish $15; Gemini 3.8 Flash $16.49; Qwen 3.1 Plus $19.31; Inworld TTS-2 $20.83; Eleven v4 Turbo $40; Cartesia $49; Eleven v4 $80 — [AA](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)

### Inferences (cost per output audio minute at 850 chars/min, list price)
| Option | $/1M chars | ~$/output min |
|---|---|---|
| Inworld TTS-2 (on-demand / enterprise floor) | $25 / $5 | $0.021 / $0.004 |
| Inworld TTS-2 Flash | $15 | $0.013 |
| ElevenLabs v4 Turbo / Flash v2.5 | $40 | $0.034 |
| ElevenLabs v4 | $80 | $0.068 |
| Cartesia Sonic 3.6 | $38–65 | $0.032–0.055 |
| Qwen-Audio-3.1-TTS-Plus | $19.31 | $0.016 |
| HeyGen Voice Turbo / Simba 3.5 | $4 / $6.58 | $0.003 / $0.006 |
| Gemini 3.8 Flash TTS | $16.49 (doubling Jan 2027) | $0.014 → ~$0.028 |

- In an N-party call with M distinct listener languages, each utterance is synthesized once per target language (not per listener) — cost scales with languages, not participants.

### Gaps
- Output formats/sample rates (PCM/Opus/mu-law) and per-plan concurrency for v4 Turbo, Sonic 3.6 and Inworld TTS-2 were not verified from primary docs in this pass. Opus support specifically unconfirmed for all three (WebRTC will likely need PCM→Opus encode server-side).

---

## 6. Open-weight / self-host options and GPU cost

### Takeaway
Best commercially-usable open options for realtime cross-lingual cloning: Qwen3-TTS (0.6B/1.7B, open, streaming, 3 s clone, strongest published cross-lingual numbers), CosyVoice 3 (0.5B, bidirectional streaming), Chatterbox Multilingual (MIT, 23+ langs), VoxCPM2 (Apache-2.0), Ming-3.0-tts (top open-weights "ship freely" score). Avoid for commercial: F5-TTS weights (CC-BY-NC), XTTS-v2 (CPML, Coqui defunct), Fish Drama 3 above $500k revenue. Kokoro cannot clone. Self-hosting only beats managed APIs at thousands of sustained concurrent streams.

### Cited Findings
- Qwen3-TTS open sizes 0.6B/1.7B, streaming generation, voice design, clone from 3 s — [Qwen blog](https://qwen.ai/blog?id=qwen3tts-0115); cross-lingual superiority vs CosyVoice — [arXiv 2601.15621](https://arxiv.org/html/2601.15621v1). Coval TTFA: Qwen3 TTS 1.7b 105 ms, Qwen3 TTS Fast 77 ms (hosted) — [Coval](https://benchmarks.coval.ai/providers/elevenlabs)
- CosyVoice 3 (Fun-CosyVoice3-0.5B-2512): multilingual, zero-shot, bidirectional streaming — [cekura.ai](https://www.cekura.ai/blogs/open-source-tts); paper [arXiv 2505.17589](https://arxiv.org/pdf/2505.17589)
- Ming-3.0-tts (Ant Group): AA open-weights 1188 (Oct 2026), "best open model a company can ship freely"; cloning Elo 1073 — [micdrop.dev](https://micdrop.dev/blog/top-open-source-text-to-speech-models); [AA](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)
- OmniVoice (k2-fsa, 31 Mar 2026): Apache 2.0, 600+ languages zero-shot, 40x realtime; streaming not confirmed — [arXiv 2604.00688](https://arxiv.org/pdf/2604.00688); [DEV](https://dev.to/sangrokjung/omnivoice-open-source-tts-with-600-languages-and-zero-shot-voice-cloning-1mpn)
- Licences: Chatterbox MIT (Turbo/Nano/Multilingual, 23+ langs; vendor-commissioned Podonos test 63.75% preferred over ElevenLabs); VoxCPM2 Apache-2.0; XTTS-v2 CPML non-commercial; F5-TTS weights CC-BY-NC; Fish Drama 3 community licence (<$500k rev); Kokoro Apache-2.0 no cloning — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- Cross-lingual F5-TTS research variant exists — [arXiv 2509.14579](https://arxiv.org/pdf/2509.14579)
- GPU sizing (practitioner estimate): A10 ~3–5 concurrent zero-shot streams sub-300 ms; A100 10–15; H100 25+; RTX 4090 (XTTS/Chatterbox) 5–10. Self-hosted Chatterbox for 500k talk-min/month ~$8,000/month GPU only (~$0.016/min); crossover vs managed at "a few thousand sustained concurrent streams" once staffing included — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
- Cloning Elo of self-hostable models trails top APIs: Ming-3.0 1073, Higgs Audio V3 965, Magpie Zeroshot 944, Chatterbox 942, XTTS v2 847 vs ~1150–1210 for top APIs — [AA](https://artificialanalysis.ai/text-to-speech/leaderboard/controlled-voice)

### Inferences
- Rough self-host $/min (assumption: H100 ~$2.50/hr on-demand cloud, not sourced; 25 concurrent streams at 100% utilization): $2.50 / (25 x 60) ≈ $0.0017/min of GPU time; at realistic 30–40% utilization ≈ $0.004–0.006/min, plus engineering/ops. That matches Inworld enterprise ($0.004/min) — so self-hosting only pays for data-residency/privacy or very high scale, and costs ~100–200 Elo of cloning quality.
- Note: Cartesia offers a self-hosted Sonic deployment for enterprise customers (changelog), a middle path for on-prem with top-tier quality.

### Gaps
- No verified per-GPU throughput numbers for Qwen3-TTS or CosyVoice 3 streaming; no Chatterbox-Multilingual cross-lingual SIM table found.

---

## Ranked recommendation (as of 25 Nov 2026)

These are inferences from the cited findings above, not sourced facts.

1. **Primary: ElevenLabs Eleven v4 Turbo (`eleven_v4_turbo`)** — ~$0.034/output min at list (lower under Enterprise). Top-tier quality Elo (1340) and in the top cloning tier (1158), 90+ languages, streaming text input, independent TTFA ~190–220 ms, ~10 s instant clone, vendor-claimed native-accent cross-lingual speech from instant clones, consent attestation flow, mature deletion API. Confirms the user's original choice (the name is correct; don't use deprecated Turbo v2.5; Flash v2.5 is same price but clearly worse).
2. **Fallback / cost-optimized: Inworld Realtime TTS-2** — ~$0.021/min on-demand, down to ~$0.004/min enterprise; free cloning from 3–15 s; statistically tied with v4 Turbo on AA cloning (1150 vs 1158). Caveat: non-English languages reportedly "experimental" — validate cross-lingual quality per language pair. **Alternative fallback: Cartesia Sonic 3.6** (~$0.032–0.055/min, 44 languages, multilingual instant clones, watermarking, self-host option for enterprise; Coval TTFA ~288 ms).
3. **Self-host: Qwen3-TTS 1.7B (open) or CosyVoice 3 0.5B**, with Chatterbox Multilingual (MIT) as the permissive-licence choice — est. ~$0.004–0.016/min GPU-only depending on utilization (Fora Soft: ~$0.016/min for Chatterbox at 500k min/month), plus ops. Expect lower clone similarity than top APIs; justify only for privacy/residency or very high sustained concurrency.

Watch list: Qwen-Audio-3.1-TTS-Plus (#1 cloning Elo, ~$0.016/min, but API/consent/streaming via Alibaba Model Studio unverified), HeyGen Voice Turbo (~$0.003/min, #2–5 cloning), Speechify Simba 3.5 (#3 cloning, 110 ms TTFA, ~$0.006/min) — these may be the best cost/benefit if their realtime APIs and multilingual cloning hold up; run a bake-off before committing.

Must-do before choosing: an in-house cross-lingual bake-off (PT/ES/FR/DE/JA reference → EN and EN → each), measuring ECAPA SIM, Whisper WER, TTFA from your region, and blind MOS, since no independent cross-lingual cloning benchmark exists.
