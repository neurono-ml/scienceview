# Proposal

## Why

Every chart maps data values onto visual channels through a scale, and every
animated chart needs a scale whose domain and range can change without
reassigning series colours. No scale exists, so no axis can be drawn and no
mark can be placed.

## What Changes

- Introduce `SVScale` with the four kinds: continuous, nominal, temporal and boolean.
- Introduce the axis model: domain, range, transform, tick derivation and inverse axes.
- Keep nominal colour assignment stable across state changes, so animation is readable.
- Route every continuous colour encoding through a theme colour-map token.

## Capabilities

### New Capabilities
- `scale-mapping`: scale mapping.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/scale/`.
- Consumed by marks, decorations, every recipe and the transition layer.
- Axis domain derivation becomes the single source of truth for limits.
