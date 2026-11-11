# Proposal

## Why

The `data-view-boundary` capability is normative for every artist: data enters
through an open trait and is funnelled into a closed representation that
kernels dispatch on exactly once. Nothing of it exists. Without this layer a
chart cannot accept a borrowed slice, a sparse matrix or a live stream, and the
representation-parity rule cannot be tested at all.

## What Changes

- Introduce the `SVDataSource` ingestion trait and the `SVColumnView` representation enumeration (dense, sparse, streaming).
- Implement adapters for borrowed slices, `ndarray` views (feature-gated), sparse views and standard-library channels.
- Implement the bounded rolling window for streaming sources.
- Provide the parity harness that compares dense, sparse and streaming output for the same data.

## Capabilities

### New Capabilities
- `data-source-layer`: data source layer.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/data/`, `window/`.
- Every kernel and every recipe consumes this layer; the representation dispatch is written once.
- Optional features `data-ndarray` and `streaming` become functional.
