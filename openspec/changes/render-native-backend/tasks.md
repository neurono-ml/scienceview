# Tasks

## Window and buffer

- [x] Write failing tests for frame comparison on a headless buffer.
- [x] Implement the renderer and the window integration.

## Marks and decorations

- [x] Write failing pixel tests for each mark variant and decoration.
- [x] Implement rasterization of marks and decorations.

## Frame loop

- [x] Write failing tests for redraw on state change and viewport change.
- [x] Implement the frame loop driven by the model clock.

## Capabilities

- [x] Write failing tests for the declared capability matrix.
- [x] Implement the capability matrix and the build-time checks.

## Implementation notes

- The frame loop implementation landed in the first window-and-buffer commit
  because the present layer calls it; its tests live in
  `crates/scienceview_native/src/backend/frame_loop_tests.rs`.
- `save` is deliberately absent: the capability matrix declares no export
  format for `SVBackendKind::Native`, so the operation returns the typed
  `UnsupportedBackendCapability` error instead of writing an unrelated file.
- The interactive window test is `#[ignore]`d and gated behind
  `SCIENCEVIEW_NATIVE_DISPLAY_TESTS`, so headless CI stays green.
- `winit` and `softbuffer` are behind the crate's `window` feature, which the
  root `render-native` feature enables; a default build pulls neither.
