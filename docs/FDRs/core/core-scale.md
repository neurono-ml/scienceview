# FDR-004 — `SVScale`

| Field | Value |
|---|---|
| Id | FDR-004 |
| Name | `SVScale` |
| Family | Core — rendering |
| Status | planned |
| Layer | Core |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVScale` maps data values onto visual channels: a position on an axis, a
colour in a palette, a size, an opacity. It is the only place where a data
value becomes a visual property, and therefore the only place where a scale
change has to be animated.

A scale also owns the *domain* of an axis — the range of data the axis
covers — and the *range* of the axis — the space it covers on screen. Keeping
both in one object is what makes an animated axis possible: when the domain
changes, the range changes with it, and the marks are re-projected.

## Sources

seaborn's `Scale` objects are the model.

| `SVScale` | seaborn objects | plotly | bokeh |
|---|---|---|---|
| `Continuous` | `Continuous` | axis + `colorscale` | `LinearScale`, `LogScale` |
| `Nominal` | `Nominal` | categorical axis | `FactorRange` |
| `Temporal` | `Temporal` | `date` axis | `DatetimeTickFormatter` |
| `Boolean` | `Boolean` | — | — |

- seaborn — <https://seaborn.pydata.org/api.html#seaborn.objects.Continuous>
- plotly — <https://plotly.com/python/reference/#plotly.graph_objects.Scatter>
- bokeh — <https://docs.bokeh.org/en/latest/docs/reference/models/scales.html>

## Recipe

```
data -> [SVStat] -> values -> [SVMove] -> [SVScale] -> SVMark
```

## Data contract

A scale consumes values and produces display-space quantities.

| Scale | Input | Output |
|---|---|---|
| `Continuous` | numeric values | position on a linear or logarithmic axis |
| `Nominal` | category labels | position of a category slot |
| `Temporal` | timestamps | position on a time axis |
| `Boolean` | `true`/`false` | position of one of two slots |

### Axis model

An axis is a scale plus the decorations that describe it:

| Property | Type | Default |
|---|---|---|
| `domain` | `(f64, f64)` | derived from the data |
| `range` | `(f64, f64)` in display space | derived from the layout |
| `transform` | `SVScaleTransform` | `Linear` (`Linear`, `Log`, `SymLog`, `Sqrt`) |
| `inverse` | `Option<SVLength>` | `None` — a secondary axis |
| `tick_count` | `Option<usize>` | `None` — derived from the available space |
| `tick_format` | `SVFormatSpec` | theme default |
| `grid` | `SVGridStyle` | from the theme |

The `tick_count` derivation is the layout engine's rule: an axis in a narrow
space gets fewer ticks and stays readable. It is not a per-chart decision.

## Visual channels

| Channel | Scale | Notes |
|---|---|---|
| x position | one of the scales | |
| y position | one of the scales | |
| colour (categorical) | `Nominal` over the series palette | a nominal colour scale assigns palette entries |
| colour (continuous) | `Continuous` over a colour map | |
| size | `Continuous` | |
| opacity | `Continuous` | |

A colour scale that maps values to colours is a *value* scale over a
`SVColourMap`. A colour scale that maps series to palette entries is a
*nominal* scale. The two are distinct because the second is a discrete
assignment and the first is a continuous interpolation — and animating them
requires different rules.

## Configuration

| Scale | Option | Type | Default |
|---|---|---|---|
| `Continuous` | `transform` | `SVScaleTransform` | `Linear` |
| `Continuous` | `domain` | `Option<(f64, f64)>` | `None` — derived |
| `Continuous` | `nice` | `bool` | `true` |
| `Nominal` | `order` | `SVCategoryOrder` | `DataOrder` |
| `Nominal` | `palette` | `SVPaletteRef` | theme `mark.palette` |
| `Temporal` | `format` | `SVFormatSpec` | theme default |
| `Temporal` | `unit` | `SVTemporalUnit` | `Auto` (`Auto`, `Millisecond`, `Second`, `Minute`, `Hour`, `Day`, `Week`, `Month`, `Year`) |
| `Boolean` | `labels` | `(String, String)` | `("false", "true")` |

## Theme

Scales consume the palette and colour-map tokens. A continuous colour scale
is named by a theme token, so that a theme change restyles every continuous
encoding at once — including the colour bar of a heatmap.

## Temporal

| Property | Behaviour |
|---|---|
| domain | interpolates numerically; the axis re-ticks at each frame |
| position of a value | interpolates numerically |
| nominal colour assignment | **does not interpolate** — a series keeps its palette entry, so colours are stable across an animation |
| continuous colour | interpolates in the colour map's space |
| tick labels | recomputed per frame; a label that changes is a discrete swap, not a tween |

Stable nominal colours across an animation are not a detail — they are what
makes an animated chart readable. A series that changes colour at every frame
cannot be followed by the eye.

## Backend notes

Scales are backend-independent; the resulting display-space coordinates are
rasterized by the backend. The terminal backend snaps positions to the cell
grid, and the layout engine is responsible for keeping the decorations
consistent with the snapped marks.

## Acceptance

- [ ] The four scale kinds cover continuous, nominal, temporal and boolean
      data.
- [ ] A scale dispatches on `SVColumnView` once per operation and never
      densifies or blocks.
- [ ] The axis domain is derived from the data when not given, and honours an
      explicit domain.
- [ ] Tick count follows the space available, per `layout-engine`.
- [ ] Nominal colours are stable across an animation.
- [ ] Continuous colour scales resolve through a theme token, so a theme
      change restyles them.

## Open questions

- Should a `Continuous` scale support a `Symmetric` domain option for
  diverging colour maps, or should that live in the colour map? *Current
  answer: in the colour map.*
- Should axis domains support a "sticky" range that only expands, never
  shrinks? *Open — it matters for streaming charts, where a shrinking domain
  makes the axis jump.*
