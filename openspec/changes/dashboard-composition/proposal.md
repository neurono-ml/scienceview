# Proposal

## Why

A dashboard composes figures, shares data sources between them and
turns a selection in one figure into a filter in the others, without copying
the data. The capability is specified; the crate does not exist.

## What Changes

- Implement `scienceview_dashboard` as a separate crate composing figures in a grid.
- Share data sources between cells and derive filtered views without copying the underlying data.
- Propagate selections as coordinated events and apply them as filters.
- Expose filter state to the host application and support event-driven and timed refresh.

## Capabilities

### New Capabilities

- `dashboard-composition`: dashboard composition.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_dashboard`.
- Depends on the figure contract, the data layer, the UI kit and every recipe.
- Lives in its own crate so a dashboard need cannot force a core API change.
