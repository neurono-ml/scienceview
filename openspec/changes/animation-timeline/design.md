# Design

## Context

The design deliberately avoids a second state pipeline: the timeline is a
sequence of deltas, and the model remains the single authority. The timeline
owns the clock, the window and the interpolation between adjacent deltas; it
does not own the mutation that applies a delta to a figure.

## Decisions

### D1 — One clock, in the model

The playback clock lives in the model. Backends render the current position and never own the timing, which is what keeps an SVG preview, a native window and a terminal in agreement.

### D2 — Keyframes are deltas; application is deferred

A keyframe is the same serializable delta the model already produces. This is
the decision that makes live and scripted animation one feature, but the
guarantee is narrower than "the delta is applied through the same pipeline":
this kernel stores the delta, keeps the sequence ordered, interpolates between
neighbours and re-targets an interrupt. The `model-events` capability owns the
transactional mutation pipeline that applies a delta to a figure, and that
pipeline does not exist yet. Applying a sampled delta is therefore a documented
follow-up of the `model-events` change, not a claim of this kernel.

### D3 — A stream is a keyframe source

A streaming source advances a running timeline and is ignored by a paused one. The window of keyframes is bounded, so memory does not grow with the stream.

### D4 — Interruptible, never jumping

A new keyframe arriving mid-animation re-targets from the current interpolated position.

## Deferred — application through the model-events pipeline

The timeline exposes `sample()` / `sample_at()` as an `SVKeyframeDelta`. A
caller that wants the figure to change hands the delta to the future
`model-events` transactional mutation pipeline. The two capabilities meet at
one seam:

```
SVAnimation::sample() -> SVKeyframeDelta --(model-events)--> committed mutation -> figure
```

Until `model-events` lands, a caller can inspect the sampled delta but no
public operation mutates a figure from it. This is why the FDR's first
acceptance item stays unchecked with the note naming the mutation pipeline.

## Risks / Trade-offs

- A paused timeline silently ignoring a stream can look like data loss; the state is exposed and the UI shows the queue depth.
- The timeline can sample a state no backend can apply until `model-events`
  lands; the kernel is still exercised by its own parity and re-targeting tests,
  and the FDR keeps the application item unchecked.
