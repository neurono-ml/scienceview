# FDR-134 — `SVImagePlot`

| Field | Value |
|---|---|
| Id | FDR-134 |
| Name | `SVImagePlot` |
| Family | Matrix and field |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [ch38 — the image recipe](../../src/development-explained/ch38-image-plot.md) |
| Example program | [`examples/image_plot.rs`](../../../examples/image_plot.rs) |

## Purpose

`SVImagePlot` displays a raster: an image, or a numeric array rendered as
greyscale or through a colour map. It is the recipe for showing a
photograph, a microscopy frame, a spectrogram or a computed field.

## Sources

| Library | Function | Link |
|---|---|---|
| plotly | `go.Image`, `px.imshow` | <https://plotly.com/python/reference/#plotly.graph_objects.Image> |
| bokeh | `Image`, `ImageRGBA`, `ImageURL` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Image> |
| seaborn | — | — |

## Recipe

```
data -> [no stat] -> raster -> [SVScale::Continuous over a colour map] -> SVMark::Image
```

## Data contract

| Input | Shape | Notes |
|---|---|---|
| Raster | a dense 2D array of intensities | rendered greyscale or through a colour map |
| Colour raster | a dense 2D array of RGBA | rendered as is |
| Encoded image | encoded bytes | rendered as an image resource |

Accepted representations: dense, sparse (a sparse raster is drawn with its
missing pixels transparent), streaming (a window of frames).

### Match key

`Index` over the raster frames.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| image position | the raster extent | — |
| pixel colour | the intensity | `Continuous` over a colour map |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `raster` | `SVImageSource` | — | plotly, bokeh |
| `colour_map` | `Option<SVColourMapRef>` | `None` — greyscale | plotly `colorscale` |
| `interpolation` | `SVImageInterpolation` | `Nearest` | bokeh `interpolate` |
| `z_min`, `z_max` | `Option<f64>` | `None` | plotly `zmin`/`zmax` |
| `opacity` | `f32` | `1.0` | all three |
| `origin` | `SVImageOrigin` | `TopLeft` | plotly `yaxis.autorange` |
| `colour_bar` | `bool` | `false` | plotly |

## Theme

Consumes the greyscale token and, when a colour map is used, the continuous
colour-map token.

## Temporal

| Property | Behaviour |
|---|---|
| frame change | the raster is replaced; a cross-fade is the default transition for images |
| colour map change | pixels re-colour, interpolating |
| zoom | the raster is resampled |

Image transitions are the one case where a pixel-level cross-fade is the
natural behaviour, since there is no meaningful correspondence between the
pixels of two different images.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | an embedded `<image>` with a data URI, or a `<rect>` grid for a numeric raster |
| `Html` | same |
| `Native` / `Wasm` | the raster is sampled into the frame buffer |
| `Tui` | the raster is rendered with the terminal's half-block characters |
| `React` / `Vue` / `Angular` | an `<img>` element or a canvas |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The raster is displayed with the requested extent and origin.
      Evidence: `image_plot::image_plot_tests` (`the_default_scene_draws_one_image_mark_on_the_raster_grid`,
      `the_origin_decides_which_edge_the_first_row_touches`,
      `equal_aspect_letterboxes_and_stretch_fills_the_extent`).
- [x] A numeric raster is coloured through the requested colour map, or
      greyscale when none is given.
      Evidence: `image_plot::image_plot_tests` (`greyscale_is_the_default_colour_map`,
      `a_colour_map_bypasses_greyscale`,
      `explicit_intensity_bounds_override_the_derived_domain`).
- [x] The interpolation mode is honoured.
      Evidence: `image_plot::render_tests` (`bilinear_sampling_reaches_the_svg_document`).
- [x] A streaming source replaces the frame in place.
      Evidence: `image_plot::parity_tests`
      (`a_streaming_window_renders_its_newest_frame`,
      `a_short_streaming_window_fills_the_frame_it_has`,
      `an_empty_streaming_window_composes_a_transparent_frame`).

## Open questions

- Should encoded images (PNG, JPEG) be decoded by the core, or only numeric
      rasters, leaving encoded formats to the backend? *Current answer: the
      core handles numeric rasters only; encoded images are a backend
      concern, so the core stays dependency-light.*
