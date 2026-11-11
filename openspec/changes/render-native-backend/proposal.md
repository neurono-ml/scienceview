# Proposal

## Why

The native backend is the validation target of the library: the AGENTS rules
require a figure to be validated natively before alternate backends and
bindings are attempted. It is also the backend with a real frame loop, so it is
where transitions and animation are proven to work.

## What Changes

- Introduce `scienceview_native`, a from-scratch CPU renderer on winit and softbuffer.
- Rasterize the layout tree, marks and decorations into a pixel buffer.
- Drive the frame loop from the model clock, including in-place update.
- Expose the capability matrix for a backend with a clock.

## Capabilities

### New Capabilities
- `render-native-backend`: render native backend.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_native`.
- Validation target for every recipe before alternate backends.
- Depends on the figure contract, layout, marks, decorations and transitions.
