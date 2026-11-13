# scienceview for Python

The Python binding of `scienceview` — the same figure builders, the same explicit
backend selection and the same dense, sparse and streaming data sources as the
Rust surface.

## Two layers, one contract

- `src/spec/` is a pure-Rust conversion layer. It owns Python's lists, describes a
  figure as a value and performs the real builder calls and operations. It
  compiles with **no** `pyo3` and no Python development headers, and it carries
  the semantics the unit tests prove.
- `src/extension/` is the optional CPython extension. It only translates keyword
  arguments into that specification and maps `SVError` variants onto typed
  exceptions: `SVBindingError` and its subclasses `SVBackendSelectionError`,
  `SVConfigurationError`, `SVDataError`, `SVBackendError` and `SVScriptError`.

`pyo3` is behind the `python` Cargo feature and is never part of the default
build:

```sh
cargo build -p scienceview_python              # no pyo3, pure layer only
cargo build -p scienceview_python --features python
cargo test  -p scienceview_python              # pure layer, no interpreter
```

## Requirements for the extension

- Rust 1.85 or newer.
- A CPython development environment (`python3-dev` on Debian/Ubuntu, `python3`
  from Homebrew on macOS). PyO3 locates it with `python3`; set
  `PYO3_PYTHON=/path/to/python3` to choose another interpreter.
- [maturin](https://www.maturin.rs/) to build the wheel:

```sh
pip install maturin
cd crates/scienceview_python
maturin build --release            # writes a wheel into target/wheels
python -m pip install --force-reinstall target/wheels/scienceview-*.whl
```

`maturin develop` installs the extension into the active virtual environment for
iterative work. Maturin enables PyO3's `extension-module` behaviour for the
wheel build itself; `pyproject.toml` only selects the `python` feature. The
wheel is built for the interpreter that runs the build (PyO3 is used without
the stable ABI), so build once per CPython version you publish for; the
`requires-python` floor says which versions can build it, not that one wheel
installs on all of them.

## Usage

```python
import scienceview as sv

figure = sv.scatter_plot(
    {"x": [0, 1, 2, 3], "y": [1, 4, 2, 5], "series": [0, 0, 1, 1]},
    backend="svg",              # required: there is no automatic backend
    x="x",
    y="y",
    series="series",
    title="Bound scatter",
)
open("scatter.svg", "wb").write(figure.render())
figure.save("scatter.svg")

# Streaming: push samples and re-render the bounded window in place.
live = sv.StreamingColumn(capacity=128)
live.push([1.0, 2.0, 3.0])
line = sv.scatter_plot(live, backend="svg", x="index", y="value")
line.render()
live.push([4.0, 5.0])
line.update(appended_points=2)

# Sparse input keeps its indices; it is never densified.
sparse = sv.SparseColumn(indices=[0, 2], values=[1.0, 3.0], length=4)

# The option catalogue is introspectable, so the surface documents itself.
print(sv.options_for("scatter"))
print(sv.backend_kinds())       # no "automatic" among them

# Controls dispatch the same semantic vocabulary as Rust widgets.
state = sv.ControlState()
state.register_axis("horizontal", 0.0, 10.0)
state.dispatch("zoom", axis="horizontal", factor=0.8, anchor=0.5)
```

## Bound surface

`figure`, `scatter_plot`, `line_plot`, `area_plot`, `histogram_plot` and
`dashboard` are the constructors. Each figure is lazy: it renders on `render()`,
`render_svg()` or `save()`, and a streaming figure re-renders on `update()`.
`backend_kinds()`, `plot_kinds()` and `options_for(kind)` describe the catalogue.

The recipe kinds are bound one adapter at a time; a kind without an adapter is
reported by `plot_kinds()` rather than silently approximated.

## Tests

The pure layer is always tested. The interpreter tests are gated twice:

```sh
SCIENCEVIEW_PYTHON_TESTS=1 cargo test -p scienceview_python --features python-tests
```

Set the variable only where a Python development environment exists; without it
the interpreter tests return immediately, so CI without Python stays green.
