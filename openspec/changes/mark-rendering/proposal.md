# Proposal

## Why

`SVMark` is the geometric primitive every recipe draws with. It is specified in
`docs/FDRs/core/core-mark.md` with its full option surface and its interpolable
property table, but it does not exist — so no backend has anything to draw and
no recipe can be written.

## What Changes

- Introduce the closed `SVMark` enumeration: dot, line, area, bar, ribbon, text, image, wedge and error bar.
- Implement the option surface of each variant, including shapes, dash patterns, baselines, orientation, patterns and text formatting.
- Implement the interpolable-property table shared with the transition layer.
- Make marks consume theme tokens for size, colour, opacity and typography.

## Capabilities

### New Capabilities
- `mark-rendering`: mark rendering.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/mark/`.
- Every backend implements rendering for the enumeration; every recipe composes it.
- The interpolable table becomes part of the transition kernel's input.
