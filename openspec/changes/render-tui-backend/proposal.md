# Proposal

## Why

Scientific work often runs over SSH, in a terminal, with no display. A
terminal backend makes the same figure available there, on a cell grid, with the
same theme tokens — and it is the hardest test of the resolution-independent
layout.

## What Changes

- Implement `scienceview_tui` on ratatui and crossterm, rasterizing the layout tree onto a cell grid.
- Map every mark to cell approximations, including partial block characters where a height is fractional.
- Drive frames from the model clock with a bounded tick rate.
- Declare the capability matrix for a text backend, including which shapes are approximated.

## Capabilities

### New Capabilities

- `render-tui-backend`: render tui backend.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_tui`.
- Depends on layout, marks, decorations and the animation clock.
- Proves that decorations placed by the solver survive a coarse raster grid.
