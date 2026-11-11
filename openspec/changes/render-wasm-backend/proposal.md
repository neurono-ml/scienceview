# Proposal

## Why

The browser is a first-class target: a figure should run in a page at
native speed, without a server. The WebAssembly backend reuses the from-scratch
CPU renderer on a canvas, so the browser and the desktop draw the same pixels.

## What Changes

- Implement `scienceview_wasm` as the WebAssembly target of the CPU renderer.
- Drive rendering from the browser's animation frame while the model owns the clock.
- Expose the figure to JavaScript with the minimum binding surface.
- Declare the capability matrix for a canvas backend.

## Capabilities

### New Capabilities

- `render-wasm-backend`: render wasm backend.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_wasm`, under the `wasm` feature.
- Depends on the native renderer and the animation clock.
- Reuses the CPU rasterizer rather than reimplementing rendering in JavaScript.
