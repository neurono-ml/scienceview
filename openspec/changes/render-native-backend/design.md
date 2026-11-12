# Design

## Context

A CPU renderer keeps the backend dependency-light and portable, and it makes
the test story simple: a frame is a pixel buffer that can be compared.

## Decisions

### D1 — Rasterize the solved tree

The renderer consumes the layout tree and the mark geometry; it never re-derives positions. This is the same contract as SVG, which is what lets the two agree.

### D2 — The clock belongs to the model

The frame loop reads the playback position each frame instead of owning a timer, so a paused figure consumes no animation work.

### D3 — Frames are comparable

The renderer exposes its buffer, so acceptance tests compare pixels rather than inspecting a window.

### D4 — Capabilities are declared

Events, transitions and in-place update are declared; a static export is not a native capability.

## Risks / Trade-offs

- A CPU renderer is slower than a GPU one for very large point sets; the dependency-light constraint is accepted and the point-splat path mitigates it.


### D5 — Two layers, one contract

The rasterizer is a pure value over a pixel buffer and the present layer is the
only module that depends on a window stack. The split is what makes the frames
comparable in CI and keeps `winit`/`softbuffer` out of the default build; the
WebAssembly target reuses the rasterizer unchanged.

### D6 — The clock seam takes no animation type

The frame loop asks an `SVFrameClock` for the next playback position and calls a
frame callback only when there is one. `scienceview_native` therefore compiles
before `SVAnimation` and `SVTransition` exist and keeps no dependency on them;
a paused timeline performs no animation work.

### D7 — No export, by the capability matrix

`save` reports the missing export capability instead of inventing a format. The
frame is still exposed as bytes, so a future export change can build on the
rasterizer without changing this backend's declared row.

### D8 — A published frame updates the open window in place

The window owns a one-slot frame channel. `SVNativeWindow::publisher` returns a
cloneable producer that stores the newest frame and wakes the window with a
redraw request; the redraw presents that frame instead of repainting the
previous one, so new data never reopens the window. Only one frame is pending,
so a producer faster than the window cannot queue stale frames. The hand-off is
driven through an abstract presenter, which a recording presenter exercises
without a display; the live-window test stays ignored and environment-gated.
`SVRenderBackend::update` keeps returning the headless frame — publishing that
frame is what redraws the same window.

### D9 — Joins are geometry, not style metadata

A join fills the outer notch the segment capsules leave at a corner. A bevel
connects the two outer offset points to the corner, a miter extends them to
their intersection, and a miter whose tip exceeds four times the stroke radius
falls back to a bevel. Round joins stamp a disc, as caps already do. The path
test compares the three styles by pixel outcome and pins the fallback, so a
join is never merely accepted and ignored.
