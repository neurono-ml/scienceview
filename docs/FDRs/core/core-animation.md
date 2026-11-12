# FDR-006 — `SVAnimation`

| Field | Value |
|---|---|
| Id | FDR-006 |
| Name | `SVAnimation` |
| Family | Core — temporal |
| Status | shipped (kernel); backend frame and reduced-motion theme pending |
| Layer | Core |
| Public chapter | `ch14-animation-timeline.md` |
| Example program | _pending_ |

## Purpose

`SVAnimation` turns a sequence of figure states into a playable timeline: it
holds the keyframes, decides how the state between two keyframes is computed,
and exposes the playback controls.

The model is deliberately unified. There is no separate "animation" chart
type and no separate "live update" chart type; there is a *timeline*, and a
timeline can be driven by a fixed list of keyframes or by a stream of
arriving states. A live chart is an animation whose keyframe source is a
stream.

```
SVAnimation
  |-- keyframes      a list of states, or a stream of arriving states
  |-- interpolation  how the state between two keyframes is computed
  |-- timing         duration, loop, direction
  `-- playback       the clock: playing, position, speed
```

## Sources

Plotly's `frames` and its animation controls are the closest reference; bokeh
offers `FuncAnimation` and custom callbacks for the same effect.

- plotly `Frame` — <https://plotly.com/python/reference/#plotly.graph_objects.Frame>
- plotly layout animation — <https://plotly.com/python/reference/#plotly.graph_objects.layout.Animation>
- bokeh `FuncAnimation` — <https://docs.bokeh.org/en/latest/docs/reference/models/transforms.html>

## Recipe

```
keyframe A --(transition + interpolation)--> keyframe B
```

The interpolation between two keyframes is a transition: marks are matched by
key, and the properties in the interpolable set are computed for the current
playback position.

## Data contract

### Keyframe sources

| Source | Kind | Behaviour |
|---|---|---|
| `Fixed` | `Vec<Keyframe>` | a fixed list of states |
| `Streaming` | an `SVDataSource` in streaming mode | each arriving batch is a keyframe |

A `Streaming` source is *not* an infinite list. The animation holds the most
recent window of keyframes and drops the oldest, so that memory is bounded.
This is the same rolling-window discipline the foundation requires of
streaming data sources.

### Keyframe content

A keyframe carries a **delta**, in the same serialized form the foundation's
`model-events` capability defines. This is the reason animation needs no
parallel state pipeline: the timeline is a sequence of deltas, and the model
is the single authority.

## Visual channels

An animation interpolates the properties listed in
[core-transition](core-transition.md). The set is closed and shared: the
transition layer decides what may tween, and the animation layer only decides
*when*.

## Configuration

| Option | Type | Default | Notes |
|---|---|---|---|
| `source` | `SVKeyframeSource` | `Fixed` | `Fixed` or `Streaming` |
| `duration` | `SVDuration` | `1000 ms` | per keyframe |
| `interpolation` | `SVInterpolation` | `Linear` | `Linear`, `EaseIn`, `EaseOut`, `EaseInOut`, `Step` |
| `loop` | `SVAnimationLoop` | `None` | `None`, `Count(n)`, `Forever` |
| `direction` | `SVAnimationDirection` | `Forward` | `Forward`, `Reverse`, `Alternate` |
| `speed` | `f64` | `1.0` | a playback multiplier, adjustable at runtime |
| `playing` | `bool` | `false` | |

Playback is *state*: playing, paused, position, speed. The UI kit exposes
play, pause, step and a scrub slider, dispatching the same action vocabulary
as every other control.

## Theme

Motion tokens, as in [core-transition](core-transition.md):

| Token | Meaning |
|---|---|
| `motion.duration.fast` / `normal` / `slow` | named durations |
| `motion.easing.standard` / `entrance` / `exit` | named easings |

A theme that reduces motion collapses durations to zero, which makes every
animation an instant swap.

## Temporal

The animation *is* the temporal layer. Three rules govern it:

1. **One clock.** The playback clock lives in the model. Backends render the
   current position; they never own the timing.
2. **Interruptible, never jumping.** A new keyframe arriving mid-animation
   re-targets from the current interpolated position.
3. **Bounded memory.** A streaming source keeps a bounded window of keyframes.

## Backend notes

Animation requires in-place update and a clock. See the backend table in
[core-transition](core-transition.md) and the capability matrix in
`backend-capabilities`.

| Backend | Notes |
|---|---|
| `Svg` | a live SVG document can animate through CSS transitions; a written file resolves to the final state |
| `Html` | JavaScript drives the clock, the Rust model stays the authority |
| `Native` | the render loop reads the model clock each frame |
| `Wasm` | the browser's animation frame drives the clock |
| `Tui` | the tick rate bounds the frame rate; the clock advances by the elapsed time |
| `React` / `Vue` / `Angular` | the component subscribes to the model's playback state |
| `Tauri` / `Dioxus` | Rust-native loop |

A backend declares `animation` in its capability matrix, with the supported
sources — a backend that cannot host a streaming keyframe source says so.

## Acceptance

- [ ] A keyframe is a delta in the format `model-events` defines; there is no
      second state representation. _The delta exists and is the only state; the
      `model-events` mutation pipeline it names is not implemented yet._
- [x] A `Fixed` and a `Streaming` source produce the same timeline structure.
- [x] Playback state is observable and drivable through the action vocabulary.
- [x] A new keyframe arriving mid-animation re-targets without a jump.
- [x] A streaming source keeps a bounded window of keyframes.
- [ ] A theme that reduces motion collapses the timeline to instant swaps.
- [ ] Every backend that claims animation support renders a mid-animation
      frame that is a correct interpolation of its two neighbouring keyframes.

## Open questions

- Should the interpolation of a *categorical* channel — a series changing its
  nominal colour — be a cross-fade in colour space, or an instant swap? *Open:
  the transition layer keeps nominal colours stable, so the question is
  deferred to the case where a series is genuinely replaced.*
- Should a streaming source advance the timeline automatically, or only when
  the user presses play? *Current answer: a stream advances a running timeline
  and is ignored by a paused one.*
