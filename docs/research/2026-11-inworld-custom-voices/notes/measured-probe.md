# Measured probe with Diceri's Inworld account (2026-11-27)

Run from the Diceri production host (Debian, same network the media agent uses) with the
account's own API key, On-Demand plan. Reference audio: the synthetic voice of the
end-to-end fixture `apps/web/tests/e2e/fixtures/pt-BR.wav` (no real person's voice).
Scripts: `voices:clone` → `tts/v1/voice:stream` (HTTP, NDJSON, `inworld-tts-2`,
`audioEncoding: PCM`, 24 kHz) → `DELETE /voices/v1/voices/{id}`.

| Step | Result |
|---|---|
| `GET voices:customVoiceCapacity` before | `{"usedCustomVoices":0,"maxCustomVoices":100}` (HTTP 200; key valid) |
| Clone from 24 kHz mono 16-bit WAV, 5.84 s, `langCode: PT_BR` | HTTP 200 in **4,320 ms** |
| Clone from the same audio resampled to **16 kHz** mono 16-bit WAV (Diceri's kept format) | HTTP 200 in **3,335 ms** |
| Stream pt-BR sentence (52 chars) | HTTP 200, **first audio 238 ms**, complete 923 ms, 3.74 s of audio |
| Stream ja-JP sentence (21 chars) with the pt-BR clone | HTTP 200, **first audio 223 ms**, complete 838 ms, 3.54 s of audio |
| Delete each clone | HTTP 200 |
| Capacity after | back to `{"usedCustomVoices":0,...}`: deleting frees the slot immediately |

Not measured here: clone-creation rate limit (only 2 clones were made, well within any
published per-minute limit), voice similarity or pronunciation quality (no listening test),
WebSocket streaming, behaviour under concurrency.
