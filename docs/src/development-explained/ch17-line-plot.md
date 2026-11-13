# Chapter 17 — A line plot: turning columns into a composed chart

A line plot answers a deceptively simple question: *how does a value change as
we move along an ordered axis?* The axis is usually time, but it can be any
continuous measurement. The chart draws one line per series and, when several
observations share the same position, it can summarise them before drawing.

This chapter follows `SVLinePlot`, the first recipe of the relational family.
It is a *recipe*: it introduces no new statistical, geometric or rendering
kernel. It arranges the kernels that already exist into one named chart, the
same way a kitchen follows a recipe with ingredients it did not invent.

```mermaid
flowchart LR
    A["x, y, series, group<br/>dense · sparse · streaming"] --> B["SVStat::Aggregate<br/>(optional)"]
    B --> C["ordering by x<br/>gaps or bridging"]
    C --> D["SVLineInterpolation<br/>linear · step · spline"]
    D --> E["SVScale<br/>x and y"]
    E --> F["SVMark::Line<br/>+ Dot markers<br/>+ Ribbon interval"]
    F --> G["SVDecoration<br/>axes · grid · legend · title"]
    G --> H["SVSvgDocument<br/>self-contained SVG"]
    style A fill:#dbeafe,stroke:#2563eb,color:#1e3a8a
    style B fill:#fef3c7,stroke:#d97706,color:#78350f
    style C fill:#fce7f3,stroke:#db2777,color:#831843
    style D fill:#ede9fe,stroke:#7c3aed,color:#4c1d95
    style E fill:#dcfce7,stroke:#16a34a,color:#14532d
    style F fill:#ffedd5,stroke:#ea580c,color:#7c2d12
    style G fill:#e0f2fe,stroke:#0284c7,color:#0c4a6e
    style H fill:#f3e8ff,stroke:#9333ea,color:#581c87
```

## The data contract: long format, not wide format

A line plot does not receive a matrix of series. It receives *long-format*
columns that share one logical length:

| Column | Role | Required |
|---|---|---|
| `x` | the ordered position | yes |
| `y` | the value drawn | yes |
| `series` | grouping; one line per distinct code | no |
| `group` | extra grouping for aggregation | no |

Two series therefore look like this, with every `x` repeated once per series:

```text
x     y     series
0.0   0.0   0
0.0   1.0   1
1.0   0.4   0
1.0   0.9   1
```

The builder validates that every column has the same length before any mark is
built. That is why the example duplicates the phase column when it appends the
sine and cosine observations.

## Same chart on three representations

The recipe reads a column through `SVColumnView`, which is dense, sparse or a
bounded streaming window. It resolves a sparse column by visiting only its
stored entries — never by expanding it into a dense buffer — and it reads a
streaming column from the snapshot the source already published, so the render
thread never waits for new data.

The companion test
`a_sparse_input_is_never_densified` proves the sparse path draws three points
from a logical column of length ten. `dense_sparse_and_streaming_agree_within_the_documented_tolerance`
proves the three representations produce the same series within the documented
tolerance. A streaming chart is accepted by a backend that declares support for
streaming; an export-only backend rejects it at build time, which the test
`an_export_only_backend_rejects_a_streaming_column_at_build` asserts.

## Ordering: the line must know which point comes next

A polyline is an ordered sequence. If the observations arrive shuffled, drawing
them in arrival order produces a scribble. `SVLinePlot` therefore sorts each
series by `x` by default, with a *stable* sort: equal `x` values keep their
arrival order. Setting `sort_by_x(false)` preserves the incoming order for
callers that already know it.

Ordering is also what makes a live line cheap to grow. The transition kernel
matches points by their `x` value, so an appended point does not move the
points already drawn; the segment to it simply interpolates.

## Missing values: a gap or a bridge

A non-finite `x` or `y` is not a value on the canvas. By default such an
observation ends the current segment, and drawing resumes after it:

```text
y:  1     2    NaN    4     5
    ●────●           ●────●
```

`bridge_missing(true)` drops the missing observation and connects the
neighbours instead. The test
`missing_values_break_the_line_unless_bridging_is_requested` walks both paths
and asserts two segments versus one.

## Aggregation: many observations at one position

Seaborn's `lineplot` estimates a value per `x`; plotly and bokeh draw the
observations as given. Both behaviours are useful, so aggregation is an
explicit option, not a hidden default.

When `aggregate` is enabled, the recipe groups observations by `(series, x,
group)` and calls the existing `SVStat::Aggregate`. The estimator (`Mean` by
default, or median, sum, minimum, maximum, standard deviation, count) decides
the drawn value. An optional `error` or `ci(probability)` asks for an interval;
the recipe draws it with `SVMark::Ribbon` underneath the line.

```mermaid
flowchart TB
    O["observations at x = 1<br/>y = 1, 3, 2"] --> G["group by x"]
    G --> S["SVStat::Aggregate<br/>Mean"]
    S --> V["drawn value<br/>y = 2"]
    S --> I["interval from ci<br/>lower · upper"]
    V --> L["SVMark::Line"]
    I --> R["SVMark::Ribbon"]
    style O fill:#dbeafe,stroke:#2563eb,color:#1e3a8a
    style G fill:#fce7f3,stroke:#db2777,color:#831843
    style S fill:#fef3c7,stroke:#d97706,color:#78350f
    style V fill:#dcfce7,stroke:#16a34a,color:#14532d
    style I fill:#ede9fe,stroke:#7c3aed,color:#4c1d95
    style L fill:#ffedd5,stroke:#ea580c,color:#7c2d12
    style R fill:#e0f2fe,stroke:#0284c7,color:#0c4a6e
```

The test `aggregation_reports_the_estimator_and_draws_the_interval` feeds two
observations at `x = 1` and two at `x = 2`, asserts the drawn values are the
means `2` and `3`, and asserts both a ribbon and a line become primitives.

## Interpolation without a second line kernel

Plotly exposes `line_shape`: `linear`, `spline`, `hv`, `vh`, `hvh`, `vhv`.
`SVLinePlot` mirrors those shapes as `SVLineInterpolation`. The important design
choice is *where* the shape is applied: the recipe expands the observations into
the vertices a polyline needs, in data space, and then hands those vertices to
the existing line mark. The line mark is never reimplemented.

- `StepStart` (`hv`) holds the old value, then jumps.
- `StepEnd` (`vh`) jumps at the current `x`, then holds.
- `StepMiddle` (`hvh`) jumps at the midpoint.
- `StepCentered` (`vhv`) uses a vertical–horizontal–vertical transition around
  the midpoint.
- `Spline` samples a Catmull-Rom curve through each pair of observations.

Because markers are drawn from the original observations, not from the
interpolated vertices, a spline does not turn into a field of dots. The test
`step_interpolation_inserts_the_corner_vertices` asserts the expected five
vertices for three step observations.

## Multi-series colour, markers and the legend

When the `series` column is present, the recipe assigns each distinct code the
next entry of the scale kernel's stable categorical palette. A series keeps its
colour when a new series appears, because the palette index is assigned at first
appearance. The test
`series_divides_the_lines_and_assigns_a_stable_colour_each` asserts two series,
two different colours and a non-empty legend.

`markers(true)` adds `SVMark::Dot` on top of every line, one circle per
observation. `line_width` and `dash` flow straight into `SVMark::Line`'s paint,
which `line_width_and_dash_reach_the_stroke_paint` verifies on the produced
primitives.

## Composition, observability and rendering

`SVLinePlot::compose()` resolves the series, the scales and the decorations and
returns an `SVLinePlotComposition`. A backend then visits the mark geometries in
draw order: intervals first, lines next, markers last. This visitor shape keeps
the recipe independent of any document type while still letting the real SVG
backend serialize every geometry.

Every public operation runs inside the one operation span from the figure
substrate. `composing_records_the_backend_representation_and_point_count`
captures that span and asserts `backend.kind = svg`, `representation = Dense`
and `points = 3`. Rendering goes through `scienceview_svg::SVSvgDocument`;
`concurrent_renders_produce_byte_identical_documents` renders one chart from
eight threads and compares the bytes.

## The example and its figure

The runnable program is `examples/line_plot.rs`. It builds a damped sine and
cosine, one series each, enables markers and serializes the composition with
the real SVG document surface:

```rust,ignore
let plot = SVLinePlotBuilder::new()
    .x(SVColumnView::Dense(&x))
    .y(SVColumnView::Dense(&y))
    .series(SVColumnView::Dense(&series))
    .markers(true)
    .line_width(SVLength::absolute(2.0))
    .title("Damped oscillation by series")
    .backend(SVBackendKind::Svg)
    .build()?;

let composition = plot.compose()?;
let mut document = SVSvgDocument::new(composition.viewport())
    .with_title(composition.title().unwrap_or("A line plot"))
    .with_background(SVDecorationStyle::built_in().background());
document.push_decoration_primitives(composition.decorations());
composition.visit_mark_geometries(|geometry| document.push_mark_geometry(geometry))?;
```

![Damped oscillation line plot generated by scienceview](assets/line_plot_recipe.svg)

The figure name, `line_plot_recipe.svg`, is deliberately distinct from the
existing `line_plot.svg` produced by the `line_plot_svg` example.
