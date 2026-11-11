# Deferred chart families

This file catalogues the chart types of the three reference libraries that are
**not** covered by a full FDR, with their sources, their visual encoding and
the reason for the deferral.

Nothing here is forgotten: every chart type of
[seaborn](https://seaborn.pydata.org/examples/index.html),
[bokeh](https://docs.bokeh.org/en/latest/docs/gallery.html) and
[plotly](https://plotly.com/examples/) appears either in
[`recipes/`](recipes/) or in this file. The catalogue is complete; the
*specification* is scoped to the scientific core.

## Reference links

| Library | Gallery | API reference |
|---|---|---|
| seaborn | <https://seaborn.pydata.org/examples/index.html> | <https://seaborn.pydata.org/api.html> |
| bokeh | <https://docs.bokeh.org/en/latest/docs/gallery.html> | <https://docs.bokeh.org/en/latest/docs/reference.html> |
| plotly | <https://plotly.com/examples/> | <https://plotly.com/python/reference/> |

## Hierarchical partitions

| Type | Sources | Encoding | Why deferred |
|---|---|---|---|
| Treemap | plotly `Treemap` | nested rectangles, area proportional to value | Requires a partition layout (squarified, slice-and-dice) that is a layout concern, not a mark concern; interacts with `layout-engine` in a way the core does not model yet |
| Sunburst | plotly `Sunburst` | nested radial sectors, angle proportional to value | Same partition layout, plus a radial layout mode that does not exist yet |
| Icicle | plotly `Icicle` | nested vertical bands | Same partition layout |

## Flow and network

| Type | Sources | Encoding | Why deferred |
|---|---|---|---|
| Sankey | plotly `Sankey` | ribbons whose width is a flow magnitude | Link routing is a layout problem of its own, with no precedent in the core |
| Chord | plotly `Chord` | circular ribbon matrix | Combines a circular layout with ribbon routing |
| Network / graph | bokeh `Graph` models, `MultiLine` + markers | nodes and edges | Needs a graph layout (force, hierarchical) and a node/edge model distinct from `SVMark` |
| Sankey-like flow | bokeh `MultiLine` with tapering | — | Same as Sankey |

## Polar, ternary and geographic

| Type | Sources | Encoding | Why deferred |
|---|---|---|---|
| Polar scatter | plotly `Scatterpolar`, bokeh polar | position on a circle | Requires a polar coordinate space in the layout engine, and a different scale family than `SVScale` |
| Polar line / bar | plotly `Scatterpolar`, `Barpolar`, bokeh polar | — | Same |
| Radar | plotly radar (polar, closed) | — | Same as polar; a closed radar polygon is a polar line with a closing segment |
| Ternary | plotly `Scatterternary` | position inside a triangular simplex | A third coordinate space with its own barycentric scale |
| Choropleth | plotly `Choropleth`, `Choroplethmap` | filled country/region polygons | Needs a map projection library and a geographic data source |
| Scatter-geo / map tiles | plotly `Scattergeo`, `Densitymap`, bokeh `map_plots` | — | Same; also implies a tile server dependency the dependency-light core forbids |

## Three-dimensional

| Type | Sources | Encoding | Why deferred |
|---|---|---|---|
| 3D scatter | plotly `Scatter3D` | position in a projected 3D space | Needs a 3D camera model; `event-payloads` reserves camera state but no layout exists |
| Surface | plotly `Surface` | a triangulated surface over two dimensions | Needs mesh generation and a 3D rasterizer |
| Mesh 3D | plotly `Mesh3D` | explicit vertices and faces | Same |
| Cone | plotly `Cone` | 3D arrows | Same |
| Streamtube | plotly `Streamtube` | tube along a vector field | Same |
| Volume / isosurface | plotly `Volume`, `Isosurface` | voxel field rendered as a surface | Same |

## Specialized and dashboard-oriented

| Type | Sources | Encoding | Why deferred |
|---|---|---|---|
| Candlestick / OHLC | plotly `Candlestick`, `Ohlc` | box plus whiskers over time | Domain-specific encoding over a time axis; belongs with a financial use case |
| Waterfall | plotly `Waterfall` | floating bars showing a running total | Requires a cumulative baseline computation that no current stat kernel provides |
| Funnel | plotly `Funnel`, `Funnelarea` | tapered stages | Same as waterfall; a staged aggregate transform |
| Gauge / indicator | plotly `Indicator`, `Gauge` | a single value on an arc or number | Belongs to the UI kit rather than to the data artists |
| Table | plotly `Table` | a grid of cells | A grid widget, not an artist |
| Quiver | plotly `Quiver` | arrows on a 2D grid | Needs a vector-field mark and a head glyph |
| Carpet | plotly `Carpet` | a 2D region drawn as a boundary | Not a data encoding on its own |
| Pie | plotly `Pie` | sector angle proportional to value | A pie is a stacked bar on a polar axis; it depends on both the polar coordinate space and the stack move |
| Clustermap | seaborn `clustermap` | clustered heatmap with dendrograms | Deferred *as a whole*; the heatmap itself is specified in [matrix-heatmap-plot](recipes/matrix-heatmap-plot.md) and the dendrogram requires a clustering algorithm that is not part of the core |
| Facet grids | seaborn `FacetGrid`, `catplot`, `relplot` | a grid of small multiples | The grid composition belongs to `dashboard`, not to an artist |
| Pair grids | seaborn `PairGrid` | a grid of pairwise plots | The grid composition belongs to `dashboard`; the individual cells are specified in [multivariate-pair-plot](recipes/multivariate-pair-plot.md) |
| Joint grids | seaborn `JointGrid` | bivariate plot with marginal plots | The grid composition belongs to `dashboard`; the bivariate and marginal parts are specified in [multivariate-joint-plot](recipes/multivariate-joint-plot.md) |

## Multi-plot grids, in one place

`seaborn`'s `FacetGrid`, `PairGrid` and `JointGrid`, and `plotly`'s
`make_subplots`, all compose *existing* figures into a grid. That composition
is the `dashboard` capability of the foundation, which lives in its own crate
and is not an artist. The scientific core therefore specifies the cells, not
the grid.

## Promotion criteria

A deferred type becomes a full FDR when all of the following hold:

1. A concrete use case is named, with the data shape it needs.
2. The type can be expressed as a composition of the existing core kernels, or
   the new kernel it needs is justified on its own terms and specified in
   `core/`.
3. Dense, sparse and streaming parity is achievable, or the static-only
   exception states why and names the alternative path.
4. The acceptance criteria for the family are written before implementation
   begins.
