# Design

## Context

`SVLinePlot` is one of the scientific-core recipes specified in
`docs/FDRs/recipes/relational-line-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale x, y, colour] -> SVMark::Line`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`, `group`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

The shipped geometry is transition-ready: each line carries a stable
`(series, group)` key and stable per-series colour and dash, so a future
transition layer can match points by `x` and swap the dash without re-deriving a
position. Keyed interpolation and dash swapping are deferred to the animation
recipe change and are not claimed by this change.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4). The public module path is
`scienceview::sv_recipes::sv_relational::sv_line_plot`, reached through
`#[path]` re-exports so the file layout stays per-recipe.

### D5 — Data source and scene contract

The builder binds an `SVDataSource` and the names of its columns, so a mistyped
name is a typed `SVError::ColumnNotFound` instead of a silently wrong column.
Borrowed dense, sparse and streaming views are reached through an in-memory
source adapter and never densified. The composition is backend-agnostic: the SVG
path serialises it and the native path rasterises the same scene. Every
unwired backend fails explicitly at render time.

## Open Questions

- Should the confidence interval be a separate mark (`Ribbon`) inside this recipe, or a sibling recipe (`SVBandPlot`) that the line recipe may include? *Current answer: a sibling, so a band can be used alone.*



## Implementation outcome

The recipe was implemented as `src/recipes/relational/line_plot/`
(public path `scienceview::sv_recipes::sv_relational::sv_line_plot`) with the
builder, the composition, the series/ordering/aggregation logic, interpolation,
decorations and the scene serializers in separate files; `mod.rs` is a pure
dispatcher. It composes `SVStat::Aggregate`, `SVMark::Line`, `SVMark::Dot` and
`SVMark::Ribbon` and adds no kernel variant. The builder reads named columns
from an `SVDataSource`, so a mistyped name is a typed `ColumnNotFound`, and it
draws one line per `(series, group)`. Dense, sparse and streaming channels share
one extraction path. The same composition serialises as SVG and rasterises as a
native frame. The SVG figure is `docs/figures/line_plot_recipe.svg` (renamed
from the planned `line_plot.svg` to avoid colliding with the existing
`line_plot_svg` example). The book chapter is
`docs/src/development-explained/ch17-line-plot.md`.
