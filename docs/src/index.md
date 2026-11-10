# scienceview

Scientific visualization for Rust, in the same family as `sciencekit`.

`scienceview` covers the capabilities of the most popular Python plotting
libraries — the same styling and interactivity — plus streaming data updates.
Every figure is built through a builder, and every builder selects its
rendering backend explicitly (`SVBackendKind`): native OS embedded components,
HTML with JavaScript, WebAssembly, or terminal TUI.

```rust,ignore
let figure = SVFigure::builder()
    .backend(SVBackendKind::Automatic)
    .execution_mode(SVExecutionMode::Automatic)
    .build();
figure.render();
```

This book hosts the API reference, usage examples, and a description of every
function. The companion *Development Explained* part tells the story behind
each change, in the order the library was built.
