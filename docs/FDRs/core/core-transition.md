# FDR-005 — `SVTransition`

| Field | Value |
|---|---|
| Id | FDR-005 |
| Name | `SVTransition` |
| Family | Core — rendering |
| Status | shipped (kernel); animated example deferred |
| Layer | Core |
| Public chapter | `docs/src/development-explained/ch13-mark-transitions.md` |
| Example program | deferred until a live recipe exists |

## Purpose

`SVTransition` decides what happens to a mark when the figure's state
changes: which marks are new, which persist, which disappear, and how each
group moves to its new place.

It is the layer that turns a sequence of static figures into a continuous
experience. Without it, a chart that updates replaces everything at once and
the eye loses the correspondence between "before" and "after".

The transition layer works on **keys**, not positions. This is the single most
important rule in the layer: marks are matched by a stable identity, so that a
mark that moved is recognized as the same mark.

## Sources

No reference library has this as a separate concept — plotly's `frames` and
bokeh's `CustomJS` implement the effect ad hoc. Seaborn is static. The
concept comes from the foundation's `model-events` capability, which defines
the delta as the unit of change.

- plotly `frames` — <https://plotly.com/python/reference/#plotly.graph_objects.Frame>
- foundation `model-events` — `openspec/changes/foundation-core-architecture/specs/model-events/spec.md`

## Recipe

```
state A --(delta)--> state B
   |                     |
   |                  resolve keys
   |                     |
   +--> enter / update / exit sets
```

A transition is computed from the difference between two states, expressed as
the deltas the model already produces.

## Data contract

### Match keys

Each mark belongs to a *series*, identified by a stable key. Within a series,
a mark is identified by a **mark key**, which is one of:

| Mark key | Meaning | Example |
|---|---|---|
| `Category` | the value of a categorical column | the country of a bubble |
| `Index` | the position in the data, stable across states | a time step |
| `Identity` | an explicit identifier column | a sensor id |
| `Derived` | a hash of the mark's discriminating fields | a computed bucket |

The default for a recipe is stated in its own FDR. A recipe whose marks have
no natural identity must say so, because an animation without keys degenerates
into a cross-fade.

### The three sets

| Set | Condition | Treatment |
|---|---|---|
| enter | the key is in state B but not in state A | the enter rule applies |
| update | the key is in both | properties interpolate |
| exit | the key is in state A but not in state B | the exit rule applies |

## Visual channels

A transition governs the properties the mark kernel declares interpolable:
position, size, colour, opacity and font size. It does not govern shape, dash
pattern or font family: those are discrete and swap. The list is
`sv_mark_property_table()`, so the two layers cannot disagree about it.

## Configuration

| Option | Type | Default | Notes |
|---|---|---|---|
| `duration` | `SVDuration` | `0 ms` | `0` means an instant swap |
| `easing` | `SVEasing` | `EaseInOut` | `Linear`, `EaseIn`, `EaseOut`, `EaseInOut` |
| `delay` | `SVDuration` | `0 ms` | a per-series stagger |
| `enter_rule` | `SVEnterRule` | `FadeIn` | `None`, `FadeIn`, `ScaleIn`, `SlideIn` |
| `exit_rule` | `SVExitRule` | `FadeOut` | `None`, `FadeOut`, `ScaleOut`, `SlideOut` |
| `match_key` | `SVMatchKey` | recipe-defined | see above |
| `interpolate` | `SVInterpolatedProperties` | all interpolable | which properties tween |

## Theme

Transitions consume the theme's motion tokens, so that the timing and easing
of a figure follow its visual identity:

| Token | Meaning |
|---|---|
| `motion.duration.fast`, `motion.duration.normal`, `motion.duration.slow` | named durations |
| `motion.easing.standard`, `motion.easing.emphasized`, `motion.easing.decelerate` | named easings |

A theme may declare that motion is reduced — the accessibility setting that
collapses all durations to zero. The transition layer honours it
unconditionally.

## Temporal

This *is* the temporal layer; the rules are those of the property table above.
The one rule worth restating: a mark that changes value but keeps its key
interpolates; a mark that changes key is an exit plus an enter.

## Backend notes

A transition requires a backend that supports in-place update, per the
foundation's `backend-capabilities`. A backend that only exports a single
file resolves a transition to its final state — it has no clock, so it draws
the end of the animation. This is a legitimate degradation *provided* the
backend declares it: animation support derives from the matrix's
`in_place_update` capability, so a backend that does not declare it is
`FinalStateOnly` instead of inventing a clock.

| Backend | Support |
|---|---|
| `Svg` | export-only: a static file resolves to the final state |
| `Html` | export-only document today: resolves to the final state |
| `Native` | frame loop on the render thread, in-place |
| `Wasm` | request-animation-frame driven |
| `Tui` | redraw per tick |
| `React` / `Vue` / `Angular` | component state transitions |
| `Tauri` / `Dioxus` | Rust-native frame loop |

## Acceptance

- [x] Marks are matched by key; a mark that persists across states is
      recognized as the same mark and interpolates.
- [x] Enter, update and exit are computed between two states, and a key that
      repeats within one state is rejected before interpolation.
- [x] A property that is not interpolable swaps rather than tweens.
- [x] A theme that reduces motion collapses all durations to zero and draws the
      final state at every progress, with no way to switch the declaration off.
- [x] A backend without in-place update resolves the animation to its final
      state, with animation support derived from the `in_place_update`
      capability of the matrix.
- [x] Evaluation owns no clock: a finite progress outside `0..=1` is clamped and
      a non-finite one is rejected.

## Open questions

- Should the exit rule be allowed to delay the removal of a mark while its
  replacement enters, or are enter and exit always simultaneous? *Current
  answer: simultaneous by default, with an optional overlap expressed as a
  delay on the enter rule.*
- Should transitions be interruptible — a new state arriving mid-animation
  re-targeting the current position? *Current answer: yes, re-target from the
  current interpolated position, never jump. Open on the exact rule.*
