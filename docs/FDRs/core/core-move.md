# FDR-003 — `SVMove`

| Field | Value |
|---|---|
| Id | FDR-003 |
| Name | `SVMove` |
| Family | Core — computation |
| Status | shipped (kernel); visual example deferred |
| Layer | Core |
| Public chapter | `docs/src/development-explained/ch11-positional-moves.md` |
| Example program | deferred until the bar recipe exists (see [Example status](#example-status)) |

## Purpose

`SVMove` displaces marks along an axis so that overlapping marks remain
readable, or so that marks can be composed into a whole. It never changes the
data — it changes where a mark sits and how wide it is.

A move is applied *after* the stat and *before* the scale, because it works in
data space: a stack of bars changes the value of a bar, and a dodge changes the
position of a category.

## Sources

seaborn's `Move` objects are the model; plotly and bokeh implement the same
operations inside their chart types.

| `SVMove` | seaborn objects | plotly | bokeh |
|---|---|---|---|
| `Stack` | `Stack` | `Bar` (barmode `stack`) | — |
| `Dodge` | `Dodge` | `Bar` (barmode `group`/`relative`) | — |
| `Jitter` | `Jitter` | `Box`/`Violin` (points) | jittered `Circle` |
| `Swarm` | — | — | — |
| `Shift` | `Shift` | `Box` (`boxpoints` offset) | — |
| `Normalize` | `Norm` | `Bar` (barnorm) | — |

- seaborn — <https://seaborn.pydata.org/api.html#seaborn.objects.Dodge>
- plotly — <https://plotly.com/python/reference/#plotly.graph_objects.Bar>

## Recipe

```
data -> [SVStat] -> values -> [SVMove] -> [SVScale] -> SVMark
```

Several moves may compose, in a fixed order: `Jitter`, `Dodge` or `Stack`,
then `Shift`, then `Normalize`. The order is part of the contract, because
`Dodge` after `Normalize` and `Normalize` after `Dodge` give different
results.

## Data contract

A move consumes derived values plus their **group keys**, and writes back a
displacement and, where relevant, a new value.

| Move | Input | Output |
|---|---|---|
| `Stack` | values + group key | stacked baseline and stacked value |
| `Dodge` | values + group key + category key | displaced position and narrowed width |
| `Jitter` | values + group key | displaced position |
| `Swarm` | values + group key | collision-avoiding displacement |
| `Shift` | values | displaced position |
| `Normalize` | values + group key | value rescaled to a fraction of the group total |

### Representation parity

| Move | Dense | Sparse | Streaming | Note |
|---|---|---|---|---|
| `Stack` | yes | yes | **windowed** | a stack needs the group total; over a stream the total is of the window |
| `Dodge` | yes | yes | yes | only needs group sizes |
| `Jitter` | yes | yes | yes | purely positional |
| `Swarm` | yes | yes | yes | deterministic given the data and its order |
| `Shift` | yes | yes | yes | purely positional |
| `Normalize` | yes | yes | **windowed** | same as `Stack` |

## Visual channels

A move produces positions and widths, not colours. It may change the value a
bar represents (stacking, normalizing), which changes what the bar's height
means — the recipe must document the mapping when it uses such a move.

## Configuration

| Kernel | Option | Type | Default |
|---|---|---|---|
| `Stack` | `mode` | `SVStackMode` | `Full` (`Full`, `Center`) |
| `Dodge` | `gap` | `f32` | `0.0` — gap between dodged groups, as a fraction |
| `Dodge` | `width` | `f32` | `0.8` — total width of the dodged group, as a fraction of the slot |
| `Jitter` | `amount` | `f32` | `0.1` — fraction of the category width |
| `Jitter` | `mode` | `SVJitterMode` | `Uniform` (`Uniform`, `Gaussian`) |
| `Jitter` | `seed` | `Option<u64>` | `None` — a fixed seed makes the jitter reproducible |
| `Swarm` | `width` | `Option<f32>` | `None` — the category width |
| `Swarm` | `marker_extent` | `f64` | the marker size resolved from the theme |
| `Shift` | `amount` | `f64` | `0.0` — displacement in data units |
| `Shift` | `direction` | `SVShiftDirection` | `Horizontal` |
| `Normalize` | `to` | `SVNormalisationTarget` | `Fraction` (`Fraction`, `Percent`) |

`Jitter` with an explicit `seed` is deterministic: the same data and the same
seed always produce the same jitter. Determinism is a testable property, not
an accident.

## Theme

Moves consume no tokens. A move may reduce the effective width of a bar, in
which case the recipe chooses the visual weight.

## Temporal

| Property | Behaviour |
|---|---|
| displacement | interpolates numerically |
| width | interpolates numerically |
| stacked value | interpolates numerically |
| jitter | **does not interpolate** — jitter is re-applied per state, otherwise a mark would drift across the frame; the displacement is a function of the mark's key, not of the frame |
| swarm offset | **does not interpolate** — the swarm is re-laid out per state; the offset is a layout of the new arrangement, not a stored position |

That last row matters: an animated strip plot must not have its points slide
around. The jitter offset is recomputed from the match key, so the point stays
in place while the values around it change.

## Backend notes

Moves are backend-independent. The terminal backend receives the same
displacements and narrows the bars accordingly.

## Example status

The kernel is shipped and exercised by the companion tests, but a standalone
example program would have to draw its result, and the chart recipes that turn a
moved column into a figure are not implemented yet. Rather than shipping an
example that stops at an assertion, the visual example is **deferred**: it lands
with the bar recipe that consumes `SVMoveOutput`, in the same change that adds
`examples/` and its tool-generated figure. Tracking: the bar and stack recipes
in `docs/FDRs/recipes/`, which list this FDR as their move dependency.

## Acceptance

- [x] The composition order of moves is fixed and documented
      (`sequence_tests.rs`).
- [x] `Dodge` and `Stack` produce the same result for dense, sparse and
      streaming inputs of the same data (`parity_tests.rs`).
- [x] `Jitter` with a fixed seed is reproducible across runs
      (`jitter_tests.rs`).
- [x] `Normalize` and `Stack` over a stream use the window total and say so
      (`parity_tests.rs`).
- [x] A move never copies the data it displaces: dodge, jitter and shift share
      the borrowed column, and a value-changing move materialises only the new
      value it computes (`lazy_buffer_tests.rs`).
- [x] `Swarm` guarantees no two points of a category overlap, and the layout is
      deterministic for the same data and order (`swarm_tests.rs`).
- [x] A sparse input is moved without allocating one slot per logical point:
      the working columns keep the stored entries and one documented implicit
      zero (`sparse_boundary_tests.rs`).
- [x] Non-finite values are rejected with the typed error naming the position
      (`non_finite_tests.rs`).
- [x] A swarm width narrower than two marker extents is rejected at sequence
      construction, and a point the bounded search cannot place is a typed
      error rather than a colliding candidate (`swarm_tests.rs`).

## Open questions

- Should `Dodge` and `Stack` be mutually exclusive, or should `Stack` of
  dodged groups be expressible? *Current answer: mutually exclusive in the
  first version; the combination is a `Stack` of `Dodge`d groups and is
  deferred.*
- Should `Jitter` be applied in data space or in display space? *Current
  answer: data space, so the jitter scales with the axis — but this makes
  jitter resolution-dependent, which conflicts with the layout engine's
  backend-independent coordinates. Open.*
