# Chart implementation patterns

This file is the engineering contract for every change that implements part of
the chart surface. It is normative for the changes listed in the change grid
below, and it is referenced by `AGENTS.md` and by `openspec/config.yaml`.

The chart surface is specified in [`docs/FDRs/`](README.md): six core records
(`core/`) and thirty recipe records (`recipes/`). This file does not repeat
what those records say about a chart's behaviour; it fixes **how** the
behaviour is built, so that thirty implementations produced by thirty agents
still compose into one library.

## Status of the grid

The change grid below is a **first scaffold**. It is written before any of the
changes is implemented, and it is expected to be revised once the first waves
land. Concretely: after `workspace-bootstrap`, `figure-builder-core`,
`data-source-layer` and `render-svg-backend` are merged, the remaining
changes must be re-read against the code that actually exists and adjusted —
dependencies, file ownership and task granularity included. A change in the
grid is a hypothesis about how the work decomposes; the first merged wave is
the evidence that corrects it. Revising the grid is itself a small `chore`
change, not something done silently.

## P1 — One capability, one change, one chart

- A change introduces exactly one durable capability, under
  `openspec/specs/<capability>/spec.md`, and the capability is named for the
  behaviour, not for the work (`scatter-plot`, not `add-scatter-plot`).
- A recipe change implements exactly one chart, defined by exactly one FDR.
- Shared machinery never rides along in a recipe change. If a recipe appears
  to need a new mark, stat, move or scale variant, the variant is a change to
  the core capability first; the recipe then composes it.

Rationale: a capability is the unit of review, and a chart is the unit a
reader can hold in their head. Everything larger hides review debt.

## P2 — Builder surface

Every figure and artist builder follows the foundation's rules without
exception:

- the builder is the only public constructor; the struct's fields are private;
- `backend(SVBackendKind)` is **required**, with no `Automatic` variant;
- `execution_mode(SVExecutionMode)` defaults to `Automatic`;
- `build()` transfers the backend kind into the built figure and performs no
  backend dispatch;
- every operation (`render`, `update`, `save`, `show`) dispatches to the
  concrete `SVRenderBackend` and records the configured kind in its span.

```rust
// shape only — the option names are the FDR's, the syntax is illustrative
SVScatterPlot::new()
    .data_source(source)
    .x("weight")?
    .y("height")?
    .series("species")
    .backend(SVBackendKind::Svg)
    .theme(theme)
    .build()?
    .render()
```

## P3 — Module layout per artist

Any `.rs` file that would exceed 200 lines becomes a folder module. The shape
is fixed, so that an agent opening a chart folder knows where everything is:

```
src/recipes/<family>/<chart>/
  mod.rs                     pure dispatcher: declares submodules, re-exports
  builder.rs                 the builder and its state machine
  core_implementation.rs     the chart's own logic (composition, layout hints)
  <chart>_tests.rs           tests, declared at the end of mod.rs
```

Rules that follow from `AGENTS.md` and are repeated here because agents get
them wrong:

- `mod.rs` carries **no implementation logic**;
- tests live in the companion module, never inline except at the end of a
  file;
- no global `tests/` directory;
- mock data in borrowed slices, plus `ndarray` under `data-ndarray`, plus
  synthetic streams.

## P4 — Registration by family, never by central list

A chart is registered in its family's `mod.rs`, not in the crate root and not
in a central registry:

```
src/recipes/
  relational/mod.rs       <- owns scatter, line, area, step, band, regression
  distribution/mod.rs     <- owns the thirteen distributional charts
  matrix/mod.rs           <- owns the six matrix charts
  multivariate/mod.rs     <- owns the five multivariate charts
```

The crate root declares the four family modules and nothing else. The family
`mod.rs` is owned by the **batch integrator** for that family: recipe changes
append one line to a marked block, and the integrator resolves the ordering.

This is what makes parallel work possible. A central registry would turn every
recipe change into a conflict with every other recipe change.

## P5 — Kernels are closed enumerations

`SVMark`, `SVStat`, `SVMove` and `SVScale` are closed enumerations, dispatched
exhaustively. A recipe change **never** adds a variant. The variants are
exactly those enumerated in the core FDRs:

```
SVMark   Dot Line Area Bar Ribbon Text Image Wedge
SVStat   Bin Count Aggregate KernelDensity EmpiricalCdf Percentile Fit
SVMove   Stack Dodge Jitter Shift Swarm Normalize
SVScale  Continuous Nominal Temporal Boolean
```

If implementation shows that a variant is missing, that is a core change with
its own spec delta — never a silent addition inside a recipe.

## P6 — The data boundary is enforced once

Every recipe operation that consumes data:

1. accepts `SVDataSource` and resolves it to `SVColumnView` **once**;
2. never densifies a sparse view;
3. never blocks the render thread on a streaming view;
4. if the mathematics needs full-data geometry, rejects the streaming view up
   front with `UnsupportedRepresentation` and names the alternative path in
   the spec scenario.

The parity test is part of acceptance for every recipe: dense, sparse and
streaming inputs of the same data must produce the same output within the
tolerance recorded in the recipe's FDR.

## P7 — One error type, failures at build time

`SVError` is the single public error enumeration. A build fails when the
configuration is invalid or the backend cannot host it (per the capability
matrix), never at render time with a silent degradation.

## P8 — Observability is part of the operation, not an afterthought

Every public operation emits one span carrying at least:

| Field | Value |
|---|---|
| `backend.kind` | the configured `SVBackendKind` |
| `representation` | `Dense`, `Sparse` or `Streaming` |
| `points` | the number of drawn marks |
| `duration` | filled by the span |

A backend that renders nothing still emits the span; uniformity is the point.

## P9 — Tests are the executable specification

Every implementation ships its companion `*_tests.rs` with:

- a unit test per public option that changes output;
- a parity test for dense, sparse and streaming input;
- a concurrency test where the recipe claims thread safety;
- an SVG snapshot test for the documentation figure;
- a metric assertion for the streaming path.

Tests are written first, as required by the `tdd` skill.

## P10 — The eight tasks of a recipe change

Every recipe change has the same eight tasks, so that a subagent can execute
one without inventing a plan:

1. module, builder and composition, tests first;
2. parity and concurrency tests;
3. the example program under `examples/<chart>.rs`;
4. the generated figure under `docs/figures/<chart>.svg`, plus the mirrored
   copy on the book branch;
5. the mdBook chapter and its `SUMMARY.md` entry, on the book branch;
6. the FDR status changed to `shipped`, with the chapter and example linked;
7. the `CHANGELOG.md` entry;
8. the independent acceptance review against the FDR.

## P11 — Naming

Names are spelled out. The only abbreviation is the project prefix `sv`/`SV`.
Recipes end in `Plot` (`SVScatterPlot`, `SVHistogramPlot`); kernels carry
their noun (`SVMark`, `SVStat`, `SVMove`, `SVScale`, `SVTransition`,
`SVAnimation`).

## Scope alignment

In scope, with a full FDR and a change:

| FDR | Capability | Change | Wave |
|---|---|---|---|
| core-mark | `mark-rendering` | `mark-rendering` | 3 |
| core-stat | `statistical-transforms` | `statistical-transforms` | 4 |
| core-move | `positional-moves` | `positional-moves` | 4 |
| core-scale | `scale-mapping` | `scale-mapping` | 2 |
| core-transition | `mark-transitions` | `mark-transitions` | 5 |
| core-animation | `animation-timeline` | `animation-timeline` | 5 |
| relational (6) | one capability each | 6 changes | 6.1 |
| distributional (13) | one capability each | 13 changes | 6.2 |
| matrix (6) | one capability each | 6 changes | 6.3 |
| multivariate (5) | one capability each | 5 changes | 6.4 |

Out of scope, catalogued but not specified: everything in
[`deferred.md`](deferred.md). A deferred chart becomes a change only through
the promotion criteria stated there.

## The change grid

```
WAVE 0  workspace-bootstrap

WAVE 1  figure-builder-core | data-source-layer | theme-tokens

WAVE 2  scale-mapping | layout-solver

WAVE 3  mark-rendering | chart-decorations

WAVE 4  statistical-transforms | positional-moves | render-svg-backend

WAVE 5  mark-transitions | animation-timeline | render-native-backend

WAVE 6  recipes, four parallel batches
        6.1 relational       scatter line area step band regression
        6.2 distributional   histogram density density-contour ecdf rug box
                             violin boxen strip swarm bar count point
        6.3 matrix           heatmap contour density-heatmap image hexbin
                             clustered-heatmap
        6.4 multivariate     parallel-coordinates parallel-categories splom
                             pair joint

WAVE 7  render-html-backend | render-tui-backend | render-wasm-backend
        render-framework-components | ui-kit-controls

WAVE 8  dashboard-composition | scripting-engines

WAVE 9  python-bindings
```

The machine-readable form of this grid, with per-node dependencies,
file ownership and gates, is `temporary/dag.yaml`. The grid here is durable
and reviewed; the DAG is operational and may be regenerated.

## Parallelism rules

Two changes may run concurrently only if all of the following hold:

1. their `touches` sets are disjoint;
2. they are in the same wave, or the earlier wave is fully merged;
3. neither one modifies a file owned by the other's integrator.

Registration files (`recipes/<family>/mod.rs`) are shared. They are written by
the batch integrator after the batch's implementations merge, never by the
individual recipe agents.

## Revision policy

This grid is scaffolded ahead of implementation by design, so that work can be
dispatched to subagents. It is expected to change. A revision is recorded as:

- an edit to this file, when the change is in the decomposition or the
  dependency graph;
- an edit to `temporary/dag.yaml`, when the change is operational;
- a `chore` change when the revision alters an already-open change's scope.

The signal to revise is concrete: a wave merges and the next wave's
assumptions do not compile, or an implementation reveals that a capability
boundary was drawn in the wrong place.
