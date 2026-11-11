# Proposal

## Why

Displays a raster: an image, or a numeric array rendered as greyscale or through a colour map. It is the recipe for showing a photograph, a microscopy frame, a spectrogram or a computed field.

The chart is specified in `docs/FDRs/matrix-image-plot.md` (FDR-134) and is
part of the scientific core defined in `docs/FDRs/patterns.md`. No
implementation exists for `SVImagePlot`.

## What Changes

- Implement `SVImagePlot` as a recipe composed of the core kernels, following the
  contract in `docs/FDRs/patterns.md`:
  `data -> [no stat] -> raster -> [SVScale::Continuous over a colour map] -> SVMark::Image`
- Expose the configuration surface recorded in the FDR: `raster` (—); `colour_map` (None — greyscale); `interpolation` (Nearest); `z_min, z_max` (None); `opacity` (1.0); `origin` (TopLeft); `colour_bar` (false).
- Ship the eight tasks of P10: module and builder, parity and concurrency
  tests, example program, generated figure, book chapter, FDR status,
  CHANGELOG entry and independent acceptance review.

## Capabilities

### New Capabilities

- `image-plot`: the SVImagePlot chart and its behaviour.

### Modified Capabilities

None.

## Impact

- New module `src/recipes/matrix/image_plot/`.
- New example `examples/image_plot.rs` and generated figure
  `docs/figures/image_plot.svg`.
- One registration line in `src/recipes/matrix/mod.rs`, owned by the
  family batch integrator.
- New specification `openspec/specs/image-plot/spec.md`.
