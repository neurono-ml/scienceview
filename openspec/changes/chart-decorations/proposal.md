# Proposal

## Why

A chart is not only its marks: axes, ticks, grid lines, the legend, the colour
bar and the titles are what make it legible, and the layout engine needs their
sizes to solve the plot area. Without decorations the layout has nothing to
measure and every backend would draw its own axes.

## What Changes

- Introduce axes and ticks with space-adaptive tick counts and formatted labels.
- Introduce the legend, including per-series entries that follow series enter and exit.
- Introduce the colour bar for continuous colour scales.
- Introduce titles, axis labels and grid styles, all themed from tokens.

## Capabilities

### New Capabilities
- `chart-decorations`: chart decorations.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/decoration/`.
- Consumed by every recipe and every backend; measured by the layout engine.
- Legend and colour bar entries are model objects, so they can animate with the data.
