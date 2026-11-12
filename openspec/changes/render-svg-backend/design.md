# Design

## Context

SVG is the documentation target, so it must be deterministic: the same figure
must produce byte-identical output, or the figures job cannot check drift.

## Decisions

### D1 — Determinism is a requirement

Attribute order, number formatting and element grouping are fixed. A non-deterministic SVG would make the CI drift check meaningless.

### D2 — Group large marks into paths

A scatter of ten thousand points becomes one path rather than ten thousand elements, which keeps documents openable.

### D3 — Animation resolves to the final state

An SVG file has no clock. A transition resolves to its end state and the backend declares `animation: unsupported` for a static document.

### D4 — No dependencies

The crate uses the standard library only. This is what allows the backend to be enabled everywhere, including documentation builds.

## Risks / Trade-offs

- Without a clock, an SVG of an animated figure shows only the last frame; the capability matrix states this explicitly.

