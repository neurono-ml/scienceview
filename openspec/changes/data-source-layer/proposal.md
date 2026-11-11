# Proposal

## Why

The `data-view-boundary` capability is normative for every artist: data enters
through an open trait and is funnelled into a closed representation that
kernels dispatch on exactly once. Nothing of it exists. Without this layer a
chart cannot accept a borrowed slice, a sparse matrix or a live stream, and the
representation-parity rule cannot be tested at all.

## What Changes

- Introduce the `SVDataSource` ingestion trait and the `SVColumnView` representation enumeration (dense, sparse, streaming).
- Implement adapters for borrowed slices, `ndarray` views, `sprs` sparse vectors, standard-library channels and Tokio channels, each behind its feature.
- Implement the bounded rolling window for streaming sources.
- Provide the parity harness that compares dense, sparse and streaming output for the same data.
- Expose the layer through the public modules `sv_data` and `sv_window` (the on-disk folders stay `data/` and `window/`), so the public surface follows the `sv_` naming rule.
- Fold the data-boundary failures into `SVError`, so the crate keeps a single public error enumeration.
- Carry the data-layer `SVColumnView` inside `SVFigure`, so the figure substrate and the data layer share one representation type instead of duplicating the enumeration.

## Capabilities

### New Capabilities
- `data-source-layer`: data source layer.

### Modified Capabilities

None.

## Impact

- New paths on disk: `crates/scienceview_core/src/data/` and `crates/scienceview_core/src/window/`, exposed publicly as `sv_data` and `sv_window`.
- Every kernel and every recipe consumes this layer; the representation dispatch is written once.
- Optional features `data-ndarray`, `data-sprs`, `streaming` and `streaming-tokio` become functional.
- `SVError` is the single public error type: the data-boundary variants are folded into it and no `SVDataError` remains public.
- `SVFigure` and the backend request values carry `SVColumnView`; the duplicate figure representation enumeration is removed.
