---
status: accepted
date: 2026-11-25
---

# Record architecture decisions as MADR files

## Context and problem statement

The stack (speech providers, media plane, frameworks) changes quickly and is
easy to second-guess. We need a lightweight, reviewable history of why choices
were made, readable by humans and AI agents.

## Considered options

- MADR 4 files in `docs/decisions/`
- Wiki pages
- No formal record

## Decision outcome

Chosen option: "MADR 4 files in `docs/decisions/`", because they live next to
the code, are reviewed in pull requests and are plain Markdown.

### Consequences

- Good, because decisions are versioned with the code they affect.
- Bad, because writing them takes discipline; `AGENTS.md` §12 lists when one is required.

### Confirmation

Pull requests that change the stack, layout or limits include an ADR.
