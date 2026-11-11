# Proposal

## Why

A figure that updates by replacing everything is unreadable: the eye cannot
follow a mark from one state to the next. `SVTransition` computes enter,
update and exit sets by matching marks on a stable key, and interpolates the
properties that can be interpolated. It is the layer that makes every later
chart animation possible without touching the recipes.

## What Changes

- Introduce `SVTransition` with match keys, the enter/update/exit sets and the interpolation rules.
- Interpolate only the properties the mark kernel declares interpolable.
- Consume the theme's motion tokens, including reduced motion.
- Require in-place update from any backend that claims transition support.

## Capabilities

### New Capabilities
- `mark-transitions`: mark transitions.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/transition/`.
- Applied uniformly to every recipe; no recipe implements its own tweening.
- Depends on match keys supplied by the recipe.
