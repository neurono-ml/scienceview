# Design

## Context

The design deliberately avoids a second state pipeline: the timeline is a
sequence of deltas, and the model remains the single authority.

## Decisions

### D1 — One clock, in the model

The playback clock lives in the model. Backends render the current position and never own the timing, which is what keeps an SVG preview, a native window and a terminal in agreement.

### D2 — Keyframes are deltas

A keyframe is the same serializable delta the model already produces. This is the decision that makes live and scripted animation one feature.

### D3 — A stream is a keyframe source

A streaming source advances a running timeline and is ignored by a paused one. The window of keyframes is bounded, so memory does not grow with the stream.

### D4 — Interruptible, never jumping

A new keyframe arriving mid-animation re-targets from the current interpolated position.

## Risks / Trade-offs

- A paused timeline silently ignoring a stream can look like data loss; the state is exposed and the UI shows the queue depth.

