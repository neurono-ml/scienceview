# Chapter 13 — Transitions: making a figure move without losing track of its marks

> This chapter explains `SVTransition`, the kernel that turns a sequence of
> static states into a continuous experience. It is the transition change of
> wave 5 and assumes what earlier chapters established: the data boundary
> (chapter 4), the theme's motion tokens (chapter 3) and the mark kernel's
> interpolable-property table (chapter 8). No Rust background is assumed.
>
> A scale answers *where does this value sit*. A mark answers *what shape do I
> draw there*. A transition answers a question about time: *when the figure
> changes, where did each mark come from, and how does it travel to where it is
> going?*

## 1. The problem: a chart that blinks

Imagine a bar chart of revenue per country that refreshes every second. On each
frame the data changes and the figure is redrawn. If every mark is simply placed
at its new position, the eye sees a flash: last frame's bars vanish, this
frame's bars appear, and nothing connects the two pictures. The viewer cannot
tell whether the bar for Brazil grew, whether it moved, or whether Brazil was
replaced by a different bar that happens to sit beside it.

The fix could live in every chart type — the bar chart tweens its bars, the dot
plot tweens its dots, the area chart tweens its vertices — and then each chart
would own a slightly different copy of "move from the old position to the new
one", with different opinions about what happens when a point disappears and
different rounding behaviour.

The project drew a boundary instead: the animation of marks is a kernel of its
own, `SVTransition`, and a recipe composes it. The FDR
(`docs/FDRs/core/core-transition.md`, FDR-005) places it exactly here:

```text
state A --(delta)--> state B
   |                     |
   |                  resolve keys
   |                     |
   +--> enter / update / exit sets
```

The transition sits **between the model's deltas and the backend's frame loop**.
It knows nothing about pixels, windows or file formats: it takes two states of
marks and a progress, and hands back the properties each mark draws at that
moment. That position is what lets the SVG, HTML, terminal, native and
component backends all animate the same figure with the same rules.

## 2. Three ideas to learn first

![Two mark states are matched by typed keys into the enter, update and exit sets; updates interpolate the interpolable properties while discrete ones swap at the end; the caller-supplied progress produces one deterministic frame, and reduced motion or a static backend resolves it to the final state](assets/transition-lifecycle.svg)

**Matching is by key, never by position.** This is the single most important
rule in the layer. If the transition paired the first mark of state A with the
first mark of state B, inserting one value at the front of a dataset would
silently re-target every later mark: each one would think it had moved one place
and animate to a different bar. That is the classic animation bug, and the
kernel refuses to make it possible. Each mark carries a *match key* — a category
code, an index, an explicit identity or a deterministic hash — and the
transition pairs marks whose keys are equal.

**Only the properties the mark kernel declares interpolable tween.** Position,
size, colour, opacity and font size move continuously. Shape, dash pattern and
font family are discrete: there is no meaningful colour halfway between a circle
and a square, so those properties hold their previous value and swap when the
transition completes. The list is not written twice — the transition consumes
`sv_mark_property_table()`, the table that lives next to the properties it
describes (chapter 8). If a future mark adds a property, the two layers cannot
disagree about it.

**The kernel owns no clock.** Time belongs to the caller — the animation
timeline of the backend. The transition evaluates a frame from a progress in
`0..=1`, or maps a caller duration onto that progress using the theme's motion
tokens. Nothing inside the kernel reads the wall clock, which is what makes a
frame deterministic: the same plan and the same progress always produce the same
frame, on every platform and in every thread.

## 3. The data contract: keys, states and properties

### Match keys

`SVMatchKey` is a closed enumeration, in the sense of the closed kernels of
chapters 8–11:

| Rule | Meaning | Example |
|---|---|---|
| `Category` | the value of a categorical column, as its numeric slot code | the country of a bubble |
| `Index` | the logical position in the column, stable across states | a time step |
| `Identity` | an explicit identifier column, as its numeric code | a sensor id |
| `Derived { seed }` | a deterministic hash of the discriminating value | a computed bucket |

Resolving a rule produces a **typed** `SVMatchKeyValue`: the variant carries the
rule as well as the code. That detail matters. If keys were bare numbers, a
category code `7.0` would equal an identity code `7.0`, and two different things
in the same figure could be paired by accident. Typed keys make that
impossible — `SVMatchKeyValue::category(7.0)` and
`SVMatchKeyValue::identity(7.0)` are different values, and the companion test
`a_category_key_never_equals_an_identity_key_with_the_same_code` pins it.

The numeric code is stored as its 64-bit representation rather than as a
floating-point number, so equality and hashing are exact and identical across
platforms. The constructors reject a non-finite code (`NaN` or an infinity)
up front: a mark without a finite identity cannot be matched, and the resolution
reports the position of the offending value with
`SVError::NonFiniteValue`.

### Reading keys from the data boundary

Keys come from a column, and the data boundary has three representations
(chapter 4). `sv_resolve_match_keys` dispatches on the representation exactly
once, following the parity rule of the spec `data-view-boundary`:

- a **dense** column is read as a slice, in order;
- a **sparse** column is read over its stored observations and never expanded —
  an implicit zero is not an observation, so it carries no mark and no key;
- a **streaming** column is read from the bounded window snapshot it already
  carries, so a live figure never blocks the render thread.

`Index` needs no value at all: every logical position is a stable key, which is
why the same column resolves the same index keys in all three representations.
The parity tests (`every_representation_resolves_the_same_category_keys`,
`index_keys_are_identical_for_every_representation`) pin that agreement.

### Two marks cannot share a key

A repeated key would make the match ambiguous: which of the two marks is the one
that persisted? The kernel refuses it before any interpolation, with
`SVError::InvalidConfiguration` naming the repeated key and both positions. The
duplicate check runs on both states, so the error cannot depend on which side of
the transition is read first, and it is deterministic: the first repetition in
state order is the one reported.

### States

`SVMarkState` is one mark at one moment: a key plus a bag of
`SVMarkPropertyValue`. The bag keeps its entries in the order of the mark
kernel table, so iterating a state is deterministic and two states with the same
values compare equal. The values are typed per property — a position is a
`SVMarkPoint`, a colour is an `SVColour`, a dash pattern is an
`SVDashPattern` — so the transition never has to guess which channel a value
addresses.

## 4. The three sets

`SVTransition::plan(&previous, &next)` computes three sets from the two states:

| Set | Condition | Treatment |
|---|---|---|
| enter | the key is in the next state but not in the previous one | the enter rule applies |
| update | the key is in both states | the properties interpolate |
| exit | the key is in the previous state but not in the next one | the exit rule applies |

The sets are computed from the states themselves, exactly as design decision D2
records: there is no second state representation that could diverge from the
first. A mark that changes value but keeps its key is an update; a mark that
changes key is an exit plus an enter. The test
`matching_never_pairs_marks_by_position` is the executable statement of that
rule: two marks with keys `4` and `5` are followed by marks with keys `5` and
`6`, and the kernel reports one update (key `5`), one enter (key `6`) and one
exit (key `4`) — never a pair by position.

### Enter and exit rules

An entering mark has no previous value to interpolate from, and an exiting mark
has no target. The FDR gives each a small closed vocabulary:

| Enter rule | Draws at progress 0 |
|---|---|
| `None` | the target state, immediately |
| `FadeIn` | the target with opacity zero |
| `ScaleIn` | the target with size zero |
| `SlideIn` | the target shifted down by `SV_TRANSITION_SLIDE_DISTANCE` |

| Exit rule | Draws after progress 0 |
|---|---|
| `None` | nothing — the mark disappears immediately |
| `FadeOut` | the source opacity, falling to zero |
| `ScaleOut` | the source size, shrinking to zero |
| `SlideOut` | the source position, sliding down |

Enter and exit are simultaneous by default, which is the FDR's current answer to
its first open question. An exit stops being drawn when the transition completes
(`progress == 1.0`), because at that moment the figure must be exactly state B;
an enter is drawn from the first frame, invisible under `FadeIn` but present,
so a backend can interpolate its paint.

## 5. Interpolation: the table decides, not the transition

The update set is where the mark kernel's property table earns its keep. For
every property the two states share, in table order:

- if the property is **interpolable** and the caller's mask includes it, the
  value moves continuously between the two endpoints;
- otherwise the value holds the previous state until the effective progress
  reaches `1.0` and then becomes the target.

The mask, `SVInterpolatedProperties`, can exclude a property but can never make
a discrete property tween: `contains` consults the table's `is_interpolable`
first. That is design decision D3, and the test
`the_mark_kernel_table_drives_the_transition` walks the real table and asserts
that interpolable properties move at the midpoint while discrete ones do not.

The discrete decision uses the **raw** effective progress, not the eased one.
An easing curve may overshoot its range (a cubic Bézier can rise above one
before settling), and a shape swap driven by an overshooting value would flicker
mid-flight. Swapping only at `1.0` keeps the shape stable for the whole
transition.

Colour interpolation happens channel by channel, in floating point with a
rounding step, and every interpolation is exact at both endpoints: at progress
`0.0` the value is state A's value and at `1.0` it is state B's. The companion
tests assert the endpoints and the midpoint for position, size, opacity, font
size and colour, and the swap point for shape, dash pattern and font family.

## 6. Motion follows the theme, and reduced motion wins

The durations and easings of a transition are not literals. They are **motion
tokens**, resolved from the compiled theme exactly like colours and spacing
(chapter 3):

| Token | Meaning |
|---|---|
| `motion.duration.fast`, `motion.duration.normal`, `motion.duration.slow` | named durations |
| `motion.easing.standard`, `motion.easing.emphasized`, `motion.easing.decelerate` | named easings |

With the `theme` feature, `SVMotionStyle::from_motion_tokens` reads the normal
duration and the standard easing, and records the theme's reduced-motion flag.
Without the feature — the core takes no dependency it does not need —
`SVMotionStyle::built_in` supplies the same documented defaults the default
theme carries: fast 120 ms, normal 240 ms, slow 400 ms, and the standard
cubic-bezier(0.2, 0, 0, 1). The same figure therefore looks the same on a build
with the theme and one without it.

**Reduced motion is an accessibility setting, not a per-call flag.** When the
theme declares it, `SVMotionStyle::duration` resolves to zero and
`is_reduced_motion` is true; the plan then draws its final state at *every*
progress. There is no override, deliberately: design decision D4 says the layer
honours the theme unconditionally, and the test
`a_reduced_motion_plan_draws_the_final_state_at_any_progress` proves it by
comparing frames at four different progresses against the final one.

The easing curves themselves are evaluated in-crate, with the same bisection
solver the theme uses, so a terminal or native backend samples the same curve a
browser applies declaratively. A linear curve returns the progress itself, so an
interpolation that uses it is exact.

## 7. The missing clock: progress and time

The kernel is a pure function of the plan and a progress. Two operations express
that:

- `SVTransitionPlan::evaluate(progress)` clamps a finite progress into
  `0..=1` and rejects a non-finite one with
  `SVError::InvalidConfiguration`. Rejecting rather than silently substituting
  zero keeps a broken timeline observable instead of hiding it.
- `SVTransitionPlan::progress_at(elapsed)` maps a caller-supplied duration onto
  the progress, subtracting the per-series delay first. A mark scheduled later
  holds its previous state until its turn.

The default transition is an instant swap: a zero duration, so
`progress_at` returns `1.0` immediately. A recipe that wants motion resolves its
tokens from the theme or sets a duration explicitly.

Because evaluation reads nothing but the plan and the progress, it is safe to
evaluate from many threads at once. `SVTransitionPlan` and
`SVTransitionFrame` are `Send + Sync`, and the concurrency test evaluates the
same plan on eight scoped threads and compares every frame to the expected one.

## 8. The backend contract: never pretend to have a clock

A transition needs a backend that can update the figure in place; a static
export has no frame loop, so it must not animate. The FDR records the rule and
the change implements it by **deriving** animation support from the existing
capability matrix rather than adding a parallel field:

`sv_transition_support` asks the matrix whether the backend declares
`InPlaceUpdate`. If it does — the terminal, native, WebAssembly, component and
Rust-native backends — the transition animates. If it does not — the SVG and
HTML exports — the backend is `FinalStateOnly`, and
`evaluate_for_backend` resolves every progress to the final state. This is a
legitimate degradation *provided* the backend declares it, which is exactly what
the matrix does.

The tests `a_static_export_backend_declares_transition_unsupported`,
`a_static_backend_draws_the_final_state_at_any_progress` and
`transition_support_agrees_with_the_capability_matrix` make the contract
executable, and the last one walks every backend kind so a new backend cannot
forget the rule.

## 9. The road not taken

**Positional matching.** Pairing marks by their order is trivial to implement
and produces beautiful animation for a dataset that never changes shape — and
nonsense for one that does. The kernel rejects it, and the tests state why.

**A second property table.** Copying the interpolable list into the transition
would let the two layers drift: a mark adds a property, the transition keeps its
own opinion, and a figure animates something it should not. The transition reads
the mark kernel's table instead, which is why `the_mark_kernel_table_drives_the_transition`
exists.

**A global animation clock in the kernel.** Reading the wall clock inside
`evaluate` would make frames non-deterministic, impossible to test without
sleeping, and impossible to evaluate concurrently. Time is the caller's
responsibility.

**A per-call "ignore reduced motion" flag.** Accessibility settings are not a
hint. The theme owns the decision, and the layer offers no override.

**Interrupting a transition mid-flight.** The FDR leaves the exact rule open.
The kernel makes the safe part possible today: because evaluation is pure, a
caller that wants to re-target can evaluate the current frame and use its values
as the next transition's source, with no jump and no kernel state.

## 10. Summary

`SVTransition` is a small pure kernel with a large contract:

- marks are matched by typed keys, resolved once from dense, sparse or streaming
  columns, with duplicate keys rejected;
- the enter, update and exit sets are computed from two states;
- interpolation and discrete swaps follow the mark kernel's table, with a
  caller-controlled mask that can exclude but never corrupt a property;
- motion comes from the theme's tokens, with documented built-in defaults and an
  unconditional reduced-motion collapse;
- the kernel owns no clock: progress is supplied, clamped and validated;
- a static backend draws the final state instead of inventing a frame loop.

Every later chart animation is built on this kernel without a recipe
implementing its own tweening: a recipe supplies match keys and two states, and
the transition does the rest.
