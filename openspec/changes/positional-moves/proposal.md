# Proposal

## Why

Overlapping marks are unreadable, and composed marks need a shared baseline.
`SVMove` is the kernel that handles both: stacking, dodging, jittering,
shifting, normalising and the collision-avoiding swarm. It is specified in
`docs/FDRs/core/core-move.md` and does not exist.

## What Changes

- Introduce the closed `SVMove` enumeration: stack, dodge, jitter, shift, swarm and normalize.
- Fix the composition order of moves, because order changes the result.
- Implement deterministic jitter and deterministic swarm layouts.
- Implement the window semantics of the moves that need a group total.

## Capabilities

### New Capabilities
- `positional-moves`: positional moves.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/move/`.
- Consumed by the categorical, area, bar and swarm recipes.
- Jitter and swarm offsets are matched by key, not by frame, so they do not drift.
