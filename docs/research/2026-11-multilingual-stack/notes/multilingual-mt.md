# Multilingual MT for Live Voice Translation: Per-Pair Quality, Fast Models, Coverage (as of 2026-11-25)

Scope: choose the MT layer for a live voice-translation call app (cascade STT → MT → voice-clone TTS), covering many any-to-any pairs. This builds on `/home/debian/diceri.com/docs/research/2026-11-voice-translation-stack/notes/s2st-and-mt.md`.

Source labels:
- **[independent]**: WMT shared-task human evaluation or academic work.
- **[vendor]**: the vendor's own claim or price page.
- **[3rd-party]**: an aggregator, press outlet or benchmark site.

All WMT scores are human scores (ESA, cESA or MQM) taken from the findings PDFs. I extracted the text locally from the published PDFs and read the ranking tables directly.

---

## Q1. Which systems win per language pair (WMT25, WMT26), and what do 2026 multilingual evals say?

### Takeaway
**WMT26 (Oct 2026), the newest independent human evaluation:**
- **Gemini 3.1 Pro** is the strongest single general-purpose model. It is in the top cluster on 15 of 23 pairs.
- The overall winner, **Wayfinder**, scores top cluster on 17 of 23. It is an agent or ensemble built on Gemini 3.1 Pro, **Gemini 3.5 Flash**, GPT-5.5 and Qwen 3.7 Max.
- **GPT-5.5** is close behind, and wins or ties some pairs (cs→de, cs→vi, en→kk).
- **Google Translate (NMT)** ranked near the bottom of the field: average rank 26.9, against 4.8 for Gemini 3.1 Pro.
- Claude was not among the organizer-collected systems in WMT26. In WMT25, Claude 4 was mid-pack on most pairs.

**WMT25:** Gemini 2.5 Pro was top or tied-top on 11 of the 15 human-evaluated pairs. The exceptions were EN→Egyptian Arabic (GPT-4.1 first) and EN→Chinese (a fine-tuned participant system first).

**Hardest content:** the speech and social domains score lowest in both years. Speech in particular stays among the hardest because of ASR noise. That matters for a voice app.

### Cited Findings

**WMT26 General MT (findings paper, Kocmi et al., Oct 2026, Budapest)** [independent] — [ACL Anthology 2026.wmt-main.48](https://aclanthology.org/2026.wmt-main.48/); [PDF](https://aclanthology.org/2026.wmt-main.48.pdf)

Set-up:
- 23 pairs: cs→uk, cs→de, cs→vi, zh→ja, plus EN→ar-EG, hy, be, zh-Hans, zh-Hant, cs, et, de, is, id, ja, kk, ko, lld (Ladin), lij (Ligurian), se (N. Sami), ru, th, uk.
- 46 systems were evaluated with a new contrastive ESA (cESA) protocol.
- Organizer-collected API systems: Gemini 3.1 Pro, GPT 5.5, Google Translate, Command A+, DeepSeek V4 Pro, Tiny Aya Global, plus open models (Gemma 4, Qwen 3.5/3.6, GPT-OSS, Mistral Medium 3.5, Tower+). **No Claude or DeepL** among the organizer-collected systems.
- All LLMs were zero-shot with document-level input. API systems ran with default settings, which means high reasoning.

Headline findings:
- "The best system overall was Wayfinder, scoring in the top cluster for 17 out of 23 language pairs. It was followed by Gemini 3.1 Pro (15), TRIVE (14), and Lumen (13)... the top three systems are all built utilizing Gemini."
- TRIVE is a Gemini-3.5-Flash agent with automatic post-editing and an LLM judge. UvA-MT ensembles GPT-5.4 and Gemini-3.5-Flash. Lumen is a fine-tuned constrained model.
- Average rank across pairs: Gemini 3.1 Pro 4.8, TRIVE 5.1, Wayfinder 5.2, Lumen 6.8, GPT 5.5 8.6, DeepSeek V4 Pro 17.5, Gemma 4 31B 22.0, GoogleTranslate 26.9, Mistral Medium 3.5 29.3, GPT-OSS-120B 32.1, Qwen 3.6 27B 35.1, Tiny Aya 35.5.
- Cost of translating the full test set at July 2026 API prices: Gemini 3.1 Pro $251.7 (20.6M output tokens, because reasoning is verbose), GPT 5.5 $133.5, Google Translate $82.9, DeepSeek V4 Pro $41.5, Tiny Aya $3.9.
- "Low-resource languages remain challenging": the top systems reach 80+ on some, but most systems don't exceed 60.
- The social domain was hardest and speech second hardest. Test suites reveal "hallucination and semantic detachment from source" and gender bias that aggregate scores hide.

WMT26 per-pair human scores (cESA, 0–100):

| Pair | Gemini 3.1 Pro | GPT 5.5 | Best system (score) | Google Translate |
|---|---|---|---|---|
| **zh→ja (non-EN)** | 75.7 (rank 3–6) | 75.3 (3–5) | Wayfinder 78.1, after Human post-edit 79.6 | **40.6 (rank 30, last)** |
| **cs→de (non-EN)** | 84.0 (1–5) | **84.8 (1–3)** | GPT 5.5 | 64.5 |
| **cs→uk (non-EN)** | 84.1 (5–8) | 85.8 (1–4) | TRIVE 86.9 | 74.7 |
| **cs→vi (non-EN)** | 88.5 (1–8) | **89.5 (1–5)** | GPT 5.5; Qingqiu-MT-9B 89.3 | 68.9 |
| en→ja | 90.3 (1–4) | 86.1 (6–11) | Lumen / Wayfinder 90.6 | 69.4 (26–29) |
| en→ko | **83.5 (1–3)** | 80.9 (3–5) | Gemini 3.1 Pro | 63.5 |
| en→zh-Hans | **79.3 (2–3)** | 73.3 (6–12) | Gemini; Lumen 78.6 | 69.2 |
| en→zh-Hant | 70.1 (5–8) | 68.6 (5–8) | TRIVE 80.5 | 51.7 |
| en→ar (Egyptian) | 83.7 (1–5) | 80.8 (12–17) | Wayfinder 83.9 | 72.4 |
| en→ru | 79.4 (4–6) | 76.0 (7–11) | TRIVE 80.7 | 66.1 |
| en→uk | **84.3 (1–1)** | 80.9 (4–8) | Gemini 3.1 Pro | 65.3 |
| en→de | 78.2 (1–4) | 77.1 (1–4) | Wayfinder 79.1 | 57.1 |
| en→cs | 75.5 (5–6) | 77.1 (4) | Wayfinder 78.2 | 60.2 |
| en→th | **86.2 (1–3)** | 82.0 (5–6) | Gemini 3.1 Pro | 70.9 |
| en→id | 83.5 (1–4) | 80.3 (5–10) | Gemini / TRIVE | 70.8 |
| en→is | 84.9 (2–4) | 79.7 (7–9) | Human 87.5 | 66.1 |
| en→et | 81.1 (2–6) | 80.3 (3–7) | Wayfinder 82.9 | 62.8 |
| en→kk | 80.0 (5–7) | **81.9 (1–5)** | Lumen 82.2 | 58.6 |
| en→hy | 80.7 (5–7) | 80.0 (5–7) | TRIVE / Lumen 83.4 | 52.3 |
| en→be | 77.6 (1–3) | 69.6 (7–8) | Gemini / TRIVE | 54.7 |
| en→se (N. Sami) | 61.3 (1–3) | 49.3 | Gemini | — |
| en→lld (Ladin) | 45.6 | 29.7 | Human 77.2 | — |
| en→lij (Ligurian) | 12.4 (16–17) | 38.3 | Human 84.7; **Google Translate 47.4 (rank 4)** | 47.4 |

- Best score per pair (top system): 91 for en→ja and cs→vi, down to 54 for en→Ladin and 65 for en→N. Sami. Speech-domain best scores were mostly 75–90 (Table 10). — [WMT26 PDF](https://aclanthology.org/2026.wmt-main.48.pdf)
- Open small models collapse on low-resource targets. For example, Gemma 4 4B scored 15.7 on en→et, 18.3 on en→be and 16.2 on en→is, and GPT-OSS-20B scored 24.1 on en→is. Tiny Aya Global scored 35.5 on average rank. — [WMT26 PDF](https://aclanthology.org/2026.wmt-main.48.pdf)

**WMT25 General MT** [independent] — [Findings of WMT25, ACL Anthology 2025.wmt-1.22](https://aclanthology.org/2025.wmt-1.22.pdf)

Set-up:
- 16 human-evaluated pairs: cs→uk, cs→de, ja→zh, plus EN→ar-EG, bho, zh, cs, et, is, it, ja, ko, mas, ru, sr-Cyrl, uk.
- Another 15 pairs were automatic-only: EN→bn, de, el, hi, id, kn, lt, mr, fa, ro, sr-Latn, sv, th, tr, vi.
- Human scores, with the rank range in brackets:
  - **ja→zh** (MQM, negative = error penalty; higher is better): Human −3.5; Gemini-2.5-Pro −4.4 (2); Claude-4 −5.9, GPT-4.1 −6.2 and Shy-hunyuan-MT −6.1 (3–7); Qwen3-235B −10.9.
  - **cs→de:** Gemini-2.5-Pro 90.7 (1); GPT-4.1 89.5, Claude-4 88.8, DeepSeek-V3 88.1; Human ref 82.8.
  - **cs→uk:** Gemini 92.9, GPT-4.1 92.1, Shy-hunyuan 91.8; Claude-4 89.1.
  - **en→ja:** Human 89.2; Gemini 85.8; Mistral-Medium 84.8; GPT-4.1 83.7; Claude-4 79.3; ONLINE-B 78.0.
  - **en→zh:** Algharb 88.4 (1); Shy-hunyuan-MT 88.2; Claude-4 86.9 (2–5); GPT-4.1 84.0; Gemini 83.8 (6–11).
  - **en→ko** (MQM): Human −1.9; Shy-hunyuan −2.5; Gemini −2.7 (1–3); GPT-4.1 −3.3; Claude-4 −3.4.
  - **en→ar-EG:** Human 78.5; **GPT-4.1 77.0**; CommandA 74.0; Gemini 60.6; Claude-4 55.7; ONLINE-B 1.7, which means it produced MSA or failed on the dialect.
  - **en→is:** Human 87.5; Gemini 77.6; GPT-4.1 68.0; Claude-4 47.8; NLLB 24.1.
  - **en→et:** Gemini 78.8; GPT-4.1 72.2; Claude-4 53.0.
  - **en→it:** Gemini 79.4 ≈ GPT-4.1 79.0; Claude-4 72.1.
  - **en→ru:** Gemini 83.4; GPT-4.1 76.2; Claude-4 75.9.
  - **en→uk:** Gemini 90.3; GPT-4.1 87.9; Claude-4 85.6.
  - **en→sr-Cyrl:** Gemini 94.2; GPT-4.1 92.5; Claude-4 90.0.
  - **en→bho:** Gemini 94.9; Claude-4 83.2; GPT-4.1 82.8; NLLB 75.6.
  - **en→mas** (Maasai): every system is under 10, including the human reference at 9.6.

Other WMT25 findings:
- The speech domain received the lowest scores ("likely due to its reliance on ASR-generated text"). EN→Egyptian Arabic and EN→Italian were the hardest pairs.
- Automatic metric winners (Shy-hunyuan-MT with MBR) did not consistently win under human evaluation.

**Intento "State of Translation Automation 2026"** [3rd-party summary of a vendor-neutral industry report; not verified against the original]
- Covers 37 vendors and 57 models/engines, EN→100 languages.
- LLMs make up 99% of top-performing solutions, against 89% in 2025 and 55% in 2024.
- The only non-LLM top performers are 2 DeepL solutions.
- [woshipm.com summary](https://www.woshipm.com/ai/6481394.html); 2025 edition: [Intento](https://inten.to/the-state-of-translation-automation-2025/); [Slator](https://slator.com/intento-releases-9th-annual-state-of-translation-automation-2025-industry-report/)

### Inferences
- **The Gemini family is the safest single choice for breadth.** It has led both WMT years across high-, mid- and low-resource pairs and both non-English directions. GPT-5.x is a close second and occasionally better (cs→de, cs→vi, en→kk, en→cs). The best 2026 systems are ensembles or agents built on Gemini 3.5 Flash. That suggests Flash-class quality, with post-editing or judging, is near the frontier, but those pipelines are not real-time.
- **Classic NMT (Google Translate) now lags frontier LLMs by 10–35 points on hard, document-level content.** On zh→ja it scored 40.6, last place. Do not use it as the primary MT for quality. Its role is as a fallback for very-low-resource languages where LLMs collapse (it was 4th on Ligurian).
- **Dialects** (Egyptian Arabic) and **Traditional Chinese** are weak spots where model choice changes the score by 15–20 points. Test them specifically.
- WMT scores reflect frontier "Pro" models with reasoning on whole documents. A Flash-class model with thinking off, translating short ASR segments, will score lower. No benchmark quantifies the gap (see Gaps).

### Gaps
- I found no human evaluation of **Flash-Lite, Haiku or nano-class** models on per-pair MT. WMT26 collected only Pro, frontier and open models.
- Claude (any 2026 version) and DeepL were not in the WMT26 organizer set. Dubformer (a Claude Opus 4.8 / Sonnet 4.6 / Gemini pipeline) reached rank 1–5 on en→ru, but that is a pipeline, not plain Claude.
- WMT covers only a few non-English pairs (cs→de, cs→uk, cs→vi, ja↔zh). I found **no human evaluation for pt↔ja, es↔zh or ar↔fr**.
- I did not retrieve Lokalise, Phrase or other LLM-translation leaderboards, or FLORES+ / WMT24++ 2026 LLM tables.

---

## Q2. Fast models: price and latency (Nov 2026)

### Takeaway
For short-segment real-time MT, the candidates are:
- **Gemini 3.5 Flash-Lite**: $0.30/$2.50 per M tokens, released 2026-07-21.
- **Claude Haiku 5.5**: $0.10/$0.50, released 2026-10-07.
- **OpenAI gpt-6-luna**: $0.10/$0.50; gpt-5.6-luna $0.20/$1.20.
- **Gemini 3.5 Flash** as a quality tier: $1.50/$9.

At any of these prices MT costs about **$0.001–0.01 per speaking-minute**, which is negligible next to TTS. Latency, not cost, is the deciding factor. The published Artificial Analysis TTFT figures (4.7–11 s for Flash-Lite) include reasoning time, so **thinking must be set to minimal or off** for live MT. No independent TTFT figure for thinking-off translation calls was found.

### Cited Findings
- **Gemini 3.5 Flash-Lite:** [3rd-party]
  - $0.30 per M input and $2.50 per M output; cache discount 90%; released July 21, 2026.
  - Artificial Analysis: output speed 296 tok/s and TTFT 4.67 s. The page says this **includes "thinking" time** and lists the model as a reasoning model. The class median TTFT is 9.02 s.
  - Other snapshots show 406–427 tok/s and 6–11 s TTFT.
  - [Artificial Analysis model page](https://artificialanalysis.ai/models/gemini-3-5-flash-lite); [AA comparison vs Claude 4.5 Haiku](https://artificialanalysis.ai/models/comparisons/gemini-3-5-flash-lite-vs-claude-4-5-haiku-reasoning)
- Google's model page puts 3.5 Flash-Lite input at $0.30, against $0.25 for 3.1 Flash-Lite. [vendor] — [Google DeepMind Flash-Lite](https://deepmind.google/models/gemini/flash-lite/)
- **Gemini 3.5 Flash** (May 2026): $1.50 per M input and $9 per M output, which is "3x the price of 3 Flash Preview and 6x the price of 3.1 Flash-Lite". [3rd-party] — [Simon Willison, 2026-05-19](https://simonwillison.net/2026/May/19/gemini-35-flash/)
- **Claude Haiku 5.5:** [vendor / press]
  - Released 2026-10-07. $0.10 per M input and $0.50 per M output for prompts up to 100K tokens; $0.50/$2.50 above that.
  - Pitched for "voice agents, live support". Box reported "about half the latency" of Haiku 4.5. No independent TTFT was found.
  - [AI Weekly](https://aiweekly.co/alerts/anthropic-releases-claude-haiku-55-cutting-prices-by-75); [Decrypt](https://decrypt.co/380351/anthropic-launches-haiku-5-5-cheapest-fastest-claude-model); [Anthropic Haiku page](https://anthropic.com/claude/haiku)
- **OpenAI**, standard tier per M input/output: [vendor] — [OpenAI API pricing](https://developers.openai.com/api/docs/pricing)

  | Model | Input | Output |
  |---|---|---|
  | gpt-6-luna | $0.10 | $0.50 |
  | gpt-5.6-luna | $0.20 | $1.20 |
  | gpt-5.4-mini | $0.75 | $4.50 |
  | gpt-5.4-nano | $0.20 | $1.25 |
  | gpt-5-mini | $0.25 | $2.00 |
  | gpt-5-nano | $0.05 | $0.40 |
  | gpt-6.1-terra | $1.00 | $5.00 |
  | gpt-5.5 | $5.00 | $30.00 |

  - A paid "Fast" tier exists, e.g. gpt-6-luna at $0.20/$1.00. The page lists no latency figures.
- **GPT-5.5 (low reasoning):** TTFT 1.91 s on Artificial Analysis; GPT-5.5 (medium) 7.31 s. Reasoning effort dominates latency. [3rd-party] — [AA comparison](https://artificialanalysis.ai/models/comparisons/gemini-3-5-flash-lite-vs-gpt-5-5-low)
- **Earlier context, from the prior notes:**
  - Translation TTFT of 0.15–0.40 s for Gemini, with no methodology given. [vendor] — [MachineTranslation.com](https://www.machinetranslation.com/blog/time-to-first-token-latency-of-llms)
  - A cascade with a small LLM (GPT-4o-mini) measured 2.7 s from speech end to translation, and the MT stage was not the bottleneck. [vendor-run] — [LiveLingo](https://www.livelingo.io/research/benchmark-2026)
- **DeepL:**
  - 36 languages after adding Vietnamese, Hebrew and Thai (early access) in June 2025. [vendor] — [DeepL blog](https://www.deepl.com/en/blog/vietnamese-thai-hebrew-launch)
  - The current list comes from `/v3/languages`. [vendor] — [DeepL docs](https://developers.deepl.com/docs/getting-started/supported-languages)
  - Voice API beta added Croatian and Tagalog speech output in Sept 2026. [vendor] — [DeepL release notes](https://developers.deepl.com/docs/resources/roadmap-and-release-notes.md)
  - DeepL markets "100+ languages" in 2026, but that likely counts across products and variants and is unverified.
  - DeepL claims GPT-5.2 is 4x slower, Opus 4.6 6x slower and Gemini 3.1 Pro 29x slower than DeepL. [vendor marketing] — [DeepL on X](https://x.com/DeepLcom/status/2093352915486339437)
- **Google Translate:** cost $82.9 for the WMT26 set (about 4.1M chars, so about $20 per M chars) and ranked low on quality (see Q1). [independent] — [WMT26](https://aclanthology.org/2026.wmt-main.48.pdf)

### Inferences
- **Cost per speaking-minute.** My calculation assumes about 10 segments per minute, about 500 input tokens per call (system prompt, glossary and 2–4 context segments) and about 300 output tokens per minute.

  | Model | Cost per speaking-minute |
  |---|---|
  | Haiku 5.5 / gpt-6-luna | about $0.0007 |
  | Gemini 3.5 Flash-Lite | about $0.002 |
  | Gemini 3.5 Flash | about $0.010 |

  Prompt caching of the static system prompt and glossary cuts the input cost further.
- **Latency.** With thinking off or minimal and a 20–60-token output, a fast model should return a segment in a few hundred ms. That is consistent with the prior cascade figures, but it is **unverified for these specific 2026 models**. Benchmark it yourself: TTFT and full-segment time, p50/p95, from your deployment region.
- **Quality vs breadth.** Haiku 5.5 is the cheapest and probably the fastest, but the Claude family placed second-tier at WMT25 on low-resource targets (en→is 47.8, en→et 53.0, en→ar-EG 55.7). It is not evidenced for breadth. Gemini Flash-class models belong to the family that leads both WMTs, so they are the better bet for "many pairs".

### Gaps
- No independent TTFT for Gemini 3.5 Flash-Lite or Flash with thinking at minimal or off, Haiku 5.5, or gpt-6-luna.
- No pricing or latency retrieved for Qwen-MT, Mistral, Google "Translation LLM" (Cloud Translation Advanced) or DeepL API per-character pricing in 2026.
- I did not verify GPT-6-luna's multilingual quality anywhere.

---

## Q3. Soniox built-in real-time translation vs LLM MT

### Takeaway
Soniox translation streams token by token, mid-sentence, on the same WebSocket as STT (model `stt-rt-v6`). It supports one-way and two-way modes and custom translation terms. Soniox publishes **no translation quality numbers**, and I found **no independent evaluation**. It is convenient and has the lowest latency, but its quality on hard pairs is unknown against Gemini or GPT-class MT.

### Cited Findings
- Real-time translation streams "mid-sentence, word by word", with one-way or two-way modes and custom translation terms. [vendor] — [Soniox docs](https://docs.soniox.com/stt/rt/real-time-translation)
  - Translated tokens carry **no timestamps**, and transcription and translation tokens "are not 1-to-1 mapped and may not align".
  - The language set equals Soniox's STT set: 60 languages and "3,600 pairs" per the prior notes.
- Soniox's comparison tool has "no hidden benchmark dataset and no aggregate quality score" and recommends testing on your own audio. It estimates about $0.18/h for real-time STT plus translation. [vendor] — [Soniox compare translate](https://soniox.com/compare/translate)
- Soniox claims an STT WER of 1.25% vs Google's 2.84% across 60 languages. This is STT only, with no published methodology. [vendor] — [Soniox vs Google](https://soniox.com/compare/soniox-vs-google)

### Inferences
- Word-by-word streaming translation is ideal for **captions** but risky for **TTS**. Mid-sentence output in a different word order (for example ja/ko/de verb-final) gets revised or produced awkwardly. A TTS stage still needs segment commits.
- Treat Soniox translation as a **low-latency caption path** and a fallback. Use an LLM MT on committed STT segments for the spoken path, where context and glossary control matter. A/B test them on your own call recordings.

### Gaps
- No independent quality evaluation of Soniox (or Speechmatics, Gladia or Deepgram) built-in translation. No information on which underlying MT model Soniox uses.

---

## Q4. Pivot through English vs direct translation for non-English pairs

### Takeaway
For frontier LLMs, **direct translation is the default**. WMT26 non-English pairs (cs→de, cs→uk, cs→vi, zh→ja) were all top-scored by direct LLM systems in the 75–90 range. Research shows English pivoting helps mainly weaker, smaller or low-resource set-ups, and can hurt otherwise. One WMT26 system (Grammatist) used pivoting as part of an agent.

### Cited Findings
- **Direct translation scores, WMT26:** GPT 5.5 scored 84.8 on cs→de and 89.5 on cs→vi; Gemini 3.1 Pro scored 75.7 on zh→ja. Google Translate's zh→ja score of 40.6 shows how badly non-LLM systems handle some non-English pairs. [independent] — [WMT26](https://aclanthology.org/2026.wmt-main.48.pdf)
- Grammatist, a GPT-5.4 agent using "pivot (relay translation via a pivot language)", reached the top participant cluster in every direction it entered, but only for low-resource targets (e.g. en→lij rank 2–3). [independent] — [WMT26](https://aclanthology.org/2026.wmt-main.48.pdf)
- LLM translation between two non-English languages trails their English-centric performance after fine-tuning. [academic] — [arXiv 2509.19770](https://arxiv.org/pdf/2509.19770v1)
- Pivot-augmented prompting gives moderate chrF++ gains for Konkani but inconsistent ones for Tunisian Arabic, so its value depends on language relatedness. [academic] — [arXiv 2603.16660](https://arxiv.org/html/2603.16660)
- English as a pivot is "task-dependent" and can reduce direct transfer between non-English languages. [academic] — [arXiv 2504.09378](https://arxiv.org/pdf/2504.09378v1.pdf)

### Inferences
- **Use direct X→Y with a Gemini or GPT-class model for all pairs among high- and mid-resource languages.** Two-step pivoting doubles MT latency (two serial calls) and compounds errors, such as gender, formality and honorifics lost via English. That is especially relevant for ja/ko politeness.
- Consider pivoting only when **both** sides are lower-resource (for example is↔th) and a test shows a gain. Even then, a "show English draft then translate" single prompt keeps it to one call.

### Gaps
- No 2026 benchmark directly compares pivot vs direct for frontier Flash-class models on pt↔ja, es↔zh or ar↔fr.

---

## Q5. Coverage of managed speech translators and language intersection

### Takeaway
Managed end-to-end products differ in breadth:
- **Gemini 3.5 Live Translate:** 70+ languages. Google Meet uses it for 63 languages any-to-any.
- **Azure Live Interpreter:** 76 input languages, but only about 9 output languages with personal voice in the GA list.
- **OpenAI gpt-realtime-translate:** 13 output languages.
- **DeepL Voice:** about 45–48 sources into up to 5 targets.
- **Palabra:** 60–70+ claimed.

A cascade's coverage is the **intersection** of STT (Soniox about 60), MT (LLM about 100+, effectively unconstrained), and clone TTS (ElevenLabs v4 about 90, Cartesia 44). **The limits are STT and TTS, not MT.**

### Cited Findings
- **Azure Live Interpreter:**
  - Auto-detects across 76 input languages (143 locales).
  - The GA output list is en, de, es, fr, it, ja, ko, pt and zh-Hans, with "more planned".
  - Personal voice is limited access.
  - [3rd-party summarizing a 2025 Microsoft announcement; no 2026 update found] — [Slator](https://tech.slator.com/microsoft-live-interpreter-api-ai-live-speech-translation/); [Microsoft Learn speech translation](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/speech-translation)
- **Gemini 3.5 Live Translate:** 70+ languages. **Google Meet:** any combination of 63 languages since 2026-10-22. [vendor] — [Google Workspace Updates](https://workspaceupdates.googleblog.com/2026/10/real-time-speech-translation-in-google-Meet-expands-to-63-languages-with-automatic-language-detection.html) (from prior notes)
- OpenAI gpt-realtime-translate: 13 output languages. Palabra: 60–70+ languages with cloning. DeepL Voice: up to 5 targets. — see the prior notes, [s2st-and-mt.md](/home/debian/diceri.com/docs/research/2026-11-voice-translation-stack/notes/s2st-and-mt.md)
- **Assignment brief, not independently re-verified here:** Soniox STT/translation about 60 languages, ElevenLabs v4 about 90 languages, Cartesia 44.

### Inferences
- **MT is no longer a coverage constraint.** Gemini and GPT translate into every WMT26 target, including Kazakh, Armenian and Belarusian, at 77–84. **STT accuracy on accented or low-resource speech and clone-TTS naturalness are the real constraints** on each tier.
- Meet's 63-language any-to-any list is a useful sanity benchmark. A cascade with 60-language STT matches it in breadth and adds true voice cloning.

### Gaps
- I did not retrieve exact language lists for Soniox, ElevenLabs v4, Cartesia, Meet's 63, or Palabra, so the tier table below is an **inference**. Cross-check it against the STT and TTS researchers' notes.
- No per-language quality notes for Gemini Live Translate or Meet were found.

---

## Recommendation (derived from the findings above)

**Primary MT:** **Gemini 3.5 Flash-Lite**, thinking at minimal or off, streaming output, translating **committed STT segments directly X→Y**.
- Why: it belongs to the family that leads WMT25 and WMT26 across high- and low-resource and non-English pairs. It costs about $0.002 per speaking-minute.
- Quality tier: upgrade per pair to **Gemini 3.5 Flash** (minimal thinking, about $0.01/min) where offline tests show a gain. That likely applies to low-resource targets and to dialect or script cases such as Egyptian Arabic, zh-Hant, Icelandic, Kazakh, Armenian and Belarusian.

**Fallback (cross-vendor, for outage or timeout):** **GPT-5.x/6 small model** (gpt-6-luna or gpt-5.6-luna).
- GPT is the second-strongest family at WMT26 and won cs→de, cs→vi and en→kk.
- Fail over when the primary misses a TTFT budget of about 400–600 ms.
- Haiku 5.5 is a cheaper, latency-oriented alternative for high-resource European and CJK pairs only. Claude's 2025 low-resource scores were weak, and it had no 2026 WMT data.

**Per-pair routing (offline-test-driven, simple table):**
- **Default:** Gemini Flash-Lite, direct translation.
- **Low-resource or dialect targets:** Gemini 3.5 Flash.
- **Very-low-resource targets where LLMs collapse** (e.g. Ligurian-like languages): Google Cloud Translate NMT, the only case where it beat LLMs at WMT26. In practice these will be excluded by STT and TTS anyway.
- **Captions:** Soniox built-in streaming translation, for the instant on-screen layer only.
- **Not recommended:**
  - DeepL as primary: only 36 languages, and only 2 DeepL solutions remained among Intento's top performers. Keep it as an optional EU-pair alternative.
  - Google Translate NMT as primary: it lags 10–35 points.

**Context and glossary strategy:**
- Static system prompt, cached:
  - language pair and register;
  - formality or honorific rule per target (ja/ko keigo, de/fr Sie/vous);
  - "spoken, concise, no explanations, output only the translation";
  - a per-call glossary (participant names, company and product terms, auto-extracted from the join form and the first minutes).
- Dynamic part: the last 2–4 source+target segment pairs, so pronouns, gender and terminology stay consistent.
- Pass the ASR text with a note that it may contain recognition errors. WMT notes speech as the hardest domain because of ASR noise.
- Never pivot through English by default.

**Validation:**
- Run an internal eval on your own call recordings for the target pair matrix, including pt↔ja, es↔zh and ar↔fr. Use an LLM judge (GEMBA-ESA style, as WMT26 used with GPT-5.4) plus spot human checks.
- Measure p50/p95 MT latency per model and region.

---

## Language tier table (inference)

Basis: WMT25/26 scores for frontier LLMs, plus assumed STT (Soniox about 60) and clone-TTS (ElevenLabs v4 about 90, Cartesia 44) coverage from the brief. Verify every row against the STT and TTS notes. The limiting component is named for each tier.

| Tier | Languages (examples) | Status | Limiting component |
|---|---|---|---|
| **Tier 1: all components strong** | English, Spanish, French, German, Italian, Portuguese (BR/PT), Dutch, Polish, Russian, Ukrainian, Czech, Japanese, Chinese (Mandarin, Simplified), Korean, Turkish, Indonesian, Vietnamese, Hindi, Arabic (MSA), Swedish | MT: WMT best scores of 78–91 on en→ja/ko/zh/ru/uk/de/cs/id, and 84–90 on non-EN cs→de/uk/vi; zh→ja about 76–78. STT and both clone TTS engines are expected to cover them. | None major. Watch ja/ko honorifics and zh→ja (the hardest high-resource pair, about 76). |
| **Tier 2: supported, weaker** | Thai, Hebrew, Greek, Romanian, Hungarian, Danish, Finnish, Norwegian, Bulgarian, Croatian, Serbian, Slovak, Lithuanian, Latvian, Estonian, Icelandic, Kazakh, Armenian, Belarusian, Persian, Urdu, Bengali, Tamil, Telugu, Marathi, Malay, Filipino/Tagalog, Swahili, Catalan; dialects (Egyptian Arabic, Cantonese, zh-Hant) | MT is good with frontier models (en→th 86, en→is 85, en→kk 82, en→hy 83, en→be 78, en→ar-EG 84, en→zh-Hant 70–80), but Flash-Lite-class and open models drop sharply on these. | **TTS** (many are outside Cartesia's 44, so ElevenLabs only, with weaker clone quality expected) and **STT** accuracy for lower-resource or accented speech; **MT** for dialects and zh-Hant. |
| **Tier 3: not supported** | Low-resource or regional languages: Maasai, Northern Sami, Ladin, Ligurian, Bhojpuri (MT fine at 94.9 but no STT/TTS assumed), most African languages beyond Swahili, many Indic languages beyond the top ~8 | WMT best scores of 10–65 for mas, se, lld and lij. | **STT and TTS coverage** (outside Soniox's 60 and the TTS lists); **MT** too for mas, lld, lij and se. |

Notes on the table:
- MT quality for Tier 1 and Tier 2 assumes a Gemini or GPT model. For Tier 2, route to Gemini 3.5 Flash rather than Flash-Lite. This is an inference: there is no Flash-Lite WMT data, but the steep drop of small and open models on low-resource targets at WMT26 is documented.
- For comparison, the managed options' ceilings are:
  - Gemini Live Translate: 70+ languages (Meet: 63);
  - Azure Live Interpreter: about 9 personal-voice outputs;
  - OpenAI gpt-realtime-translate: 13 outputs.

  A cascade's practical ceiling is therefore about 55–60 languages, which is the STT-bound intersection.
