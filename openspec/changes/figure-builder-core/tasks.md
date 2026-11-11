# Tasks

## Error and observability primitives

- [x] Write failing tests for `SVError` Display and for the span fields.
- [x] Implement `SVError` and the operation-span helper.

## Backend trait and kind

- [x] Write failing tests for `SVBackendKind` (no automatic variant) and for per-operation dispatch.
- [x] Implement `SVBackendKind`, `SVRenderBackend` and a test backend that records the calls it receives.
- [ ] Implement the capability matrix lookup and the build-time rejection of unsupported combinations.

## Builder typestate

- [ ] Write failing tests proving a figure cannot be built without a data source or a backend.
- [ ] Implement the builder state machine and `SVFigure`.
- [ ] Implement `build`, transferring the kind without dispatch.

## Acceptance

- [ ] Verify concurrency: the same built figure rendered from several threads is safe.
- [ ] Verify the span fields through a test subscriber.
- [ ] Update the crate documentation and the `CHANGELOG.md` entry.

