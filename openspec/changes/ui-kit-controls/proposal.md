# Proposal

## Why

A figure without controls cannot be explored, and a dashboard without
controls cannot be filtered. The UI kit specifies controls as model objects
driven by an action vocabulary; implementing it is what lets a third-party
widget drive a figure exactly like a built-in one.

## What Changes

- Implement controls as model objects: zoom in and out, reset, pan toggle, series visibility, range slider, region selection and playback controls.
- Dispatch the action vocabulary and observe the resulting events.
- Theme the controls from tokens and place them inside the figure frame or in a host arrangement.
- Expose the integration contract that third-party UI frameworks implement.

## Capabilities

### New Capabilities

- `ui-kit-controls`: ui kit controls.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/control/`.
- Depends on the figure contract, the theme and the animation clock.
- Controls are not artists; they never appear in the mark pipeline.
