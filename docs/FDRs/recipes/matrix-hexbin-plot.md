# FDR-135 — `SVHexbinPlot`

| Field | Value |
|---|---|
| Id | FDR-135 |
| Name | `SVHexbinPlot` |
| Family | Matrix and field |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVHexbinPlot` bins two continuous variables into hexagonal cells and draws
each occupied cell, coloured by the number of observations. It is a scatter
plot for very large samples: a million points become a few thousand hexagons
that the eye can read.

## Sources

| Library | Function | Link |
|---|---|---|
| bokeh | `HexTile` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.HexTile> |
| plotly | `go.Histogram2d` (as a square-grid equivalent) | <https://plotly.com/python/reference/#plotly.graph_objects.Histogram2d> |
| seaborn | — | — |

## Recipe

```
data -> [SVStat::Bin (2D, hexagonal lattice)] -> (cells, counts)
     -> [SVScale::Continuous over a colour map] -> SVMark::HexTile
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | first variable | continuous | yes |
| `y` | second variable | continuous | yes |
| `series` | one lattice each | nominal | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Derived` — the hexagonal cell index.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` |
| y | `y` | `Continuous` |
| fill colour | the count | `Continuous` over a colour map |
| cell size | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `SVColumnRef` | — | bokeh |
| `series` | `Option<SVColumnRef>` | `None` | — |
| `size` | `SVLength` | derived from the width | bokeh `size` |
| `orientation` | `f64` radians | `0` | bokeh `angle` |
| `colour_map` | `SVColourMapRef` | theme default | — |
| `normalise` | `SVNormalisation` | `Count` | — |
| `min_count` | `usize` | `1` | — |

## Theme

Consumes the continuous colour-map token, axis tokens, text tokens.

## Temporal

| Property | Behaviour |
|---|---|
| cell colour | interpolates |
| cell entering or leaving | enters or exits |
| lattice growing | new cells enter |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per hexagon |
| `Html` | same |
| `Native` / `Wasm` | polygon rasterization |
| `Tui` | one cell per hexagon, approximated on the cell grid |
| `React` / `Vue` / `Angular` | one `<path>` per hexagon |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The hexagonal lattice covers the data domain with no gaps.
- [ ] Each hexagon's colour is the colour map's colour for its count.
- [ ] Cells below `min_count` are not drawn.
- [ ] Dense, sparse and streaming inputs give the same lattice for the same
      data.

## Open questions

- Should the hexagonal lattice be an option of the 2D bin stat, rather than a
      separate stat? *Current answer: yes — the lattice geometry is an option
      of `SVStat::Bin`, and this recipe configures it. Open whether the
      recipe earns its own type at all.*
