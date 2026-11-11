# Proposal

## Why

`theming` requires one authored theme to produce a consistent visual identity
across ten backends, resolved into backend-independent tokens. No theme exists,
so every mark, decoration and control would otherwise hardcode colours, fonts
and spacing — and the first backend would set an accidental precedent.

## What Changes

- Introduce the theme source format and compile it at build time and at runtime.
- Resolve the compiled theme into design tokens in Rust: colours, typography, spacing, radii, shadows and motion.
- Provide discrete, continuous and data-derived palettes, and the colour-map tokens that heatmaps and contours consume.
- Emit the compiled stylesheet from document backends so a user can override individual rules.

## Capabilities

### New Capabilities
- `theme-tokens`: theme tokens.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_theme`.
- Every mark, decoration and recipe consumes tokens; no literal colour or size in a renderer.
- Motion tokens become the input of the transition and animation kernels.
