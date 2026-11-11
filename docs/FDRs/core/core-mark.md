# FDR-001 — `SVMark`

| Field | Value |
|---|---|
| Id | FDR-001 |
| Name | `SVMark` |
| Family | Core — rendering |
| Status | planned |
| Layer | Core |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVMark` is the geometric primitive that turns data values into something a
backend can draw. It is the smallest unit of the layer-1 substrate and the
single place where the question "what does a mark *look* like" is answered.

A mark never owns data. It receives already-computed positions and visual
channel values and describes how to realize them. Keeping marks free of data
is what allows the same mark to be reused by every recipe, and what allows a
streaming update to redraw a mark without re-running the whole recipe.

## Sources

The mark vocabulary merges the three reference libraries, keeping the union of
their visual options under one name per geometry:

| `SVMark` | seaborn objects | bokeh glyph | plotly |
|---|---|---|---|
| `Dot` | `Dot`, `Dots` | `Circle`, `Scatter` (marker) | `Scatter` (markers) |
| `Line` | `Line`, `Lines`, `Path`, `Paths` | `Line`, `MultiLine` | `Scatter` (lines) |
| `Area` | `Area` | `VArea`, `HArea` | `Scatter` (fill) |
| `Bar` | `Bar`, `Bars` | `VBar`, `HBar`, `Rect` | `Bar` |
| `Ribbon` | `Band` | `Patches`, `MultiPolygons` | `Scatter` (fill between) |
| `Text` | `Text` | `Text`, `TeXGlyph`, `MathTextGlyph` | `Scatter` (text) |
| `Image` | — | `Image`, `ImageRGBA`, `ImageURL` | `Image`, `Heatmap` |
| `Wedge` | — | `Wedge`, `Annulus`, `AnnularWedge`, `Arc` | `Pie` |

- seaborn — <https://seaborn.pydata.org/api.html#seaborn.objects.Dot>
- bokeh — <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html>
- plotly — <https://plotly.com/python/reference/#plotly.graph_objects.Scatter>

## Recipe

`SVMark` is the leaf of every recipe:

```
mark -> (stat) -> (move) -> (scale) -> draw
```

The mark consumes the final, display-space values. It performs no statistics
and no aggregation.

## Data contract

A mark consumes a *view* of display-space values, never a data source:

| Channel | Type | Cardinality |
|---|---|---|
| position | `SVPoint` (f64 pair) | one per mark instance |
| size | `f64` | per instance, or one default for the whole mark |
| colour | `SVColour` (token or literal) | per instance, or one default |
| opacity | `f32` in `[0, 1]` | per instance, or one default |
| shape | mark-specific variant | per instance, or one default |

Per-instance values arrive as borrowed slices. A mark never allocates a copy
of the data it draws.

## Visual channels and their options

### `Dot`

| Option | Type | Default | Notes |
|---|---|---|---|
| `shape` | `SVDotShape` | `Circle` | `Circle`, `Square`, `Triangle`, `Diamond`, `Cross`, `Hexagon` |
| `size` | `SVLength` | token `mark.size` | diameter or area, per the `size_mode` option |
| `size_mode` | `SVDotSizeMode` | `Diameter` | `Diameter`, `Area` |
| `stroke_width` | `SVLength` | `0` | a stroked dot is seaborn's `Dots` |
| `fill` | `SVFillMode` | `Filled` | `Filled`, `Stroked`, `Both` |

`shape` covers the union of bokeh's marker list and plotly's
`marker.symbol`; the terminal backend maps every shape to a cell
approximation.

### `Line`

| Option | Type | Default | Notes |
|---|---|---|---|
| `width` | `SVLength` | token `mark.line_width` | stroke width |
| `dash` | `SVDashPattern` | `Solid` | `Solid`, `Dashed`, `Dotted`, `DashDot`, custom |
| `join` | `SVLineJoin` | `Round` | `Miter`, `Round`, `Bevel` |
| `cap` | `SVLineCap` | `Butt` | `Butt`, `Round`, `Square` |
| `simplify` | `f32` | `0` | tolerance for polyline simplification, in display units |

### `Area`

An area is a line closed against a baseline.

| Option | Type | Default | Notes |
|---|---|---|---|
| `baseline` | `f64` or `SVBaselineMode` | `0` | `Zero`, `Min`, `Max`, explicit value |
| `opacity` | `f32` | token `mark.area_opacity` | fills are usually translucent |
| `stack` | handled by `SVMove::Stack` | — | stacking is a move, not a mark property |

### `Bar`

| Option | Type | Default | Notes |
|---|---|---|---|
| `orientation` | `SVBarOrientation` | `Vertical` | `Vertical`, `Horizontal` |
| `width` | `f64` | `0.8` | bar width as a fraction of the category slot |
| `corner_radius` | `SVLength` | `0` | rounded bar ends |
| `baseline` | `f64` | `0` | baseline the bar grows from |
| `group_gap` | `f32` | `0` | gap between grouped bars, as a fraction |

### `Ribbon`

A ribbon is a filled region between two boundary lines.

| Option | Type | Default | Notes |
|---|---|---|---|
| `opacity` | `f32` | token `mark.area_opacity` | |
| `boundary` | `SVRibbonBoundary` | `Both` | `Upper`, `Lower`, `Both` |

### `Text`

| Option | Type | Default | Notes |
|---|---|---|---|
| `font` | `SVFontRef` | token `text.font` | resolved from the theme |
| `size` | `SVLength` | token `text.size` | |
| `align` | `SVTextAlign` | `Center` | `Left`, `Center`, `Right` |
| `baseline` | `SVTextBaseline` | `Middle` | |
| `format` | `SVFormatSpec` | theme default | how a value becomes a string |

### `Image`

| Option | Type | Default | Notes |
|---|---|---|---|
| `interpolation` | `SVImageInterpolation` | `Nearest` | `Nearest`, `Bilinear` |
| `opacity` | `f32` | `1.0` | |
| `colour_map` | `SVColourMapRef` | none | present for heatmap-like use |

### `Wedge`

| Option | Type | Default | Notes |
|---|---|---|---|
| `start_angle` | `f64` radians | `0` | |
| `end_angle` | `f64` radians | `TAU/2` | |
| `inner_radius` | `f64` | `0` | `0` is a pie sector, `>0` an annulus |

## Configuration

Every option above is exposed on the owning builder, never as a free function.
The builder pattern is mandatory; direct constructors are private.

```rust
// shape is illustrative of the option surface, not of final naming
SVScatterPlot::new()
    .backend(SVBackendKind::Svg)
    .data_source(source)
    .x("weight")
    .y("height")
    .mark(SVMark::dot().shape(SVDotShape::Triangle))
    .build()
```

## Theme

Marks consume tokens, never literal colours:

| Token | Used by |
|---|---|
| `mark.size`, `mark.line_width`, `mark.area_opacity` | all marks |
| `mark.palette` | the colour assigned to a series |
| `text.font`, `text.size`, `text.colour` | `Text` |

A theme may change every one of these without touching the mark code. The
terminal backend consumes the same tokens, so a figure looks the same in a
terminal and in a browser.

## Temporal

Marks are the unit of transition. For each mark instance the transition layer
needs a stable **match key** so that a change of state can be animated rather
than redrawn. See [core-transition](core-transition.md) and
[core-animation](core-animation.md).

| Property | Interpolable | Note |
|---|---|---|
| position | yes | linear interpolation in display space |
| size | yes | |
| colour | yes | interpolated in the theme's colour space |
| opacity | yes | |
| shape | no | a shape change is a discrete swap |
| dash pattern | no | discrete |
| font, size of text | size yes, family no | |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | a mark becomes one or more SVG elements; `Dot` may be a `<circle>`, `<rect>` or `<path>` |
| `Html` | same elements, embedded in a self-contained document |
| `Native` | rasterized by the CPU renderer onto the resolved layout tree |
| `Wasm` | same CPU renderer, on a browser canvas |
| `Tui` | marks are rasterized onto the cell grid; `Dot` uses the cell's glyph set, `Bar` uses block characters |
| `React` / `Vue` / `Angular` | component output with the same element structure as `Svg` |
| `Tauri` / `Dioxus` | Rust-native component targets |

A backend that cannot express a mark variant must declare that limitation in
its capability matrix and fail at build time, never degrade silently.

## Acceptance

- [ ] The mark set covers `Dot`, `Line`, `Area`, `Bar`, `Ribbon`, `Text`,
      `Image`, `Wedge`, each with the option surface above.
- [ ] Every mark renders on every backend that declares support for it, and
      the SVG, HTML, native and terminal renderings show the same data.
- [ ] Marks accept borrowed display-space values without copying them.
- [ ] Marks consume theme tokens; no literal colour or size is hardcoded.
- [ ] Mark geometry is resolved in the backend-independent coordinate system
      and only the final rasterization is snapped to the backend grid.
- [ ] Interpolable properties are enumerated exactly as above and shared with
      the transition layer.

## Open questions

- Should `Dot` with `FillMode::Stroked` be one variant of `Dot`, or a
  distinct `SVMark::Stroke`? *Current answer: one variant — the difference is
  a paint attribute, not a geometry.*
- Should `Image` own the colour map, or should the colour map belong to the
  recipe that produces the image data? *Current answer: the recipe owns the
  mapping, the mark owns the sampling — this keeps `Image` reusable for
  non-heatmapped rasters.*
- How are `Dash` patterns rendered in the terminal backend, where the cell
  grid has no sub-cell dashes? *Open.*
