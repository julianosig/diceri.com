# Multilingual and Cross-Lingual Quality of Low-Latency Streaming Voice-Cloning TTS (state as of 25 Nov 2026)

Context: live voice-translation calls. A speaker's voice is cloned from about 10 s of speech in their own language and must then speak the translation in any target language, sounding like the same person with native pronunciation. This builds on `/home/debian/diceri.com/docs/research/2026-11-voice-translation-stack/notes/voice-cloning-tts.md`. That note's current pick is ElevenLabs `eleven_v4_turbo`, with Inworld Realtime TTS-2 and Cartesia Sonic 3.6 as fallbacks. It also records English-only Artificial Analysis (AA) cloning Elo, Coval TTFA and price per 1M characters. Those numbers are not repeated here.

Labels: **[V]** marks a vendor claim or a vendor-authored paper. **[I]** marks an independent or third-party source. **[V-paper, rival]** marks a vendor paper that benchmarks competitors. Vendor-run comparisons are useful but self-interested.

---

## 1. Language lists and native/GA vs experimental tiers

### Takeaway
ElevenLabs v4 / v4 Turbo has the broadest flat list: 90+ languages with no tiers documented. Inworld TTS-2 is the only provider that publishes explicit quality tiers. Tier 1 has 15 languages, Tier 2 has 79, and Tier 3 (376 languages) is best-effort only. Cartesia Sonic 3.6 covers 44 languages with no tiers. Hume Octave 2 (11 languages) appears to have had its API sunset on 13 Nov 2026, so it should be dropped.

### Cited Findings
- **ElevenLabs `eleven_v4` and `eleven_v4_turbo`** [V]: the same 90+ language list, given as ISO codes. It includes af, am, ar, hy, as, az, be, bn, bs, bg, my, yue (Cantonese), ca, ceb, hr, cs, da, nl, en, et, fil, fi, fr, gl, ka, de, el, gu, ha, he, hi, hu, is, id, it, ja, jv, kn, kk, ko, ky, lo, lv, lt, lb, mk, ms, ml, mt, zh, mi, mr, mn, ne, no, ps, fa, pl, pt, pa, ro, ru, sr, sd, sk, sl, so, ckb, es, sw, sv, tg, ta, te, th, tr, uk, ur, uz, vi, cy, zu and others. The docs do not separate fully supported languages from experimental ones. v4 "Maintains consistent voice quality across language switches". v4 Turbo has a median latency of about 100 ms, excluding network. — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
- **ElevenLabs `eleven_flash_v2_5`** [V]: 32 languages, which are the Multilingual v2 set plus hu, no and vi. Text normalization is off by default "to maintain low latency", so numbers and dates are read raw unless you set `apply_text_normalization`. — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
- **ElevenLabs `eleven_multilingual_v2`** [V]: 29 languages (en, ja, zh, de, hi, fr, ko, pt, it, es, id, nl, tr, fil, pl, sv, bg, ro, ar, cs, el, fi, hr, ms, sk, da, ta, uk, ru). The docs say it keeps "the speaker's unique characteristics **and accent**" across languages. — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
- **Cartesia Sonic 3.6** [V]: 44 languages (en, fr, de, es, pt, zh, ja, hi, it, ko, nl, pl, ru, sv, tr, tl, bg, ro, ar, cs, el, fi, hr, ms, sk, da, ta, uk, hu, no, vi, bn, th, he, ka, id, te, gu, kn, ml, mr, pa, or, ur). Regional locales such as en-GB are accepted. No per-language quality tiers are given. Odia and Urdu support instant cloning. — [Cartesia Sonic 3.6 docs](https://docs.cartesia.ai/build-with-cartesia/tts-models/latest)
- A third-party guide says Cartesia's quality is concentrated in en, es, fr, de, pt, it, hi and ja, and that long-tail languages are "functional but less depth-tuned". — [via FutureAGI, 2026](https://futureagi.com/blog/elevenlabs-vs-cartesia-tts-2026/) [I, low rigor]
- **Inworld TTS-2 and TTS-2 Flash** [V]: both have the same coverage, grouped into three tiers.
  - **Tier 1**: 15 languages, "most extensively evaluated, highest and most consistent quality". The examples given include ar, zh, en, fr, de, hi, ja and es. The full 15 were not enumerated in the fetched text.
  - **Tier 2**: 79 languages, "fully supported" but more variable.
  - **Tier 3**: 376 languages, not evaluated. Pronunciation may not be native and there is no text normalization.
  - Tiers 1 and 2 together cover 94 languages. Model differences: `inworld-tts-2` has the most accurate pronunciation, and Flash has the same coverage at lower latency and cost.
  — [Inworld Multilingual docs](https://docs.inworld.ai/tts/capabilities/multilingual)
- Inworld's language counts are inconsistent across its own pages: "200+ languages and locales" in the docs and "100+" in marketing. — [Inworld TTS models docs](https://docs.inworld.ai/tts/tts-models); [Dasha leaderboard, 23 Sep 2026](https://dasha.ai/blog/tts-leaderboard)
- **MiniMax Speech**: 2.8 (announced 23 Jan 2026) is still the latest flagship. I found no "Speech 3.x". Claimed coverage is 40+ languages (from 2.5/2.6 onward). Cloning needs about 10 s. One third-party guide says the public docs describe no consent-verification step. — [MiniMax Speech 2.8 news](https://www.minimax.io/news/minimax-speech-28); [MiniMax Speech 2.6](https://www.minimax.io/news/minimax-speech-26); [Invideo, 2026](https://invideo.io/blog/minimax-ai-voice-models/)
- **Fish Audio Drama 3** (Oct 2026): pre-training covers "about 80 languages and dialects", the same as S2. No tiers are defined. — [Fish Audio Drama 3 Technical Report, arXiv 2610.18010](https://arxiv.org/html/2610.18010v2) [V-paper]
- **Hume Octave 2**: 11 languages (ar, en, fr, de, hi, it, ja, ko, pt, ru, es) [V] — [Hume Octave 2 launch](https://www.hume.ai/blog/octave-2-launch).
  - **Status**: a third party quotes Hume's changelog of 2 Oct 2026: "Hume is sunsetting the TTS and EVI APIs. Access ends November 13, 2026". It also reports that Hume's TTS docs now redirect to Audio Decisions. — [Versely, 2026](https://www.versely.studio/blog/hume-octave-2-emotional-tts-2026) [I, unverified against the primary source]. The Hume developer FAQ I fetched now covers only Audio Decisions, which is consistent with the sunset. — [Hume dev FAQ](https://dev.hume.ai/voice/docs/text-to-speech-tts/faq)
- **Speechify Simba 3.5** (21 Oct 2026) [V]: 48 languages under `simba-3.5`. Cloning uses a 10–30 s clip plus spoken consent, on paid plans. The cloning docs list only `simba-3.0`, `simba-english` and `simba-multilingual`, so whether cloned voices get all 48 languages on 3.5 is unconfirmed. Cloning endpoints return `403 voice_cloning_unavailable_in_region` in some jurisdictions. List price is $15/1M characters; the $10 launch price ended 21 Nov 2026. — [Speechify Simba 3.5 blog](https://speechify.com/blog/speechify-simba-3-5/); [Speechify models](https://speechify.ai/models); [Speechify voice cloning](https://speechify.ai/voice-cloning); [Speechify changelog](https://docs.speechify.ai/build/changelog)
  - Note the price conflict: AA lists Simba 3.5 at $6.58/1M (see the prior note).
- **Alibaba Qwen-Audio-3.1-TTS** (`qwen-audio-3.1-tts-flash` / `-plus`) on Model Studio [V]: streaming, cloning, voice design and instructions. A third party reports 16 languages plus 20 Chinese dialect regions. The predecessor, Qwen-Audio-3.0-TTS, supports 16 languages: zh, en, ja, ko, de, es, fr, it, ru, ar, id, pt, th, vi, ms, tl. — [Alibaba Cloud Qwen-Audio-3.1-TTS-Flash](https://www.alibabacloud.com/help/en/model-studio/qwen-audio-3-1-tts-flash); [Alibaba news](https://www.alibabacloud.com/en/news/product/qwen-audio-31-tts-flash-real-time-speech-synthesis-large-model-published-o87); [Qwen-Audio-3.0-TTS report, arXiv 2607.23938](https://arxiv.org/html/2607.23938)
- **Open weights**:
  - Qwen3-TTS: 10 languages (zh, en, de, it, pt, es, ja, ko, fr, ru). — [arXiv 2601.15621](https://arxiv.org/html/2601.15621v1)
  - CosyVoice 3: evaluated on 9 languages (zh, en, ja, ko, de, es, fr, it, ru). — [arXiv 2607.23938, Table 4](https://arxiv.org/html/2607.23938)
  - Chatterbox Multilingual: 23+ languages, MIT licence. — [Fora Soft, 18 Nov 2026](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology)
  - OmniVoice: 600+ languages zero-shot, Apache-2.0. — [arXiv 2604.00688](https://arxiv.org/pdf/2604.00688)
- **Google Chirp 3 HD Instant Custom Voice**: 30+ locales. It is allow-listed and needs a consent script. Cross-language transfer is documented only from en-US to de-DE, es-US, es-ES, fr-CA, fr-FR and pt-BR. — [Google Chirp 3 ICV docs](https://docs.cloud.google.com/text-to-speech/docs/chirp3-instant-custom-voice) (carried from the prior note)
- **Gradium** (5 languages: en, fr, de, es, pt) [V]: claims cross-lingual cloning across en, fr, es and de. On AA's per-language controlled-voice boards (10 Sep 2026) it reports ranks of 6/23 in French (1,209), 7/23 in Portuguese (1,240) and 8/24 in German (1,152). — [Gradium, updated 19 Nov 2026](https://gradium.ai/content/best-multilingual-tts-apis-2026)

### Inferences
- For a product that must work with any language pair, the realistic candidates are ElevenLabs v4 Turbo (90+), Inworld TTS-2 (94 languages in Tiers 1–2), Cartesia Sonic 3.6 (44), Speechify Simba 3.5 (48, but cloning coverage is unclear), MiniMax 2.8 (40+) and Fish Drama 3 (about 80).
- Qwen-Audio-3.1 (16) and the Qwen3-TTS and CosyVoice open models (9–10) are narrow, but they lead on CJK.
- Gradium's mention of per-language AA boards suggests AA ran per-language controlled-voice boards in Sep 2026. I could not verify them directly, so treat that as a lead for follow-up.

### Gaps
- Inworld's full list of 15 Tier-1 languages was not enumerated in the fetched page.
- **Azure** (DragonHD / Personal Voice), **OpenAI** custom voices, **HeyGen Voice API**, **Resemble** and **IndexTTS2**: I did not confirm their language lists or cross-lingual cloning coverage in this pass, and no source was retrieved. IndexTTS2 is generally understood to target zh and en only; this is unverified.
- MiniMax 2.8's exact language list was not fetched.

---

## 2. Per-language benchmarks (WER/CER and SIM)

### Takeaway
The only usable per-language data comes from vendor papers. They agree on three points:
1. On CJK, Chinese vendors (Alibaba Qwen-Audio, MiniMax, Qwen3-TTS) clearly beat ElevenLabs on intelligibility and speaker similarity. ElevenLabs Multilingual v2 had very high zh/ja/vi/th error rates in 2025. ElevenLabs v3 lagged badly on SIM in 2026.
2. ElevenLabs v4 has largely closed the intelligibility gap. In Fish's Nov 2026 run of the MiniMax 24-language test set, v4 has the lowest WER/CER of all systems in 9 of 24 languages (European, Hindi, Greek, Vietnamese). Inworld TTS-2 is competitive but slightly worse in most languages.
3. No published per-language **SIM** exists for v4, Cartesia 3.6 or Inworld TTS-2.

### Cited Findings: MiniMax 24-language test set, error rate % (lower is better), run by Fish Audio, Oct/Nov 2026 [V-paper, rival]
Source: [Fish Audio Drama 3 Technical Report, arXiv 2610.18010v2, Table 6](https://arxiv.org/html/2610.18010v2). The authors ran ElevenLabs v4, Inworld TTS-2 and Soniox themselves. The OmniVoice, FireRedTTS3 and Fish S2 figures are quoted from their own papers. WER only; **no SIM**.

| Lang | Fish Drama 3 | ElevenLabs v4 | Inworld TTS-2 | OmniVoice | Best |
|---|---|---|---|---|---|
| Arabic (CER) | 0.76 | 0.71 | 1.39 | 1.39 | v4 ≈ Drama3 |
| Cantonese (CER) | 22.70 | 19.74 | 24.14 | 17.71 | all poor |
| Chinese (CER) | 0.63 | 0.66 | 0.97 | 1.01 | Drama3 ≈ v4 |
| Czech | 3.53 | 2.13 | 3.95 | 2.86 | v4 |
| Dutch | 0.59 | 1.00 | 1.53 | 1.36 | Drama3 |
| English | 1.13 | 1.56 | 1.56 | 1.56 | Drama3 |
| Finnish | 3.09 | 2.43 | 4.74 | 3.75 | v4 |
| French | 2.68 | 2.63 | 2.63 | 3.35 | v4 = TTS-2 |
| German | 0.48 | 0.90 | 0.83 | 0.96 | Drama3 |
| Greek (CER) | 3.39 | 0.56 | 0.98 | 1.06 | v4 |
| Hindi (CER) | 6.35 | 5.78 | 7.16 | 4.33 | v4 (OmniVoice lower, from its own paper) |
| Indonesian | 4.93 | 1.59 | 1.67 | 1.97 | FireRedTTS3 1.42 |
| Italian | 1.52 | 1.29 | 1.76 | 2.07 | Fish S2 1.27 |
| Japanese (CER) | 5.02 | 3.66 | 4.23 | 4.03 | Fish S2 2.76 |
| Korean (CER) | 2.36 | 2.63 | 2.65 | 2.65 | Fish S2 1.18 |
| Polish | 0.82 | 0.77 | 1.39 | 0.87 | v4 |
| Portuguese | 1.11 | 1.11 | 1.79 | 2.51 | Soniox 1.40? (paper bolds Soniox; v4 = Drama3 = 1.11 is lower; possible table error) |
| Romanian | 4.94 | 1.60 | 2.86 | 2.42 | v4 |
| Russian | 2.23 | 2.32 | 2.65 | 2.23 | ~tie |
| Spanish | 0.96 | 1.09 | 0.90 | 1.03 | TTS-2 |
| Thai (CER) | 2.27 | 3.08 | 4.19 | 3.98 | FireRedTTS3 1.87 |
| Turkish | 0.52 | 0.87 | 1.82 | 2.17 | Drama3 |
| Ukrainian | 2.10 | 1.37 | 2.18 | 1.77 | FireRedTTS3 0.55 |
| Vietnamese (CER) | 0.80 | 0.61 | 1.25 | 1.37 | v4 |

- Inworld TTS-2's error rate is higher than ElevenLabs v4's in 19 of 24 languages. The largest gaps are in Finnish, Czech, Turkish, Polish, Romanian, Arabic and Thai. — computed from the table above.
- Drama 3 claims a lower error rate than OmniVoice in 15 of 24 languages. — [arXiv 2610.18010](https://arxiv.org/html/2610.18010v2)

### Cited Findings: Qwen-Audio-3.0-TTS report, CV3-Eval multilingual, 2026 [V-paper, rival]
Source: [arXiv 2607.23938, Tables 4–5](https://arxiv.org/html/2607.23938). Error rate %: CER for zh, ja and ko, WER otherwise.

| Lang | Qwen-Audio-3.0 | MiniMax-2.8-HD | ElevenLabs v3 | Qwen3-TTS-1.7B | CosyVoice3-1.5B |
|---|---|---|---|---|---|
| zh | 3.35 | 3.42 | 4.46 | 3.09 | 3.91 |
| en | 4.25 | 3.45 | 3.61 | 3.67 | 4.99 |
| ja | 4.78 | 6.29 | 5.71 | 6.48 | 7.57 |
| ko | 4.30 | 7.49 | 5.46 | 5.64 | 5.69 |
| de | 4.00 | 3.30 | 3.64 | 3.31 | 6.43 |
| es | 3.08 | 2.79 | 3.92 | 3.22 | 4.47 |
| fr | 9.77 | 8.74 | 9.31 | 9.05 | 11.8 |
| it | 3.82 | 3.67 | 4.68 | 4.01 | 10.5 |
| ru | 4.68 | 5.39 | 5.38 | 7.40 | 6.64 |
| ar | 3.36 | 3.38 | 5.51 | – | – |
| id | 2.35 | 1.46 | 2.85 | – | – |
| pt | 1.99 | 1.86 | 2.33 | 2.71 | – |
| th | 1.45 | 1.65 | 3.44 | – | – |
| vi | 3.17 | 1.64 | 4.08 | – | – |
| ms | 2.62 | 3.10 | 4.41 | – | – |
| tl | 6.43 | 6.35 | 12.8 | – | – |

- **Speaker similarity on hard subsets**. These are the only 2026 SIM numbers that include ElevenLabs.
  - hard-zh SIM: Qwen-Audio-3.0 78.7, CosyVoice3 78.5, MiniMax-2.8-HD 74.6, Qwen3-TTS 69.1, **ElevenLabs v3 50.3**.
  - hard-en SIM: Qwen-Audio-3.0 76.6, CosyVoice3 76.1, MiniMax 72.1, Qwen3-TTS 66.1, **ElevenLabs v3 48.9**.
  - DNSMOS is similar across systems (3.74–4.04).
  — [arXiv 2607.23938, Table 5](https://arxiv.org/html/2607.23938)

### Cited Findings: Qwen3-TTS report (Jan 2026) and MiniMax-Speech paper (May 2025) [V-paper, rival]
- On the MiniMax multilingual test set, Qwen3-TTS-12Hz-1.7B has higher SIM than MiniMax and ElevenLabs in all 10 languages.
  - Qwen3-TTS SIM: zh 0.799, en 0.775, de 0.775, it 0.817, pt 0.817, es 0.814, ja 0.788, ko 0.799, fr 0.714, ru 0.792.
  - MiniMax SIM: 0.780, 0.756, 0.733, 0.699, 0.805, 0.762, 0.776, 0.776, 0.628, 0.761 (same language order).
  - ElevenLabs SIM: 0.677, 0.613, 0.614, 0.579, 0.711, 0.615, 0.738, 0.700, 0.535, 0.676 (same order).
  - The ElevenLabs numbers are identical to the 2025 MiniMax paper, i.e. **Multilingual v2, not v4**.
  — [Qwen3-TTS, arXiv 2601.15621 Table 6](https://arxiv.org/html/2601.15621v1)
- MiniMax-Speech vs ElevenLabs Multilingual v2, 24 languages (2025):
  - MiniMax has higher SIM in all 24 languages.
  - ElevenLabs v2 failed badly on vi (WER 73.4) and th (73.9), and was weak on zh (16.0) and ja (10.6).
  - ElevenLabs v2 had lower WER in several European languages: de 0.57 vs 1.91, pl, ro, el, cs, fi, tr, nl.
  - Hindi SIM: MiniMax 0.818 vs ElevenLabs 0.730.
  — [MiniMax-Speech, arXiv 2505.07916 Table 2](https://arxiv.org/html/2505.07916)
- AA per-language controlled-voice boards existed as of 10 Sep 2026 (fr/pt/de, 23–24 models each), according to Gradium. — [Gradium](https://gradium.ai/content/best-multilingual-tts-apis-2026) [V citing I]

### Inferences
- The "Chinese vendors beat ElevenLabs on SIM" result is consistent across three papers (2025–2026). But every one of them used ElevenLabs Multilingual v2 or v3, not v4. Fish's Nov 2026 run shows v4 has closed the **WER** gap, including zh CER 0.66 and vi 0.61, where v2 had 73% WER. There is still **no SIM evidence for v4**.
- On WER, v4 Turbo is assumed to track v4 closely. That is an inference: no source benchmarks Turbo per language.
- Inworld TTS-2 is consistently a little worse than v4 on intelligibility outside English and Spanish.
- Cantonese is poor for every system (CER 17–24%). Plan on Mandarin output, or warn users.

### Gaps
- No per-language SIM for ElevenLabs v4/v4 Turbo, Cartesia Sonic 3.6, Inworld TTS-2, Speechify Simba 3.5 or HeyGen.
- No independent per-language MOS or listening test for any 2026 commercial model. A Podonos or Labelbox multilingual study was not found.
- I could not open AA's per-language boards directly.

---

## 3. Cross-lingual cloning: accent leakage and similarity across language pairs

### Takeaway
Vendors make explicit design choices about accent:
- **Inworld TTS-2**: native accent by default. A cross-lingual clone gives up some similarity, and "localization" (remove the accent) currently works only for English-source voices.
- **ElevenLabs Multilingual v2**: keeps the speaker's accent, according to its docs.
- **ElevenLabs v4**: press quotes the claim that "instant clones speak other languages with a native accent".
- **Cartesia**: third parties say instant clones keep their accent across languages.

The only quantitative cross-lingual data is for CJK and English pairs from Alibaba. There, Qwen-Audio-3.0 and Qwen3-TTS beat MiniMax-2.8-HD and ElevenLabs v3 by large margins into ja and ko.

### Cited Findings
- **Inworld TTS-2** [V]:
  - "A cloned voice gives its best speaker similarity in the language it was cloned in."
  - In another language the model "aims for native-sounding speech and doesn't carry over the original accent by default". The accent can be kept by prompting (steering).
  - Setting `language` on a non-localized voice "may keep its original accent".
  - Localization (two steps via API: candidates, then approve; up to 20 languages per voice ID; drafts expire after 3 days) is "currently only supported for voices whose original audio was English".
  - TTS 1.5 was not cross-lingual and carried the prompt's accent.
  — [Inworld Multilingual docs](https://docs.inworld.ai/tts/capabilities/multilingual); [Inworld search summary](https://inworld.ai/resources/cross-lingual-voice-cloning)
- **ElevenLabs** [V]:
  - Multilingual v2 keeps "the speaker's unique characteristics and accent" across languages. — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
  - v4: "an instant clone can speak other languages with a native accent" (vendor claim relayed by press). — [Progressive Robot, 28 Sep 2026](https://www.progressiverobot.com/2026/09/28/eleven-v4-turbo-elevenlabs-tops-tts-arena/)
  - Third party: cross-lingual fidelity "varies by language pair and tends to be higher for better-represented languages". — [Gradium](https://gradium.ai/content/best-multilingual-tts-apis-2026)
- **Cartesia Sonic 3.6** [I, secondary]: "instant clones keep accent across languages". — [Fora Soft](https://www.forasoft.com/blog/article/real-time-voice-cloning-technology); [FutureAGI](https://futureagi.com/blog/elevenlabs-vs-cartesia-tts-2026/). The Cartesia changelog says instant clones "can now speak several languages natively" (see the prior note). These two claims are in tension: accent carryover vs native speech. Test it.
- **Anecdotal** [I, weak]:
  - A blind A/B write-up (19 Apr 2026, older models) says ElevenLabs is "the only viable option of the two" vs Cartesia for cloned voices in ja, ko and zh. The page contains no per-language test data. — [CTAIO](https://ctaio.dev/en/labs/my-ai-clone/compare/elevenlabs-vs-cartesia/)
  - ElevenLabs is good on Modern Standard Arabic but inconsistent on regional dialects. — [reviewnexa](https://reviewnexa.com/elevenlabs-review/) via search
- **Cross-lingual error rate %, CV3-Eval, source → target** [V-paper, rival]. Source: [arXiv 2607.23938, Table 6](https://arxiv.org/html/2607.23938).

| Pair | Qwen-Audio-3.0 | MiniMax-2.8-HD | ElevenLabs v3 | Qwen3-TTS-1.7B | CosyVoice3-1.5B |
|---|---|---|---|---|---|
| en→zh | 5.23 | 9.96 | 7.15 | 4.77 | 8.01 |
| ja→zh | 3.29 | 6.07 | 6.17 | 3.43 | 6.78 |
| ko→zh | 1.09 | 3.63 | 2.82 | 1.08 | 3.30 |
| zh→en | 2.40 | 3.59 | 4.86 | 2.77 | 4.32 |
| ja→en | 3.15 | 5.79 | 5.30 | 3.04 | 5.39 |
| ko→en | 3.54 | 4.28 | 4.88 | 3.09 | 5.94 |
| zh→ja | 6.53 | **30.7** | 10.7 | 8.40 | 13.7 |
| en→ja | 6.66 | 13.7 | 12.4 | 7.21 | 13.4 |
| ko→ja | 2.98 | 6.39 | 6.00 | 3.67 | 4.19 |
| zh→ko | 4.27 | 6.45 | 6.26 | 4.82 | **31.6** |
| en→ko | 4.34 | 6.78 | 5.48 | 5.14 | 14.0 |
| ja→ko | 5.15 | 11.3 | 8.01 | 5.59 | 10.5 |

- Averages across the 12 pairs: Qwen-Audio-3.0 4.05 vs CosyVoice3 10.09. — [arXiv 2607.23938](https://arxiv.org/html/2607.23938)
- The Qwen3-TTS report gives slightly different CosyVoice3 numbers for the same pairs (for example zh→ko 14.4, ja→zh 3.05). The likely cause is a different CosyVoice3 checkpoint (0.5B vs 1.5B). — [arXiv 2601.15621 Table 7](https://arxiv.org/html/2601.15621v1)
- **MiniMax-Speech, Chinese speakers into other languages** (2025): zero-shot SIM drops to 0.55–0.73. Examples: Finnish 0.554, French 0.586, Arabic 0.619, Thai 0.729, Vietnamese 0.692. Same-language SIM is about 0.74–0.83. One-shot prompting raises SIM (fi 0.655) but worsens WER. — [MiniMax-Speech, arXiv 2505.07916 Table 3](https://arxiv.org/html/2505.07916)
- Fish Drama 3 reports **no** cross-lingual cloning results. — [arXiv 2610.18010](https://arxiv.org/html/2610.18010v2)

### Inferences
- Cross-lingual SIM typically drops by about 0.05–0.20 cosine relative to same-language SIM (MiniMax data). Expect "sounds like a relative of the speaker" rather than "the same person" for distant pairs such as zh→fi or zh→fr.
- There is a trade-off between native pronunciation and identity. Inworld documents it openly: native accent by default, lower similarity. For this product goal (native pronunciation), Inworld's default and ElevenLabs v4's claimed native accent fit. Cartesia's reported accent-keeping behaviour may sound more "same person, foreign accent". That is a product choice to test with users.
- Into ja and ko, MiniMax-2.8 and ElevenLabs v3 had double-digit error rates in some directions. Qwen-family models are the safest for East-Asian targets on published data.

### Gaps
- No independent, quantitative accent-leakage measurement for any provider. A typical method would be an L1-accent classifier, or native-rater "accent" MOS on the cross-lingual output.
- No cross-lingual data at all for ElevenLabs v4/v4 Turbo, Cartesia 3.6, Inworld TTS-2 or Speechify, especially for pairs involving ar, hi, ru, de, fr, es and pt.

---

## 4. Streaming latency in non-English languages

### Takeaway
No provider or independent benchmark publishes per-language TTFA. Coval measures aggregate TTFA, and Gradium notes that only Gradium itself documents latency per language. Single-model architectures (v4 Turbo, Sonic 3.6, TTS-2) should be broadly language-independent, but that is unverified.

### Cited Findings
- Inworld's multilingual page "gives no latency figures by language". It gives only aggregate P90 TTFB: TTS-2 100 ms, Flash 20 ms, server-side. — [Inworld Multilingual docs](https://docs.inworld.ai/tts/capabilities/multilingual); [Inworld TTS models](https://docs.inworld.ai/tts/tts-models)
- "No provider except Gradium shows latency by language". Coval medians, 8 Sep 2026: Flash v2.5 185 ms, Sonic 3.6 440 ms, Sonic 3.5 269 ms. These conflict with Coval's 30-day figure of about 282–288 ms for Sonic 3.6. — [Gradium](https://gradium.ai/content/best-multilingual-tts-apis-2026)
- ElevenLabs Flash v2.5 disables text normalization by default for latency. Enabling it, which matters for numbers and dates in non-English text, may add latency. — [ElevenLabs Models docs](https://elevenlabs.io/docs/overview/models)
- Inworld Tier 3 languages get no text normalization at all. — [Inworld Multilingual docs](https://docs.inworld.ai/tts/capabilities/multilingual)
- Qwen-Audio-3.1-TTS-Flash is described as low-latency streaming. The independent Coval TTFA for "Qwen3 TTS Flash Realtime" (722 ms) is much slower than for self-hostable Qwen3 TTS (77–105 ms), probably because of the region or gateway (see the prior note). — [Alibaba Cloud](https://www.alibabacloud.com/help/en/model-studio/qwen-audio-3-1-tts-flash)

### Inferences
- The "CJK is slower" concern is not documented for these models. A plausible mechanism is that text normalization and segmentation for unspaced scripts (zh, ja, th) delay the first chunk when the client buffers on word or punctuation boundaries. Mitigate this in the client by flushing on CJK punctuation (。、！？) rather than on spaces.
- Alibaba endpoints outside Singapore or Beijing will likely add 300–600 ms of network RTT for EU or US users.

### Gaps
- No per-language TTFA measurements found for any provider. Measure in-house from the target regions.

---

## 5. Price per output minute for non-English (character-based pricing)

### Takeaway
All the leading APIs price per character, or per token for Gemini and Qwen, and none charge different rates by language. Speech rate per character varies widely by script, so CJK output is far cheaper per minute than Latin-script output.

### Cited Findings
- Inworld prices are the same across languages. ElevenLabs: Flash v2.5 costs 0.5 credit per character and Multilingual v2 costs 1 credit per character. Gradium credits are "consumed the same way in each" language. — [Gradium](https://gradium.ai/content/best-multilingual-tts-apis-2026)
- List prices per 1M characters are in the prior note: ElevenLabs v4 Turbo $40, Inworld TTS-2 $25, Cartesia $38–65, Speechify Simba 3.5 $15 (Speechify list price; AA shows $6.58), Qwen-Audio-3.1 about $19 (AA). — [prior note](/home/debian/diceri.com/docs/research/2026-11-voice-translation-stack/notes/voice-cloning-tts.md); [Speechify models](https://speechify.ai/models)

### Inferences
These figures are not sourced. They are typical speech-rate estimates and should be validated with your own MT output.
- English and Romance or Germanic languages run about 800–900 characters per minute of speech; the prior note uses 850.
- Mandarin and Japanese (kanji/kana) run about 250–350 characters per minute, and Korean (Hangul) about 400–450. At the same $/1M characters, CJK output costs roughly **1/2 to 1/3 per minute** of English output.
  - Example: v4 Turbo at about $0.034/min for English is about $0.010–0.014/min for zh.
- German and Finnish need more characters for the same content. Expect about 10–20% more cost than English per translated utterance, though the cost per audio minute is similar.
- Arabic and Hebrew (no written vowels) are slightly cheaper per minute. Hindi in Devanagari is about the same as English.
- Token-priced providers (Gemini, Qwen after 22 Sep 2026) may show the reverse effect, depending on how the tokenizer handles CJK.

### Gaps
- I found no vendor documentation confirming how ElevenLabs, Cartesia or Inworld count CJK characters, for example whether a CJK character counts as 1 character or more than 1. Verify on invoices.

---

## Recommendation: per-language-group routing (inference from the findings above, as of 25 Nov 2026)

| Target language group | Best cloning TTS (streaming) | Fallback | Evidence strength |
|---|---|---|---|
| English | ElevenLabs v4 Turbo | Inworld TTS-2 (cheaper; tied on AA English cloning) | Medium: AA cloning Elo is English-only; Fish WER table |
| Major W. European (es, fr, de, it, pt, nl) | ElevenLabs v4 Turbo | Inworld TTS-2 (Tier 1 for es/fr/de); Cartesia Sonic 3.6 | Medium: Fish 2026 WER run; no SIM |
| Central/E. European, Greek, Turkish (pl, cs, ro, ru, uk, el, fi, tr, hu) | ElevenLabs v4 Turbo | Fish Drama 3 hosted (strong tr, nl, de; weak ro, el) | Medium: v4 has the lowest WER in pl, cs, ro, el, fi |
| Mandarin Chinese | Qwen-Audio-3.1-TTS-Flash/Plus (Alibaba; Singapore region) | ElevenLabs v4 Turbo (zh CER 0.66 in the Fish run) | Medium-high on WER/SIM vs v3; no data vs v4 |
| Japanese, Korean | Qwen-Audio-3.1-TTS | ElevenLabs v4 Turbo | Medium: strong cross-lingual into ja/ko vs MiniMax 2.8 and ElevenLabs v3; v4 untested cross-lingually |
| Cantonese | ElevenLabs v4 (yue listed; CER 19.7) | none good (all systems 17–24% CER) | Low: consider falling back to Mandarin |
| Arabic (MSA) | ElevenLabs v4 Turbo (CER 0.71) | Qwen-Audio-3.1 (ar 3.36 WER, CV3-Eval) or Inworld TTS-2 (Tier 1) | Medium; dialects untested |
| Hindi and Indic (hi, bn, ta, te, mr, gu, kn, ml, pa, ur) | ElevenLabs v4 Turbo (hi CER 5.78, best tested commercial) | Cartesia Sonic 3.6 (widest Indic list among low-latency APIs) | Low for languages other than hi |
| SE Asian (vi, th, id, ms, tl) | ElevenLabs v4 Turbo (vi 0.61) / Qwen-Audio-3.1 for th, ms, tl | Inworld TTS-2 | Medium for vi/th/id; low for ms/tl |
| Long tail (sw, am, ha, other African; Central Asian) | ElevenLabs v4 Turbo (listed in 90+) | Inworld TTS-2 Tier 2 | Low: no benchmarks |

Excluded: Hume Octave 2, whose API was reportedly sunset on 13 Nov 2026. Also excluded are Gemini, Chirp 3, OpenAI and Azure (gated or geo-restricted cloning, per the prior note) and MiniMax 2.8 (weak in some cross-lingual directions, e.g. zh→ja 30.7% CER, and expensive). Speechify Simba 3.5 is a watch-list item until cloned-voice language coverage on 3.5 is confirmed.

### Overall verdict: single provider vs routed
- **Default to a single provider, ElevenLabs v4 Turbo, for all languages.** It is the only low-latency cloning API that covers 90+ languages and has the lowest or near-lowest independent-run WER in most of 24 tested languages (Fish run, Nov 2026). A single provider also keeps one clone per speaker, one consent flow and one streaming integration.
  - Its known weakness is speaker similarity. Every 2025–2026 paper shows earlier ElevenLabs models (v2/v3) well behind Alibaba and MiniMax on SIM, for example hard-zh SIM 0.50 vs 0.79. v4's SIM is unmeasured, so this is the main open risk.
- **Add one targeted route: CJK targets (zh, ja, ko) to Qwen-Audio-3.1-TTS.** This is justified because published cross-lingual data into ja and ko strongly favours the Qwen family, and Chinese, Japanese and Korean are high-volume target languages.
  - Costs of this route: a second clone per speaker (doubling consent and capture handling), Alibaba data residency and region latency, and unverified realtime cloning consent terms.
  - Turn it on only if an in-house bake-off confirms that v4 Turbo trails on SIM or naturalness for those targets.
- **Keep Inworld TTS-2 as the cost and failover provider.** Its tiering is honest, it uses native-accent defaults, and it costs about 40% less. It is slightly worse than v4 on WER in 19/24 languages.
- **Required validation before committing.** No benchmark covers v4, Sonic 3.6 or TTS-2 cross-lingually, so run an in-house matrix:
  - 10 s references in {zh, ja, ko, ar, hi, ru, de, fr, es, pt, en}, synthesized into each other.
  - Score with ECAPA or WavLM SIM, Whisper-large WER/CER, an L1-accent classifier for accent leakage, native-rater MOS, and TTFA measured from the target regions.
