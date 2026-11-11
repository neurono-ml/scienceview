# Tasks

## Mark enumeration and geometry

- [x] Write failing tests for each variant's geometry.
- [x] Implement `SVMark` and the geometry of each variant.

## Option surface

- [x] Write failing tests for shapes, dashes, baselines, orientation, patterns and text formatting.
- [x] Implement the options and their validation.

## Paint and theme

- [x] Write failing tests for token defaults and explicit overrides.
- [x] Implement paint resolution through theme tokens.

## Interpolable table

- [x] Write failing tests asserting which properties interpolate.
- [x] Export the table for the transition kernel.

## Review follow-up

- [x] Record `ErrorBar`'s option surface in the core FDR.
- [x] State representation parity and unsupported-mark requirements in the
      delta, with uniquely named scenarios.
- [x] Implement the mark capability matrix and the typed
      `SVError::UnsupportedMark`.
- [x] Resolve each channel's representation once and keep a filled cross
      visible.

## Implementation notes

- Tests and implementations were committed in per-section commits (enumeration
  and geometry, option surface, paint and theme, interpolable table) rather
  than strictly one failing-test commit plus one implementation commit per
  task. Splitting a kernel's tests from its implementation would have produced
  intermediate commits that do not compile, which the mandatory CI gates reject;
  each section commit was reviewed as a unit. History was not rewritten after
  review.

