# Tasks

## Representation types

- [x] Write failing tests for the three variants and their borrowing rules.
- [x] Implement `SVColumnView` and the source trait.

## Adapters

- [x] Write failing tests for slice, `ndarray`, sparse and channel adapters.
- [x] Implement the adapters, keeping `data-ndarray`, `data-sprs`, `streaming` and `streaming-tokio` behind features.

## Window

- [x] Write failing tests for bounded rollover, drop-oldest and non-blocking reads.
- [x] Implement the rolling window and its policy.

## Parity harness

- [x] Implement the harness and adopt it in one kernel's tests as a proof of concept.
- [x] Document the tolerance policy.


## Review fixes and integration

- [x] Rename the public modules to `sv_data` and `sv_window` while keeping the on-disk folders.
- [x] Replace the figure's placeholder binding with `SVColumnView` and remove the duplicate representation enumeration.
- [x] Fold `SVDataError` into `SVError`, keeping one public error surface.
- [x] Split the parity harness into a folder module below the 200-line limit.
- [x] Record the optional features and the integration in the proposal impact.
