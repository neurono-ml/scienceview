# scienceview

[![CI](https://github.com/andreclaudino/scienceview/actions/workflows/ci.yml/badge.svg)](https://github.com/andreclaudino/scienceview/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/scienceview.svg)](https://crates.io/crates/scienceview)
[![docs.rs](https://img.shields.io/docsrs/scienceview)](https://docs.rs/scienceview)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/scienceview.svg)](LICENSE-MIT)

Scientific visualization for Rust: builder-style figures with styling, interactivity and streaming updates across native, HTML, WebAssembly and terminal targets.

`scienceview` covers the capabilities of the most popular Python plotting libraries — the same styling and interactivity — plus streaming data updates.

## Quick start

```toml
[dependencies]
scienceview = "0.1"
```

```rust
let figure = SVFigure::builder()
    .backend(SVBackendKind::Automatic)
    .execution_mode(SVExecutionMode::Automatic)
    .build();
figure.render();
```

Every figure is built through a builder, and every builder selects its rendering backend explicitly (`SVBackendKind`): native OS embedded components, HTML with JavaScript, WebAssembly, or terminal TUI.

## Features

Borrowed slices are the core data model — no heavy algebra or GUI toolkit is required to describe a figure. Everything else is an opt-in Cargo feature:

- `data-ndarray`: zero-copy views over `ndarray` arrays.
- `spec-serde`: JSON serialization of the figure specification.
- `render-svg` (default): self-contained SVG output, no extra dependencies.
- `render-html`: self-contained HTML page embedding the SVG.
- `render-tui`: terminal rendering via `ratatui`.
- `render-native`: from-scratch interactive renderer (CPU pixels via `softbuffer` in a `winit` window).
- `streaming`: live updates over standard-library channels.
- `streaming-tokio`: live updates over Tokio watch channels.
- `wasm`: WebAssembly canvas target.

Requires Rust 1.85+, edition 2024.

## Documentation

- Book (API, usage examples, every function): <https://andreclaudino.github.io/scienceview>
- API reference: <https://docs.rs/scienceview>
- Product source of truth and contributor workflow: see [`AGENTS.md`](AGENTS.md).

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
